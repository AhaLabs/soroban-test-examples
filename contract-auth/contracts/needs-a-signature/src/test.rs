#![cfg(test)]

use super::*;
use soroban_sdk::{vec, Env, String};
use this_one_signs::SigningContract;

#[test]
fn test() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, SignedContract);
    let signing_contract_id = env.register_contract(None, SigningContract);
    let client = SignedContractClient::new(&env, &contract_id);

    let words = client.hello(&String::from_str(&env, "Dev"), &signing_contract_id);
    assert_eq!(
        words,
        vec![
            &env,
            String::from_str(&env, "Hello"),
            String::from_str(&env, "Dev"),
        ]
    );
}
