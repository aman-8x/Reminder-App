#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, Env, Map, String};

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    // Set reminder
    pub fn set_reminder(env: Env, reminder_id: String, message: String) {
        let key = symbol_short!("reminders");
        let mut reminders_map: Map<String, String> = env
            .storage()
            .instance()
            .get(&key)
            .unwrap_or(Map::new(&env));
        
        reminders_map.set(reminder_id.clone(), message);
        env.storage().instance().set(&key, &reminders_map);
    }

    // Get reminder
    pub fn get_reminder(env: Env, reminder_id: String) -> String {
        let key = symbol_short!("reminders");
        let reminders_map: Map<String, String> = env
            .storage()
            .instance()
            .get(&key)
            .unwrap_or(Map::new(&env));
        
        reminders_map.get(reminder_id).unwrap_or(String::from_str(&env, ""))
    }
}
