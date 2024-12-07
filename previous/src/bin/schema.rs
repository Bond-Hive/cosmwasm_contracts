use cosmwasm_schema::write_api;

use bond_hive::contract::sv::{ContractExecMsg, ContractQueryMsg, InstantiateMsg};
use sylvia::cw_std::Empty;

fn main() {
    write_api! {
        instantiate: InstantiateMsg,
        execute: ContractExecMsg<Empty, Empty>,
        query: ContractQueryMsg<Empty, Empty>,
    }
}
