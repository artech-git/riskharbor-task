use web3::futures::StreamExt;
use web3::{transports::WebSocket, Web3};
use web3::types::{BlockNumber, BlockId, U64};
use web3::types::FilterBuilder;
use tokio::runtime::Handle;
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

     fn new(addr: String, filter: Option<Filter>) -> Self {

    }

    fn start_listener(&mut self) -> Result<(), BackendError> {

        Handle.current().spawn(async {

            let client = Client::new();
            let response = client
                .get(SSE_HTTP_URL)
                .send()
                .await?;
            
            let mut stream = response.bytes_stream();
            
            tokio::spawn(async {

                let local_client = Client::new();
                
                while let Some(item) = stream.next().await {
                    let bytes = item?;
                    let event: Value = serde_json::from_slice(&bytes)?;

                    let slot = event["data"]["slot"].parse::<u64>();

                    let epoch_transistion = event["data"]["epoch_transition"].parse::<bool>();
                    if let Ok(true) == epoch_transistion {
                        self.db.insert(slot);
                    }

                    let block_info = local_client
                            .get(
                                format!("{QUICKNODE_BASE_URL}/eth/v1/beacon/headers/{:?}",slot)
                            ).send();
                    let attestations_info = local_client
                            .get(
                                format!("{QUICKNODE_BASE_URL}//eth/v1/beacon/blocks/{:?}/attestations",slot)
                            ).send();
                    let query_1 = format!(" ")
                    let query_2 = format!(" ")
                    
                    self.db.insert(Block::from(event))    
                    
                    
                }
                
            })
        });

        OK(())

    }

}

