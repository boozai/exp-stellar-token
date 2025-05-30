#![no_std]
use soroban_sdk::{contract, contractimpl, contractmeta, symbol_short, Address, Env};

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn mint(env: Env, to: Address, amount: i128) {
        env.events().publish((symbol_short!("mint"), &to), amount);
    }
}
