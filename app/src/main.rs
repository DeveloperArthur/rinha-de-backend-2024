#![warn(clippy::all,)]

use warp::Filter;

mod models;
mod tratamento_erros;
mod controllers;
mod use_cases;
mod database;

#[tokio::main]
async fn main() {
    let database = database::DatabaseManager::connect_with_database().await;
    let database_filter = warp::any().map(move || database.clone());

    /* POST /clientes/[id]/transacoes */
    let transacao = warp::post()
        .and(warp::path("clientes"))
        .and(warp::path::param::<i32>())
        .and(warp::path("transacoes"))
        .and(warp::path::end())
        .and(warp::body::json())
        .and(database_filter.clone())
        .and_then(controllers::transacao_controller);

    /* GET /clientes/[id]/extrato */
    let extrato = warp::get()
        .and(warp::path("clientes"))
        .and(warp::path::param::<i32>())
        .and(warp::path("extrato"))
        .and(warp::path::end())
        .and(database_filter.clone())
        .and_then(controllers::extrato_controller);

    let routes = transacao
        .or(extrato)
        .recover(tratamento_erros::return_error);

    println!("Started server at localhost:3030");
    warp::serve(routes).run(([0, 0, 0, 0], 3030)).await;
}
