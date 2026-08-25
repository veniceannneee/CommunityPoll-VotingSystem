#![no_std]
use soroban_sdk::{
    contract, contractimpl, symbol_short, Address, Env, Symbol, Vec,
};

// Storage Keys
const ADMIN: Symbol = symbol_short!("ADMIN");
const IS_INIT: Symbol = symbol_short!("IS_INIT");
const TOPIC: Symbol = symbol_short!("TOPIC");
const EXPIRATION: Symbol = symbol_short!("EXPIR");

// Data structure key for user votes
#[derive(Clone)]
pub enum DataKey {
    Voted(Address),
    OptionVotes(u32),
}

#[contract]
pub struct CommunityPollContract;

#[contractimpl]
impl CommunityPollContract {
    /// Initialize a new poll with an admin, a topic string, and a duration in seconds.
    pub fn initialize(env: Env, admin: Address, topic: Symbol, duration_seconds: u64) {
        // Ensure initialize is only called once
        if env.storage().instance().has(&IS_INIT) {
            panic!("Poll is already initialized");
        }

        admin.require_auth();

        let expiration = env.ledger().timestamp() + duration_seconds;

        env.storage().instance().set(&ADMIN, &admin);
        env.storage().instance().set(&TOPIC, &topic);
        env.storage().instance().set(&EXPIRATION, &expiration);
        env.storage().instance().set(&IS_INIT, &true);
    }

    /// Cast a vote for a specific option ID (e.g., 0, 1, 2)
    pub fn vote(env: Env, voter: Address, option_id: u32) {
        voter.require_auth();

        // 1. Check if poll is active
        let expiration: u64 = env.storage().instance().get(&EXPIRATION).unwrap();
        if env.ledger().timestamp() >= expiration {
            panic!("Poll has ended");
        }

        // 2. Prevent double-voting
        let voter_key = DataKey::Voted(voter.clone());
        if env.storage().persistent().has(&voter_key) {
            panic!("Address has already voted");
        }

        // 3. Increment vote tally for option_id
        let vote_key = DataKey::OptionVotes(option_id);
        let current_votes: u32 = env.storage().persistent().get(&vote_key).unwrap_or(0);
        
        env.storage().persistent().set(&vote_key, &(current_votes + 1));
        
        // 4. Mark voter as having voted
        env.storage().persistent().set(&voter_key, &true);
    }

    /// Read current vote count for a given option
    pub fn get_votes(env: Env, option_id: u32) -> u32 {
        let vote_key = DataKey::OptionVotes(option_id);
        env.storage().persistent().get(&vote_key).unwrap_or(0)
    }

    /// Check if a user has already voted
    pub fn has_voted(env: Env, voter: Address) -> bool {
        let voter_key = DataKey::Voted(voter);
        env.storage().persistent().has(&voter_key)
    }
}