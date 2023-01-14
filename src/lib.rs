
// #[derive(Clone, Debug)]
// pub struct MetricsClient {
//     inner: CallerWrapper,
// }
//
// impl MetricsClient {
//     pub fn new(client: RpcClient) -> MetricsClient {
//         let Self { inner } = client;
//         MetricsClient { inner }
//     }
//
//     pub async fn get_sync_info(&self) -> anyhow::Result<NonZeroU64> {
//         #[derive(Deserialize)]
//         struct Rsp {
//             height: NonZeroU64,
//             target_height: NonZeroU64,
//         }
//
//         Ok(self
//             .inner
//             .request::<Rsp>("sync_info", RpcParams::None)
//             .await?
//             .height)
//     }
// }
