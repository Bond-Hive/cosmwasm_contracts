use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;

#[cw_serde]
pub struct CountResponse {
    pub count: u32,
}

#[cw_serde]
pub struct AdminsResponse {
    pub admins: Vec<Addr>,
}

#[cw_serde]
pub struct Cw20AddressResponse {
    pub cw_20_address: Addr,
}
