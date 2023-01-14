mod args;
mod config;
mod metrics;
mod serve;

use monero_rpc::RpcClientBuilder;
use hyper::{
	server::conn::AddrStream,
	service::{make_service_fn, service_fn},
	Server,
};
use std::sync::Arc;
use monero_rpc::monero::Error;

use crate::{
	args::Args,
	config::Config,
	serve::serve_req,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	// setup logging
	env_logger::init_from_env(
		env_logger::Env::new()
			.default_filter_or("monerod_exporter=info")
			.default_write_style_or("auto"),
	);
	log::info!("{} v{}", env!("CARGO_BIN_NAME"), env!("CARGO_PKG_VERSION"));

	// parse args
	let args: Args = args::from_env();
	// parse yaml config
	let config = Config::read(&args.config)?;
	let addr = &config.bind.parse()?;

	// create rpc client
	let rpc= RpcClientBuilder::new().build(&config.host)?;
	let client = Arc::new(rpc.metrics());

	let serve_future = make_service_fn(move |socket: &AddrStream| {
		log::info!("listening on http://{}", addr);
		let client = client.clone();
		let addr = socket.remote_addr();
		async move {
			Ok::<_, Error>(service_fn(move |req| {
				let client = client.clone();
				serve_req(req, addr, client)
			}))
		}
	});

	// launch server
	let server = Server::bind(addr).serve(serve_future);
	if let Err(err) = server.await {
		log::error!("server error: {}", err);
	}
	Ok(())
}
