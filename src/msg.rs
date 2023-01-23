use serde::{Deserialize, Serialize};
use schemars::JsonSchema;
use crate::sei_type::{ContractOrderResult};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub count: i32,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum ExecuteMsg {
    Increment {},
    IncrementAdv { contract_order_results: Vec<ContractOrderResult> },
    Reset { count: i32 },
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum QueryMsg {
    // GetCount returns the current count as a json-encoded number
    GetState {},
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct GetStateResponse {
    pub count: i32,
    pub order_id: u64
}
