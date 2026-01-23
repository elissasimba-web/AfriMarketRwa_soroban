#![cfg(test)]

use super::*;
use escrow::EscrowContract;
use escrow::EscrowContractClient;

use soroban_sdk::{
    Env, Address,
    testutils::{Address as TestAddress},
};

#[test]
fn full_rental_escrow_flow() {
    let env = Env::default();
    env.mock_all_auths();

    // 👤 Abantu
    let owner = Address::generate(&env);
    let renter = Address::generate(&env);

    // 🧾 Register Escrow contract
    let escrow_id = env.register_contract(None, EscrowContract);
    let escrow_addr = Address::from_contract_id(&env, escrow_id);
    let escrow = EscrowContractClient::new(&env, &escrow_addr);

    // 🏠 Register Rental contract
   let contract_id = env.register(RentalContract);
    let rental = RentalContractClient::new(&env, &rental_id);

    // Init rental with escrow address
    rental.init(&escrow_addr);

    // ➕ Add item
    rental.add_item(
        &owner,
        &1,
        &Symbol::short("CAR"),
        &100,
    );

    // 💰 Deposit escrow
    let fake_xlm = Address::generate(&env); // token mock
    escrow.deposit(&fake_xlm, &renter, &500);

    // 🚗 Rent item for 2 days
    rental.rent_item(
        &1,
        &renter,
        &2,
        &500,
    );

    // ⏰ Simulate time pass (3 days → 1 day late)
    env.ledger().set_timestamp(3 * 86400);

    // 🔁 Return item
    let (_days_used, refund, penalty) =
        rental.return_item(&1);

    // 🧮 Assertions
    assert!(penalty > 0);
    assert!(refund < 500);

    let remaining = escrow.balance(&renter);
    assert_eq!(remaining, 0);
}
