use anyhow::Result;
use monero_rpc::InfoClient;
use hyper::{header::CONTENT_TYPE, Body, Method, Request, Response};
use prometheus::{Encoder, TextEncoder};
use std::{net::SocketAddr, sync::Arc};

use crate::metrics::{
	MONEROD_BLOCK_DIFFICULTY, MONEROD_CONNECTIONS_INCOMING, MONEROD_TX_MEMPOOL, MONEROD_CONNECTIONS_OUTGOING,
	MONEROD_BLOCK_REWARD, MONEROD_TX_CHAIN, MONEROD_DATABASE_SIZE, MONEROD_HEIGHT, MONEROD_UPDATE_AVAILABLE,
	MONEROD_IS_MINING_ACTIVE, MONEROD_MINIG_THREADS, MONEROD_MINING_SPEED, MONEROD_SYNC_PERCENTAGE,
};

async fn get_metrics(rpc: Arc<InfoClient>) -> Result<()> {
	{
		let info = rpc.get_info().await?;
		{
			MONEROD_BLOCK_DIFFICULTY.set(info.difficulty as f64);
			MONEROD_CONNECTIONS_INCOMING.set(info.incoming_connections_count as f64);
			MONEROD_CONNECTIONS_OUTGOING.set(info.outgoing_connections_count as f64);
			MONEROD_TX_MEMPOOL.set(info.tx_pool_size as f64);
			MONEROD_TX_CHAIN.set(info.tx_count as f64);
			MONEROD_DATABASE_SIZE.set(info.database_size as f64);
			MONEROD_HEIGHT.set(info.height as f64);
			MONEROD_UPDATE_AVAILABLE.set(info.update_available as u8 as f64);
		}
		let mining_status = rpc.mining_status().await?;
		if info.restricted {
			MONEROD_IS_MINING_ACTIVE.set(-1.);
			MONEROD_MINIG_THREADS.set(-1.);
			MONEROD_MINING_SPEED.set(-1.);
		} else {
			MONEROD_IS_MINING_ACTIVE.set(mining_status.active as u8 as f64);
			MONEROD_MINIG_THREADS.set(mining_status.threads_count as f64);
			MONEROD_MINING_SPEED.set(mining_status.speed as f64);
		}
	}

	{
		let syncinfo = rpc.sync_info().await?;
		let sync_percentage = {
			if syncinfo.target_height > syncinfo.height {
				(syncinfo.height as f64 / syncinfo.target_height as f64) * 100.
			} else {
				100.
			}
		};
		MONEROD_SYNC_PERCENTAGE.set(sync_percentage);
	}

	{
		let last_block_header = rpc.last_block_header().await?;
		MONEROD_BLOCK_REWARD.set(last_block_header.block_header.reward.as_xmr() as f64)
	}

	Ok(())
}

/// Create Prometheus metrics to track monerod stats.
pub(crate) async fn serve_req(
	req: Request<Body>,
	addr: SocketAddr,
	rpc: Arc<InfoClient>,
) -> Result<Response<Body>> {
	if req.method() != Method::GET || req.uri().path() != "/metrics" {
		log::warn!("  [{}] {} {}", addr, req.method(), req.uri().path());
		return Ok(Response::builder()
			.status(404)
			.body(Body::default())
			.unwrap());
	}

	let response = match get_metrics(rpc).await {
		Ok(_) => {
			let metric_families = prometheus::gather();
			let encoder = TextEncoder::new();
			let mut buffer = vec![];
			encoder.encode(&metric_families, &mut buffer).unwrap();

			Response::builder()
				.status(200)
				.header(CONTENT_TYPE, encoder.format_type())
				.body(Body::from(buffer))
				.unwrap()
		}
		Err(e) => Response::builder()
			.status(404)
			.header(CONTENT_TYPE, "text/plain")
			.body(Body::from(e.to_string()))
			.unwrap(),
	};
	Ok(response)
}
