use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Serialize)]
pub struct TransacaoRequest {
    pub valor: i32,
    pub tipo: char,
    pub descricao: String,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct TransacaoResponse {
    pub limite: i32,
    pub saldo: i32,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct ExtratoResponse {
    pub saldo: SaldoResponse,
    pub ultimas_transacoes: Vec<UltimasTransacoesResponse>,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct SaldoResponse {
    pub total: i32,
    pub data_extrato: String,
    pub limite: i32,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct UltimasTransacoesResponse {
    pub valor: i32,
    pub tipo: String,
    pub descricao: String,
    pub realizada_em: String,
}