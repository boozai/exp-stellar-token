#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, Address, Env, String};

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn mint(env: Env, to: Address, amount: i128) {
        env.events().publish((symbol_short!("mint"), &to), amount);
    }

    pub fn symbol(env: Env) -> String {
        String::from_str(&env, "ABC")
    }

    pub fn transfer(env: Env, from: Address, to: Address, amount: i128) {
        env.events().publish((symbol_short!("transfer"), &from, &to), amount);
    }
}
