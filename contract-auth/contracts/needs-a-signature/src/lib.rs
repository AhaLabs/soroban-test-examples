#![no_std]
use soroban_sdk::{contract, contractimpl, vec, Address, Env, String, Vec};

#[contract]
pub struct SignedContract;

#[contractimpl]
impl SignedContract {
    pub fn hello(env: Env, to: String, sign_with: Address) -> Vec<String> {
        sign_with.require_auth();
        vec![&env, String::from_str(&env, "Hello"), to]
    }
}

mod test;
