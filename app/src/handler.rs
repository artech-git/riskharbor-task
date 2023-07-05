use axum::{handler::get, Router};
use axum::http::StatusCode;
use axum::response::{Json, IntoResponse};
use serde_json::json;
use axum::{handler::get, Router};

// Implement the logic to fetch and return the participation rate for a specific validator.
async fn get_validator_participation(Path(validator_id): Path<u64>, db_state: Extension<Arc<State>>) -> impl Response {
    
    let slots_per_epoch: u64 = 5;
    
    let client = Client::new();

    let validator_url = format!("https://your-quicknode-endpoint/eth/v1/validator/{}", validator_id);
    let attestations_url = "https://your-quicknode-endpoint/eth/v1/validator/attestations";

    // Retrieve validator information
    let validator_info: serde_json::Value = client.get(&validator_url).send().await.unwrap().json().await.unwrap();

    // Retrieve attestations for the validator
    let attestations: serde_json::Value = client.get(attestations_url).send().await.unwrap().json().await.unwrap();

    let mut num_missed_attestations = 0;

    // Filter attestations for the 5 most recent epochs
    // Calculate the number of missed attestations for the validator
    // ...

    let participation_rate = 1.0 - (num_missed_attestations as f64 / (5 * slots_per_epoch as f64));

    Ok(participation_rate)
    
}

pub async fn get_network_participation(db_state: Extension<Arc<State>>) -> Impl Response {

    let missedAttestations = 
    let validatorSetSize = 
    let slotsPerEpoch = 

    let attestation_api = "eth/v1/beacon/blocks/{block_id}/attestations"


}