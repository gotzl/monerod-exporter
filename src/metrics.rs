use lazy_static::lazy_static;
use prometheus::{
	register_gauge, Gauge,
};

lazy_static! {

	pub(crate) static ref MONEROD_BLOCK_DIFFICULTY: Gauge = register_gauge!("monerod_block_difficulty", "Last block difficulty").unwrap();
	pub(crate) static ref MONEROD_CONNECTIONS_INCOMING: Gauge = register_gauge!("monerod_connections_incoming", "Number of incoming connections").unwrap();
	pub(crate) static ref MONEROD_TX_MEMPOOL: Gauge = register_gauge!("monerod_tx_mempool", "Number of transactions in the mempool").unwrap();
	pub(crate) static ref MONEROD_CONNECTIONS_OUTGOING: Gauge = register_gauge!("monerod_connections_outgoing", "Number of outgoing connections").unwrap();
	pub(crate) static ref MONEROD_BLOCK_REWARD: Gauge = register_gauge!("monerod_block_reward", "Last block reward").unwrap();
	pub(crate) static ref MONEROD_TX_CHAIN: Gauge = register_gauge!("monerod_tx_chain", "Number of transactions in total").unwrap();
	pub(crate) static ref MONEROD_DATABASE_SIZE: Gauge = register_gauge!("monerod_database_size", "Number of blockchain size in bytes").unwrap();
	pub(crate) static ref MONEROD_HEIGHT: Gauge = register_gauge!("monerod_height", "Number of blockheight").unwrap();
	pub(crate) static ref MONEROD_UPDATE_AVAILABLE: Gauge = register_gauge!("monerod_update_available", "Boolean of update available").unwrap();
	pub(crate) static ref MONEROD_IS_MINING_ACTIVE: Gauge = register_gauge!("monerod_is_mining_active", "Status of mining active").unwrap();
	pub(crate) static ref MONEROD_MINIG_THREADS: Gauge = register_gauge!("monerod_mining_threads", "Number of active mining threads").unwrap();
	pub(crate) static ref MONEROD_MINING_SPEED: Gauge = register_gauge!("monerod_mining_speed", "Number of mining speed in hashes per second").unwrap();
	pub(crate) static ref MONEROD_SYNC_PERCENTAGE: Gauge = register_gauge!("monerod_sync_percentage", "Percentage of synced blocks").unwrap();

}
