use cosmwasm_schema::{cw_serde, QueryResponses};
use sei_cosmwasm::{
  ContractOrderResult
};

#[cw_serde]
pub struct InstantiateMsg {
    pub count: i32,
}

#[cw_serde]
pub enum ExecuteMsg {
    Increment {},
    IncrementAdv { contract_order_results: Vec<ContractOrderResult> },
    Reset { count: i32 },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    // GetCount returns the current count as a json-encoded number
    #[returns(GetStateResponse)]
    GetState {},
}

// We define a custom struct for each query response
#[cw_serde]
pub struct GetStateResponse {
    pub count: i32,
    pub order_id: u64
}
