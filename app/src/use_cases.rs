use crate::{database::DatabaseManager, models::TransacaoRequest, tratamento_erros::Error};
use crate::models::{ExtratoResponse, TransacaoResponse};

pub async fn efetua_transacao(
    id: i32,
    request: TransacaoRequest,
    database: DatabaseManager,
) -> Result<TransacaoResponse, Error> {
    match database.execute_procedure(id, request).await {
        Ok(extrato_response) => {
            println!("database.execute_procedure: {:?}", extrato_response);
            return Ok(extrato_response);
        }
        Err(e) => return Err(e),
    };
}

pub async fn consulta_extrato(id: i32, database: DatabaseManager) -> Result<ExtratoResponse, Error> {
    match database.consulta_extrato(id).await {
        Ok(extrato_response) => {
            println!("database.consulta_extrato: {:?}", extrato_response);
            return Ok(extrato_response);
        }
        Err(e) => return Err(e),
    };
}

/*  
pub async fn executa_transacao(
    id: i32,
    request: TransacaoRequest,
    cliente_dao: ClienteDao,
) -> Result<TransacaoResponse, Error> {
    let cliente = match cliente_dao.get_cliente(id).await {
        Ok(cliente) => {
            println!("cliente_dao.get_cliente: {:?}", cliente);
            cliente
        }
        Err(e) => return Err(e),
    };

    let mut novo_saldo = 0;
    if request.tipo == 'd' {
        println!("transacao de debito: subtraindo saldo do cliente");
        novo_saldo = cliente.saldo - request.valor;
        match valida_limite_disponivel(cliente.limite, novo_saldo) {
            Err(e) => return Err(e),
            _ => (),
        };
    } else if request.tipo == 'c' {
        println!("transacao de credito: adicionando valor ao saldo do cliente");
        novo_saldo = cliente.saldo + request.valor;
    }

    let transacao_entity = TransacaoEntity {
        id: None,
        cliente_id: id,
        valor: request.valor,
        tipo: request.tipo,
        descricao: request.descricao,
        realizada_em: None,
    };

    match cliente_dao
        .efetura_transacao(id, novo_saldo, transacao_entity)
        .await
    {
        Err(e) => return Err(e),
        _ => {
            println!("transacao realizada com sucesso");
            ()
        }
    }

    return Ok(TransacaoResponse {
        limite: cliente.limite,
        saldo: novo_saldo,
    });
}

fn valida_limite_disponivel(limite: i32, valor: i32) -> Result<(), Error> {
    if valor < 0 && valor.abs() > limite {
        return Err(Error::LimiteIndisponivel);
    }
    Ok(())
}

#[cfg(test)]
mod transacao_test {
    use crate::use_cases::valida_limite_disponivel;

    #[test]
    fn se_limite_nao_for_suficiente_deve_retornar_erro() {
        let limite = 1000;
        let valor = -1001;

        let response = valida_limite_disponivel(limite, valor);

        assert!(response.is_err());
    }

    #[test]
    fn se_limite_nao_for_suficiente_deve_retornar_erro_2() {
        let limite = 1000;
        let valor = -3000;

        let response = valida_limite_disponivel(limite, valor);

        assert!(response.is_err());
    }

    #[test]
    fn se_limite_for_suficiente_nao_deve_retornar_erro() {
        let limite = 1000;
        let valor = -1000;

        let response = valida_limite_disponivel(limite, valor);

        assert!(response.is_ok());
    }

    #[test]
    fn se_limite_for_suficiente_nao_deve_retornar_erro_2() {
        let limite = 1000;
        let valor = -900;

        let response = valida_limite_disponivel(limite, valor);

        assert!(response.is_ok());
    }

    #[test]
    fn se_valor_for_positivo_nao_deve_retornar_erro() {
        let limite = 1000;
        let valor = 500;

        let response = valida_limite_disponivel(limite, valor);

        assert!(response.is_ok());
    }
}
*/