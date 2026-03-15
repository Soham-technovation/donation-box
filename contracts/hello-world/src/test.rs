#![cfg(test)]

use super::*;
use soroban_sdk::{Address, Env, testutils};
use soroban_sdk::testutils::Address as _;

#[test]
fn test_donation_box() {
    let env = Env::default();
    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);

    // Create test addresses
    let admin = Address::generate(&env);
    let donor1 = Address::generate(&env);
    let donor2 = Address::generate(&env);

    // Mock all auths for testing
    env.mock_all_auths();

    // Initialize the contract
    client.init(&admin);

    // Check initial state
    assert_eq!(client.total_donated(), 0);
    assert_eq!(client.available(), 0);
    assert_eq!(client.is_open(), true);
    assert_eq!(client.admin(), admin.clone());

    // Donate
    client.donate(&donor1, &100);

    assert_eq!(client.total_donated(), 100);
    assert_eq!(client.available(), 100);
    assert_eq!(client.donor_amount(&donor1), 100);
    assert_eq!(client.donor_count(), 1);

    // Another donation
    client.donate(&donor2, &50);

    assert_eq!(client.total_donated(), 150);
    assert_eq!(client.available(), 150);
    assert_eq!(client.donor_amount(&donor2), 50);
    assert_eq!(client.donor_count(), 2);

    // Withdraw as admin
    let withdrawn = client.withdraw();
    assert_eq!(withdrawn, 150);
    assert_eq!(client.available(), 0);
    assert_eq!(client.total_withdrawn(), 150);
}
