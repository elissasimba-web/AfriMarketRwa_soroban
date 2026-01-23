use soroban_sdk::{contracttype, Address, Symbol};

#[contracttype]
#[derive(Clone)]
pub struct AgriItem {
    pub id: u64,
    pub owner: Address,
    pub name: Symbol,
    pub price: u64,
    pub quantity: u64,
}

#[contracttype]
#[derive(Clone)]
pub struct AgriOrder {
    pub id: u64,
    pub item_id: u64,
    pub buyer: Address,
    pub total_price: u64,
}
