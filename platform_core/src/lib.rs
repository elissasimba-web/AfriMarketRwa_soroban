#![no_std]

use soroban_sdk::{contracttype, Address, Symbol};

#[contracttype]
#[derive(Clone)]
pub struct Product {
    pub id: u32,
    pub name: Symbol,
    pub owner: Address,
    pub price: u64,
}

pub mod agriculture;
pub mod enums;
pub mod types;

pub use enums::{OrderStatus, ProductStatus};
