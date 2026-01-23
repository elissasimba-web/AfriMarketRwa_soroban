#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Env, Address, Map, Symbol};
use platform_core::agriculture::{AgriItem, AgriOrder};

#[contracttype]
pub enum DataKey {
    Items,
    Orders,
}

#[contract]
pub struct AgricultureContract;

#[contractimpl]
impl AgricultureContract {
    pub fn init(env: Env) {
        env.storage().instance().set(&DataKey::Items, &Map::<u32, AgriItem>::new(&env));
        env.storage().instance().set(&DataKey::Orders, &Map::<u32, AgriOrder>::new(&env));
    }

    pub fn add_item(
        env: Env,
        seller: Address,
        id: u32,
        name: Symbol,
        price: u64,
        quantity: u32,
    ) {
        seller.require_auth();

        let mut items: Map<u32, AgriItem> =
            env.storage().instance().get(&DataKey::Items).unwrap();

        let item = AgriItem {
            id: id as u64,
            owner: seller,
            name,
            price,
            quantity: quantity as u64,
        };

        items.set(id, item);
        env.storage().instance().set(&DataKey::Items, &items);
    }

    pub fn buy_item(
        env: Env,
        order_id: u32,
        item_id: u32,
        buyer: Address,
        quantity: u32,
    ) {
        buyer.require_auth();

        let mut items: Map<u32, AgriItem> =
            env.storage().instance().get(&DataKey::Items).unwrap();
        let mut orders: Map<u32, AgriOrder> =
            env.storage().instance().get(&DataKey::Orders).unwrap();

        let mut item = items.get(item_id).expect("Item not found");

        if item.quantity < quantity as u64 {
            panic!("Not enough stock");
        }

        let total_price = item.price * quantity as u64;

        item.quantity -= quantity as u64;
        items.set(item_id, item);

        let order = AgriOrder {
            id: order_id as u64,
            item_id: item_id as u64,
            buyer,
            total_price,
        };

        orders.set(order_id, order);

        env.storage().instance().set(&DataKey::Items, &items);
        env.storage().instance().set(&DataKey::Orders, &orders);
    }

    pub fn get_item(env: Env, id: u32) -> AgriItem {
        env.storage().instance()
            .get::<_, Map<u32, AgriItem>>(&DataKey::Items)
            .unwrap()
            .get(id)
            .unwrap()
    }

    pub fn get_order(env: Env, id: u32) -> AgriOrder {
        env.storage().instance()
            .get::<_, Map<u32, AgriOrder>>(&DataKey::Orders)
            .unwrap()
            .get(id)
            .unwrap()
    }
}
