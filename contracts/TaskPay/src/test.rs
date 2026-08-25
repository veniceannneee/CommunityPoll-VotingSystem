#![cfg(test)]
use super::*;
use soroban_sdk::{testutils::Address as _, Env, Symbol};

#[test]
fn test_poll_workflow() {
    let env = Env::default();
    let contract_id = env.register_contract(None, CommunityPollContract);
    let client = CommunityPollContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let voter_1 = Address::generate(&env);
    let voter_2 = Address::generate(&env);

    // Mock authentication
    env.mock_all_auths();

    // 1. Initialize Poll (Duration: 3600 seconds)
    client.initialize(&admin, &symbol_short!("Proposal1"), &3600);

    // 2. Cast Votes
    client.vote(&voter_1, &0); // Voter 1 votes for Option 0
    client.vote(&voter_2, &1); // Voter 2 votes for Option 1

    // 3. Assert Results
    assert_eq!(client.get_votes(&0), 1);
    assert_eq!(client.get_votes(&1), 1);
    assert_eq!(client.has_voted(&voter_1), true);
}