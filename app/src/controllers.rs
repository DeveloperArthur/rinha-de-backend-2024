use crate::database::DatabaseManager;
use crate::models::TransacaoRequest;
use crate::tratamento_erros::Error;
use crate::use_cases;

pub async fn transacao_controller(
    id: i32,
    request: TransacaoRequest,
    database: DatabaseManager,
) -> Result<impl warp::Reply, warp::Rejection> {
    if request.descricao.len() > 10 || (request.tipo != 'c' && request.tipo != 'd') {
        return Err(warp::reject::custom(Error::PayloadInvalido));
    }

    //TODO: tirar essa gambiarra maldita e tratar esse erro no retorno da procedure
    if id < 1 || id > 5 {
        return Err(warp::reject::custom(Error::ClientNotFound));
    }

    match use_cases::efetua_transacao(id, request, database).await {
        Ok(transacao_response) => Ok(warp::reply::json(&transacao_response)),
        Err(e) => Err(warp::reject::custom(e)),
    }
}

pub async fn extrato_controller(
    id: i32,
    database: DatabaseManager,
) -> Result<impl warp::Reply, warp::Rejection> {
    match use_cases::consulta_extrato(id, database).await {
        Ok(extrato_response) => Ok(warp::reply::json(&extrato_response)),
        Err(e) => Err(warp::reject::custom(e)),
    }
}
