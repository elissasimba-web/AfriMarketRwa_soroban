#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short,
    Env, Address, Map, Symbol, Vec, Val, IntoVal,
};

// ================= STORAGE KEYS =================
const ITEMS_KEY: Symbol = symbol_short!("ITEMS");
const ESCROW_CONTRACT_KEY: Symbol = symbol_short!("ESCROW");
const XLM_TOKEN_KEY: Symbol = symbol_short!("XLM");

// ================= ESCROW METHODS =================
const DEPOSIT: Symbol = symbol_short!("deposit");
const SETTLE: Symbol = symbol_short!("settle_rl");

// ================= LISTING TYPE =================
#[contracttype]
#[derive(Clone, PartialEq)]
pub enum ListingType {
    Rent,
    Sale,
}

// ================= RENTAL ITEM =================
#[contracttype]
#[derive(Clone)]
pub struct RentalItem {
    pub id: u32,
    pub name: Symbol,
    pub price_per_day: i128,
    pub price: i128,
    pub owner: Address,
    pub listing_type: ListingType,
    pub rented: bool,
    pub renter: Option<Address>,
    pub start_time: u64,
    pub end_time: u64,
    pub deposit: i128,
    pub sold: bool,
}

#[contract]
pub struct RentalContract;

#[contractimpl]
impl RentalContract {

    // ============== INIT =================
    pub fn init(env: Env, escrow: Address, xlm_token: Address) {
        env.storage().instance().set(&ESCROW_CONTRACT_KEY, &escrow);
        env.storage().instance().set(&XLM_TOKEN_KEY, &xlm_token);
        env.storage()
            .instance()
            .set(&ITEMS_KEY, &Map::<u32, RentalItem>::new(&env));
    }

    // ============== ADD RENT ITEM =================
    pub fn add_item(
        env: Env,
        owner: Address,
        id: u32,
        name: Symbol,
        price_per_day: i128,
    ) {
        owner.require_auth();

        let mut items: Map<u32, RentalItem> =
            env.storage().instance().get(&ITEMS_KEY).unwrap();

        let item = RentalItem {
            id,
            name,
            price_per_day,
            price: 0,
            owner,
            listing_type: ListingType::Rent,
            rented: false,
            renter: None,
            start_time: 0,
            end_time: 0,
            deposit: 0,
            sold: false,
        };

        items.set(id, item);
        env.storage().instance().set(&ITEMS_KEY, &items);
    }

    // ============== RENT ITEM =================
    pub fn rent_item(
        env: Env,
        id: u32,
        renter: Address,
        days: u64,
        deposit: i128,
    ) {
        renter.require_auth();

        let mut items: Map<u32, RentalItem> =
            env.storage().instance().get(&ITEMS_KEY).unwrap();

        let mut item = items.get(id).expect("Item not found");

        if item.rented {
            panic!("Item already rented");
        }

        let now = env.ledger().timestamp();
        let escrow: Address = env.storage().instance().get(&ESCROW_CONTRACT_KEY).unwrap();
        let xlm_token: Address = env.storage().instance().get(&XLM_TOKEN_KEY).unwrap();

        // ---- call escrow.deposit(xlm, renter, deposit)
        let mut args = Vec::<Val>::new(&env);
        args.push_back(xlm_token.into_val(&env));
        args.push_back(renter.clone().into_val(&env));
        args.push_back(deposit.into_val(&env));

        env.invoke_contract::<()>(&escrow, &DEPOSIT, args);

        item.rented = true;
        item.renter = Some(renter);
        item.start_time = now;
        item.end_time = now + days * 86400;
        item.deposit = deposit;

        items.set(id, item);
        env.storage().instance().set(&ITEMS_KEY, &items);
    }

    // ============== RETURN ITEM =================
    pub fn return_item(env: Env, id: u32) -> (i128, i128) {
        let mut items: Map<u32, RentalItem> =
            env.storage().instance().get(&ITEMS_KEY).unwrap();

        let mut item = items.get(id).expect("Item not found");

        let refund = item.deposit;
        let penalty: i128 = 0;

        let escrow: Address = env.storage().instance().get(&ESCROW_CONTRACT_KEY).unwrap();
        let xlm_token: Address = env.storage().instance().get(&XLM_TOKEN_KEY).unwrap();

        let renter = item.renter.clone().unwrap();
        let owner = item.owner.clone();

        // ---- call escrow.settle(xlm, renter, owner, refund, penalty)
        let mut args = Vec::<Val>::new(&env);
        args.push_back(xlm_token.into_val(&env));
        args.push_back(renter.into_val(&env));
        args.push_back(owner.into_val(&env));
        args.push_back(refund.into_val(&env));
        args.push_back(penalty.into_val(&env));

        env.invoke_contract::<()>(&escrow, &SETTLE, args);

        item.rented = false;
        item.renter = None;
        item.start_time = 0;
        item.end_time = 0;
        item.deposit = 0;

        items.set(id, item);
        env.storage().instance().set(&ITEMS_KEY, &items);

        (refund, penalty)
    }

    // ============== ADD PROPERTY FOR SALE =================
    pub fn add_property_for_sale(
        env: Env,
        owner: Address,
        id: u32,
        name: Symbol,
        sale_price: i128,
    ) {
        owner.require_auth();

        let mut items: Map<u32, RentalItem> =
            env.storage().instance().get(&ITEMS_KEY).unwrap();

        let item = RentalItem {
            id,
            name,
            price_per_day: 0,
            price: sale_price,
            owner,
            listing_type: ListingType::Sale,
            rented: false,
            renter: None,
            start_time: 0,
            end_time: 0,
            deposit: 0,
            sold: false,
        };

        items.set(id, item);
        env.storage().instance().set(&ITEMS_KEY, &items);
    }

    // ============== BUY PROPERTY =================
    pub fn buy_property(
        env: Env,
        id: u32,
        buyer: Address,
        payment: i128,
    ) {
        buyer.require_auth();

        let mut items: Map<u32, RentalItem> =
            env.storage().instance().get(&ITEMS_KEY).unwrap();

        let mut item = items.get(id).expect("Property not found");

        // 🚫 ntibyemewe kugurisha ikintu gikodeshejwe
        if item.rented {
            panic!("Cannot sell property while rented");
        }

        if item.listing_type != ListingType::Sale {
            panic!("Item is not for sale");
        }

        if item.sold {
            panic!("Property already sold");
        }

        if payment < item.price {
            panic!("Insufficient payment");
        }

        let escrow: Address = env.storage().instance().get(&ESCROW_CONTRACT_KEY).unwrap();
        let xlm_token: Address = env.storage().instance().get(&XLM_TOKEN_KEY).unwrap();
        let owner = item.owner.clone();

        // ---- call escrow.settle(xlm, buyer, owner, 0, payment)
        let mut args = Vec::<Val>::new(&env);
        args.push_back(xlm_token.into_val(&env));
        args.push_back(buyer.clone().into_val(&env));
        args.push_back(owner.into_val(&env));
        args.push_back(0_i128.into_val(&env));
        args.push_back(payment.into_val(&env));

        env.invoke_contract::<()>(&escrow, &SETTLE, args);

        item.owner = buyer;
        item.sold = true;

        items.set(id, item);
        env.storage().instance().set(&ITEMS_KEY, &items);
    }
}
