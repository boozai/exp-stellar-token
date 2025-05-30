#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, Address, Env, String};

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn symbol(env: Env) -> String {
        String::from_str(&env, "ABC")
    }

    pub fn mint(env: Env, to: Address, amount: i128) {
        let mut to_bal: i128 = env.storage().persistent().get(&to).unwrap_or_default();
        to_bal += amount;
        env.storage().persistent().set(&to, &to_bal);
        env.events().publish((symbol_short!("mint"), &to), amount);
    }

    pub fn transfer(env: Env, from: Address, to: Address, amount: i128) {
        let mut from_bal: i128 = env.storage().persistent().get(&from).unwrap_or_default();
        let mut to_bal: i128 = env.storage().persistent().get(&to).unwrap_or_default();
        from_bal -= amount;
        to_bal += amount;
        env.storage().persistent().set(&from, &from_bal);
        env.storage().persistent().set(&to, &to_bal);
        env.events().publish((symbol_short!("transfer"), &from, &to), amount);
    }
}
