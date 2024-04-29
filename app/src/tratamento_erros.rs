use warp::http::StatusCode;
use warp::{
    reject::{Reject, Rejection},
    reply::Reply,
};

#[derive(Debug)]
pub enum Error {
    LimiteIndisponivel,
    PayloadInvalido,
    DatabaseQueryError,
    ClientNotFound,
}

impl Reject for Error {}

pub async fn return_error(r: Rejection) -> Result<impl Reply, Rejection> {
    if let Some(Error::LimiteIndisponivel) = r.find() {
        Ok(warp::reply::with_status(
            "Limite indisponível",
            StatusCode::UNPROCESSABLE_ENTITY,
        ))
    } else if let Some(Error::ClientNotFound) = r.find() {
        Ok(warp::reply::with_status(
            "Cliente nao existe",
            StatusCode::NOT_FOUND,
        ))
    } else if let Some(Error::PayloadInvalido) = r.find() {
        Ok(warp::reply::with_status(
            "Payload Inválido",
            StatusCode::UNPROCESSABLE_ENTITY,
        ))
    } else if let Some(Error::DatabaseQueryError) = r.find() {
        Ok(warp::reply::with_status(
            "Query could not be executed",
            StatusCode::NOT_FOUND,
        ))
    } else {
        Ok(warp::reply::with_status(
            "Payload Inválido",
            StatusCode::UNPROCESSABLE_ENTITY,
        ))
    }
}
