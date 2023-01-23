use serde::{Deserialize, Serialize};
use cosmwasm_std::Decimal;
use schemars::JsonSchema;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ContractOrderResult {
    pub contract_address: String,
    pub order_placement_results: Vec<OrderPlacementResult>,
    pub order_execution_results: Vec<OrderExecutionResult>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct OrderPlacementResult {
    pub order_id: u64,
    pub status_code: i32,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct OrderExecutionResult {
    pub order_id: u64,
    pub execution_price: Decimal,
    pub executed_quantity: Decimal,
    pub total_notional: Decimal,
    pub position_direction: String,
}
