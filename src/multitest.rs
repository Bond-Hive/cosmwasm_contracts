use sylvia::cw_multi_test::IntoAddr;
use sylvia::multitest::App;

use crate::contract::sv::mt::{CodeId, CounterContractProxy};
use crate::error::ContractError;
use crate::whitelist::sv::mt::WhitelistProxy;

#[test]
fn instantiate() {
    let app = App::default();
    let code_id = CodeId::store_code(&app);

    // Use valid Bech32-compliant addresses
    let owner = "cosmos1w2kvwrzp23aq54n3amwav4yy4a9ahq2kz2wtmr".into_addr();
    let cw_20_token = "cosmos1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq9v5e0h".into_addr();

    println!("Owner: {:?}, CW20 Token: {:?}", owner, cw_20_token);

    // Instantiate the contract
    let contract = code_id
        .instantiate(42, cw_20_token.to_string())
        .call(&owner)
        .unwrap();

    // Verify the stored count
    let count = contract.count().unwrap().count;
    assert_eq!(count, 42);

    // Verify the stored CW20 token address
    let stored_token = contract.cw_20_address().unwrap().cw_20_address;
    println!("Stored CW20 token address: {:?}", stored_token);
    assert_eq!(stored_token, cw_20_token);
}

#[test]
fn decrement_below_zero() {
    let app = App::default();
    let code_id = CodeId::store_code(&app);

    let owner = "owner".into_addr();
    let cw_20_token = "cw20_token".into_addr();

    let contract = code_id
        .instantiate(1, cw_20_token.to_string())
        .call(&owner)
        .unwrap();

    let count = contract.count().unwrap().count;
    assert_eq!(count, 1);

    contract.decrement_count().call(&owner).unwrap();

    let count = contract.count().unwrap().count;
    assert_eq!(count, 0);

    let err = contract.decrement_count().call(&owner).unwrap_err();
    assert_eq!(err, ContractError::CannotDecrementCount);
}

#[test]
fn manage_admins() {
    let app = App::default();
    let code_id = CodeId::store_code(&app);

    let owner = "owner".into_addr();
    let admin = "admin".into_addr();
    let cw_20_token = "cw20_token".into_addr();

    let contract = code_id.instantiate(1, cw_20_token.to_string())
    .call(&owner).unwrap();

    // Admins list is empty
    let admins = contract.admins().unwrap().admins;
    assert!(admins.is_empty());

    // Admin can be added
    contract.add_admin(admin.to_string()).call(&owner).unwrap();

    let admins = contract.admins().unwrap().admins;
    assert_eq!(admins, vec![&admin]);

    // Admin can be removed
    contract
        .remove_admin(admin.to_string())
        .call(&owner)
        .unwrap();

    let admins = contract.admins().unwrap().admins;
    assert!(admins.is_empty());
}
