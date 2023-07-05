use web3::futures::StreamExt;
use web3::{transports::WebSocket, Web3};
use web3::types::{BlockNumber, BlockId, U64};
use web3::types::FilterBuilder;

mod constants;
mod objects;
mod db;


use db::Database;

#[derive(Debug, Clone)]
pub struct consensusData{

}

impl consensusData {
    fn compute_network_participation() -> Result<Option<f64>, BackendError> {

    }

    fn compute_validators_participation(id: u64) -> Result<Option<f64>, BackendError> {

    }
}


#[derive(Debug, Clone)]
pub struct EpochData {

}


#[derive(Debug, Clone,)]
pub struct SlotIndexer {
    consensus_data: consensusData,
    epochs: VecDeque<EpochData>,
    addr: String,
    eventHandle: JoinHandle<_>,
    db: Arc<Database>
}

impl SlotIndexer {

    async fn new(addr: String, filter: Option<Filter>) -> Self {

    }

    async fn start_listener(&mut self) -> Result<(), BackendError> {

        let client = Client::new();
        let response = client
            .get(SSE_HTTP_URL)
            .send()
            .await?;

        let mut stream = response.bytes_stream();

        tokio::spawn(async {
            
            while let Some(item) = stream.next().await {
                let bytes = item?;
                let event: Value = serde_json::from_slice(&bytes)?;
                let epoch_transistion = event["data"]["epoch_transition"].parse::<bool>();

                if let Ok(true) == epoch_transistion {
                    self.db.insert(slot);
                }

                self.db.insert(Block::from(event))    

            }
            
        })

    }

}

