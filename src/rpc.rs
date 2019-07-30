// Copyright 2018 akropolis

use akro_api::AkroApi;
use akro_api::TClient;
use akro_pool::TransactionPool;
use akro_primitives;
use akro_rpc;
use clap;
use cli;
use jsonrpc_http_server::Server as HttpServer;
use jsonrpc_ws_server::Server as WsServer;
use rpc_server;
use std::io;
use tokio::runtime::TaskExecutor;
use Arc;

pub fn start<A>(
    client: &Arc<TClient>,
    task_executor: &TaskExecutor,
    matches: &clap::ArgMatches,
    extrinsic_pool: &Arc<TransactionPool<A>>,
) -> (
    Result<Option<HttpServer>, io::Error>,
    Result<Option<WsServer>, io::Error>,
)
where
    A: AkroApi + Send + Sync + 'static,
{
    let handler = || {
        let subscriptions = rpc_server::apis::Subscriptions::new(task_executor.clone());

        let chain = rpc_server::apis::chain::Chain::new(client.clone(), subscriptions.clone());
        let chain_ext = akro_rpc::chainext::ChainExt::new(client.clone(), task_executor.clone());
        let state = rpc_server::apis::state::State::new(client.clone(), subscriptions.clone());
        let author = rpc_server::apis::author::Author::new(
            client.clone(),
            extrinsic_pool.inner().clone(),
            subscriptions.clone(),
        );
        akro_rpc::servers::rpc_handler::<
            akro_primitives::Block,
            akro_primitives::Hash,
            _,
            _,
            _,
            _,
            _,
            _,
        >(
            state,
            chain,
            chain_ext,
            author,
            akro_rpc::default_rpc_config(),
        )
    };
    let rpc_interface: &str = if matches.is_present("rpc-external") {
        "0.0.0.0"
    } else {
        "127.0.0.1"
    };
    let ws_interface: &str = if matches.is_present("ws-external") {
        "0.0.0.0"
    } else {
        "127.0.0.1"
    };
    let rpc_http_addr = Some(
        cli::parse_address(&format!("{}:{}", rpc_interface, 8081), "rpc-port", &matches).unwrap(),
    );
    let rpc_ws_addr = Some(
        cli::parse_address(&format!("{}:{}", ws_interface, 8082), "ws-port", &matches).unwrap(),
    );

    let rpc_http: Result<Option<HttpServer>, io::Error> =
        akro_rpc::maybe_start_server(rpc_http_addr, |address| {
            akro_rpc::servers::start_http(address, handler())
        });

    let rpc_ws: Result<Option<WsServer>, io::Error> =
        akro_rpc::maybe_start_server(rpc_ws_addr, |address| {
            akro_rpc::servers::start_ws(address, handler())
        });

    (rpc_http, rpc_ws)
}
