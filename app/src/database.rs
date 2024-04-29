use chrono::{DateTime, Utc};
use sqlx::postgres::PgRow;
use sqlx::Row;
use sqlx::{postgres::PgPoolOptions, PgPool};
use crate::models::{ExtratoResponse, SaldoResponse, TransacaoRequest, TransacaoResponse, UltimasTransacoesResponse};
use crate::tratamento_erros::Error;

#[derive(Clone)]
pub struct DatabaseManager {
    pub connection: PgPool,
}

impl DatabaseManager {
    pub async fn connect_with_database() -> Self {
        let db_pool = match PgPoolOptions::new()
            .max_connections(15)
            .connect("postgresql://root:root@postgres:5432/root?sslmode=disable")
            .await
        {
            Ok(pool) => pool,
            Err(e) => panic!("Couldn't establish DB connection: {}", e),
        };

        DatabaseManager {
            connection: db_pool,
        }
    }

    pub async fn execute_procedure(
        &self,
        id: i32,
        request: TransacaoRequest,
    ) -> Result<TransacaoResponse, Error> {
        match sqlx::query(
            "SELECT saldoretorno, limiteretorno FROM efetuar_transacao($1, $2, $3, $4)",
        )
        .bind(id)
        .bind(request.tipo.to_string())
        .bind(request.valor)
        .bind(request.descricao)
        .map(|row: PgRow| TransacaoResponse {
            saldo: row.get("saldoretorno"),
            limite: row.get("limiteretorno"),
        })
        .fetch_one(&self.connection)
        .await
        {
            Ok(transacao) => Ok(transacao),
            Err(_) => Err(Error::LimiteIndisponivel),
        }
    }

    pub async fn consulta_extrato(&self, id: i32) -> Result<ExtratoResponse, Error> {
        let rows = match sqlx::query(
            "SELECT
                c.saldo AS saldo_cliente,
                c.limite AS limite_cliente,
                t.valor,
                t.tipo,
                t.descricao,
                TO_CHAR(t.realizada_em, 'YYYY-MM-DD\"T\"HH24:MI:SS.US') AS realizada_em
            FROM
                clientes c
            LEFT JOIN
                (
                    SELECT
                        *
                    FROM
                        transacoes
                    WHERE
                        cliente_id = $1
                    ORDER BY
                        realizada_em DESC
                    LIMIT
                        10
                ) t ON c.id = t.cliente_id
            WHERE c.id = $1;",
        )
        .bind(id)
        .fetch_all(&self.connection)
        .await
        {
            Ok(rows) => rows,
            Err(_) => return Err(Error::DatabaseQueryError),
        };

        //se não retornar nada = cliente não existe
        if rows.is_empty() {
            return Err(Error::ClientNotFound);
        }

        let current_time: DateTime<Utc> = Utc::now();
        let formatted_time = current_time.format("%Y-%m-%dT%H:%M:%S%.fZ");
        let saldo_response = SaldoResponse {
            total: rows[0].get("saldo_cliente"),
            data_extrato: formatted_time.to_string(),
            limite: rows[0].get("limite_cliente"),
        };

        let mut ultimas_transacoes_response = Vec::new();
        for row in rows.iter() {
            //esse if valida se cliente tem transações
            if let Some(_valor) = row.get::<Option<i32>, _>("valor") {
                let transacao = UltimasTransacoesResponse {
                    valor: row.get("valor"),
                    tipo: row.get("tipo"),
                    descricao: row.get("descricao"),
                    realizada_em: row.get("realizada_em"),
                };
                ultimas_transacoes_response.push(transacao)
            }
        }

        Ok(ExtratoResponse {
            saldo: saldo_response,
            ultimas_transacoes: ultimas_transacoes_response,
        })
    }

    /* 
    pub async fn get_cliente(&self, id: i32) -> Result<ClienteEntity, Error> {
        match sqlx::query("SELECT limite, saldo FROM clientes WHERE id = $1")
            .bind(id)
            .map(|row: PgRow| ClienteEntity {
                limite: row.get("limite"),
                saldo: row.get("saldo"),
            })
            .fetch_one(&self.connection)
            .await
        {
            Ok(cliente) => Ok(cliente),
            Err(_) => Err(Error::DatabaseQueryError),
        }
    }

    pub async fn efetuar_transacao(
        &self,
        id: i32,
        novo_saldo: i32,
        transacao_entity: TransacaoEntity,
    ) -> Result<(), Error> {
        let mut transaction = match self.connection.begin().await {
            Ok(transaction) => transaction,
            Err(_) => return Err(Error::DatabaseQueryError),
        };

        match sqlx::query("UPDATE clientes SET saldo = $2 WHERE id = $1")
            .bind(id)
            .bind(novo_saldo)
            .execute(&mut transaction)
            .await
        {
            Ok(_) => (),
            Err(_) => {
                match transaction.rollback().await {
                    Ok(_) => (),
                    Err(_) => return Err(Error::DatabaseQueryError),
                }
                return Err(Error::DatabaseQueryError);
            }
        };

        match sqlx::query(
            "INSERT INTO transacoes (cliente_id, valor, tipo, descricao)
            VALUES ($1, $2, $3, $4)",
        )
        .bind(transacao_entity.cliente_id)
        .bind(transacao_entity.valor)
        .bind(transacao_entity.tipo.to_string())
        .bind(transacao_entity.descricao)
        .execute(&mut transaction)
        .await
        {
            Ok(_) => (),
            Err(_) => {
                match transaction.rollback().await {
                    Ok(_) => (),
                    Err(_) => return Err(Error::DatabaseQueryError),
                }
                return Err(Error::DatabaseQueryError);
            }
        };

        match transaction.commit().await {
            Ok(_) => (),
            Err(_) => return Err(Error::DatabaseQueryError),
        }

        Ok(())
    }
    */
}
