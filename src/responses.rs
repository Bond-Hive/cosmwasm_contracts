use cosmwasm_schema::cw_serde;

#[cw_serde]
pub struct CountResponse {
    pub count: u32,
}
