#![no_std]
use soroban_sdk::{contract, contractimpl, vec, Env, String, Vec};

#[contract]
pub struct Contract;

// This is a sample contract. Replace this placeholder with your own contract logic.
// A corresponding test example is available in `test.rs`.
//
// For comprehensive examples, visit <https://github.com/stellar/soroban-examples>.
// The repository includes use cases for the Stellar ecosystem, such as data storage on
// the blockchain, token swaps, liquidity pools, and more.
//
// Refer to the official documentation:
// <https://developers.stellar.org/docs/build/smart-contracts/overview>.
#[contractimpl]
impl Contract {
    pub fn hello(env: Env, to: String) -> Vec<String> {
        vec![&env, String::from_str(&env, "Hello"), to]
    }
}

#![no_std]

use soroban_sdk::{contract, contractimpl, Env, Map, String, Vec, Address};

#[derive(Clone)]
pub struct Item {
    pub name: String,
    pub price_per_day: u32,
    pub available: bool,
}

#[contract]
pub struct RentalContract;

#[contractimpl]
impl RentalContract {
    pub fn add_item(env: Env, id: String, name: String, price_per_day: u32) {
        let mut items: Map<String, Item> = env.storage().get_unchecked(&"items").unwrap_or_default();
        items.set(id.clone(), Item { name, price_per_day, available: true });
        env.storage().set_unchecked(&"items", &items);
    }

    pub fn get_item(env: Env, id: String) -> Option<Item> {
        let items: Map<String, Item> = env.storage().get_unchecked(&"items").unwrap_or_default();
        items.get(id)
    }

    pub fn rent_item(env: Env, id: String) -> Result<(), String> {
        let mut items: Map<String, Item> = env.storage().get_unchecked(&"items").unwrap_or_default();
        let mut item = items.get(id.clone()).ok_or("Item not found".to_string())?;
        if !item.available {
            return Err("Item already rented".to_string());
        }
        item.available = false;
        items.set(id, item);
        env.storage().set_unchecked(&"items", &items);
        Ok(())
    }

    pub fn return_item(env: Env, id: String) -> Result<(), String> {
        let mut items: Map<String, Item> = env.storage().get_unchecked(&"items").unwrap_or_default();
        let mut item = items.get(id.clone()).ok_or("Item not found".to_string())?;
        if item.available {
            return Err("Item is not rented".to_string());
        }
        item.available = true;
        items.set(id, item);
        env.storage().set_unchecked(&"items", &items);
        Ok(())
    }
}
