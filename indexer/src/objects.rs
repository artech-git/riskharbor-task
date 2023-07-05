use serde::{Deserialize, Serialize};
use futures::stream::Stream;

#[derive(Debug, Serialize, Deserialize)]
pub struct Validator {
    Validator_ID: i32,
    Validator_Name: String,
    PublicKey: String,
    Balance: f32,
    Status: String,
    Activation_Epoch: i32,
    Exit_Epoch: i32,
    Slashed: bool,
}

async fn parse_validators_response(response: Value) -> impl Stream<Item = Validator> {
    let validators_json = response["data"].as_array().unwrap();

    futures::stream::iter(
        validators_json
            .into_iter()
            .map(|validator_json| {
                let validator: Validator = serde_json::from_value(validator_json.clone()).unwrap();
                validator
        })
    )
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Block {
    Block_ID: String,
    Slot: u32,
    Parent_Root: String,
    State_Root: String,
    Body_Root: String,
    Signature: String,
}

impl From<Value> for Block {
    fn from(value: Value) -> Self {
        
        let slot = value["data"]["slot"].as_str().unwrap().parse().unwrap();

        let block_id = value["data"]["block"].as_str().unwrap().to_string();
        
        let parent_root = value["previous_duty_dependent_root"].as_str().unwrap().to_string();
        
        let state_root = value["data"]["state"].as_str().unwrap().to_string();
        
        let body_root = value["current_duty_dependent_root"].as_str().unwrap().to_string();
        
        let signature = value["signature"].as_str().unwrap().to_string();

        Block {
            Block_ID: block_id,
            Slot: slot,
            Parent_Root: parent_root,
            State_Root: state_root,
            Body_Root: body_root,
            Signature: signature,
        }
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Attestation {
    Attestation_ID: u32,
    Block_ID: String,
    Aggregation_Bits: String,
    Data: String,
    Slot: u32,
    Index: u32,
    Beacon_Block_Root: String,
    Source: String,
    Epoch: u32,
    Root: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AggregateAttestation {
    Aggregate_Attestation_ID: i32,
    Attestation_Data_Root: String,
    Slot: i32,
    Aggregation_Bits: String,
    Signature: String,
    Data: String,
    Index: i32,
    Beacon_Block_Root: String,
    Source: String,
    Epoch: i32,
    Root: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SyncCommitteeContribution {
    Contribution_ID: i32,
    Slot: i32,
    Subcommittee_Index: i32,
    Beacon_Block_Root: String,
    Aggregation_Bits: String,
    Signature: String,
}
