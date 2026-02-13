#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, Address, Env, Symbol};

#[test]
fn add_and_get_item_works() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, RentalContract);
    let client = RentalContractClient::new(&env, &contract_id);

    let owner = Address::generate(&env);
    let escrow = Address::generate(&env);
    let xlm_token = Address::generate(&env);

    client.init(&escrow, &xlm_token);
    client.add_item(&owner, &1, &Symbol::new(&env, "CAR"), &100);

    let item = client.get_item(&1);
    assert_eq!(item.id, 1);
    assert_eq!(item.price_per_day, 100);
    assert!(!item.rented);
}
