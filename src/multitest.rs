use sylvia::cw_multi_test::IntoAddr;
use sylvia::multitest::App;

use crate::contract::sv::mt::{CodeId, CounterContractProxy};
use crate::error::ContractError;
use crate::whitelist::sv::mt::WhitelistProxy;

#[test]
fn instantiate() {
    let app = App::default();
    let code_id = CodeId::store_code(&app);

    let owner = "owner".into_addr();

    let contract = code_id.instantiate(42).call(&owner).unwrap();

    let count = contract.count().unwrap().count;
    assert_eq!(count, 42);

    contract.increment_count().call(&owner).unwrap();

    let count = contract.count().unwrap().count;
    assert_eq!(count, 43);
}

#[test]
fn decrement_below_zero() {
    let app = App::default();
    let code_id = CodeId::store_code(&app);

    let owner = "owner".into_addr();

    let contract = code_id.instantiate(1).call(&owner).unwrap();

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

    let contract = code_id.instantiate(1).call(&owner).unwrap();

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
