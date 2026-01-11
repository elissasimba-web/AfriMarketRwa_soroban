#![cfg(test)]

use super::*;
use soroban_sdk::{vec, Env, String};

#[test]
fn test() {
    let env = Env::default();
    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);

    let words = client.hello(&String::from_str(&env, "Dev"));
    assert_eq!(
        words,
        vec![
            &env,
            String::from_str(&env, "Hello"),
            String::from_str(&env, "Dev"),
        ]
    );
}
#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::{Env, String};

    #[test]
    fn test_add_and_get_item() {
        let env = Env::default();

        // Ongeramo item
        RentalContract::add_item(env.clone(), String::from("item1"), String::from("Imodoka"), 100);
        let item = RentalContract::get_item(env.clone(), String::from("item1")).unwrap();

        assert_eq!(item.name, String::from("Imodoka"));
        assert_eq!(item.price_per_day, 100);
        assert!(item.available);
    }

    #[test]
    fn test_rent_and_return_item() {
        let env = Env::default();

        // Ongeramo item
        RentalContract::add_item(env.clone(), String::from("item2"), String::from("Inzu"), 200);

        // Rent item
        let rent_result = RentalContract::rent_item(env.clone(), String::from("item2"));
        assert!(rent_result.is_ok());

        let rented_item = RentalContract::get_item(env.clone(), String::from("item2")).unwrap();
        assert!(!rented_item.available);

        // Return item
        let return_result = RentalContract::return_item(env.clone(), String::from("item2"));
        assert!(return_result.is_ok());

        let returned_item = RentalContract::get_item(env.clone(), String::from("item2")).unwrap();
        assert!(returned_item.available);
    }
}
