#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env};

#[contract]
pub struct EscrowContract;

#[contractimpl]
impl EscrowContract {
    pub fn pay_with_xlm(
        env: Env,
        renter: Address,
        owner: Address,
        rent: i128,
        deposit: i128,
    ) {
        renter.require_auth();
        let total = rent + deposit;
        env.events().publish(("pay",), (renter, owner, rent, deposit, total));
    }

    pub fn settle_return(
        env: Env,
        renter: Address,
        refund: i128,
        penalty: i128,
    ) {
        env.events().publish(("settle",), (renter, refund, penalty));
    }
}
