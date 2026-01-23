#![no_std]

use soroban_sdk::{contracttype, Symbol, Address};

#[contracttype]
#[derive(Clone)]
pub struct Product {
    pub id: u32,
    pub name: Symbol,
    pub owner: Address,
    pub price: u64, // standardised (matches agriculture)
}

pub mod agriculture;
pub mod enums;
pub mod types;
