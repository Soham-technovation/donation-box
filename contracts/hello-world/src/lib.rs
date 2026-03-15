#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype,
    symbol_short, Address, Env, Symbol, Vec,
};

// ── Storage key types ────────────────────────────────────────────────────────

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Admin,
    TotalDonated,
    DonorAmount(Address),
    DonorList,
    BoxOpen,
    Withdrawn,
}

// ── Events ───────────────────────────────────────────────────────────────────

const DONATED:   Symbol = symbol_short!("DONATED");
const WITHDRAWN: Symbol = symbol_short!("WITHDRAWN");
const OPENED:    Symbol = symbol_short!("OPENED");
const CLOSED:    Symbol = symbol_short!("CLOSED");

// ── Contract ─────────────────────────────────────────────────────────────────

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {

    // ── Initialise ───────────────────────────────────────────────────────────

    /// Deploy and set the admin (beneficiary) who will receive donations.
    pub fn init(env: Env, admin: Address) {
        if env.storage().instance().has(&DataKey::Admin) {
            panic!("already initialised");
        }
        admin.require_auth();
        env.storage().instance().set(&DataKey::Admin,        &admin);
        env.storage().instance().set(&DataKey::BoxOpen,      &true);
        env.storage().instance().set(&DataKey::TotalDonated, &0_i128);
        env.storage().instance().set(&DataKey::Withdrawn,    &0_i128);
    }

    // ── Donate ───────────────────────────────────────────────────────────────

    /// Donate `amount` to the box. Anyone can donate while the box is open.
    pub fn donate(env: Env, donor: Address, amount: i128) {
        donor.require_auth();
        assert!(amount > 0, "donation must be positive");
        assert!(Self::is_open(env.clone()), "donation box is closed");

        // Accumulate per-donor total
        let donor_key = DataKey::DonorAmount(donor.clone());
        let prev: i128 = env.storage().persistent().get(&donor_key).unwrap_or(0);
        env.storage().persistent().set(&donor_key, &(prev + amount));

        // Track donor list (skip duplicates)
        let mut donors: Vec<Address> = env.storage()
            .persistent()
            .get(&DataKey::DonorList)
            .unwrap_or(Vec::new(&env));
        if !donors.contains(&donor) {
            donors.push_back(donor.clone());
            env.storage().persistent().set(&DataKey::DonorList, &donors);
        }

        // Update global total
        let total: i128 = env.storage().instance().get(&DataKey::TotalDonated).unwrap_or(0);
        env.storage().instance().set(&DataKey::TotalDonated, &(total + amount));

        env.events().publish((DONATED, donor), amount);
    }

    // ── Admin: withdraw ──────────────────────────────────────────────────────

    /// Admin withdraws all collected donations. Returns the amount withdrawn.
    pub fn withdraw(env: Env) -> i128 {
        let admin: Address = env.storage()
            .instance()
            .get(&DataKey::Admin)
            .expect("not initialised");
        admin.require_auth();

        let total: i128   = env.storage().instance().get(&DataKey::TotalDonated).unwrap_or(0);
        let already: i128 = env.storage().instance().get(&DataKey::Withdrawn).unwrap_or(0);
        let available     = total - already;

        assert!(available > 0, "nothing to withdraw");

        env.storage().instance().set(&DataKey::Withdrawn, &total);
        env.events().publish((WITHDRAWN, admin), available);

        available
    }

    // ── Admin: open / close ──────────────────────────────────────────────────

    /// Close the donation box — no new donations accepted.
    pub fn close_box(env: Env) {
        Self::require_admin(&env);
        env.storage().instance().set(&DataKey::BoxOpen, &false);
        env.events().publish((CLOSED,), ());
    }

    /// Re-open the donation box.
    pub fn open_box(env: Env) {
        Self::require_admin(&env);
        env.storage().instance().set(&DataKey::BoxOpen, &true);
        env.events().publish((OPENED,), ());
    }

    // ── Read-only queries ────────────────────────────────────────────────────

    /// Total amount donated since deployment.
    pub fn total_donated(env: Env) -> i128 {
        env.storage().instance().get(&DataKey::TotalDonated).unwrap_or(0)
    }

    /// Total amount already withdrawn by the admin.
    pub fn total_withdrawn(env: Env) -> i128 {
        env.storage().instance().get(&DataKey::Withdrawn).unwrap_or(0)
    }

    /// Funds currently sitting in the box (not yet withdrawn).
    pub fn available(env: Env) -> i128 {
        let total: i128     = env.storage().instance().get(&DataKey::TotalDonated).unwrap_or(0);
        let withdrawn: i128 = env.storage().instance().get(&DataKey::Withdrawn).unwrap_or(0);
        total - withdrawn
    }

    /// Amount donated by a specific address.
    pub fn donor_amount(env: Env, donor: Address) -> i128 {
        env.storage()
            .persistent()
            .get(&DataKey::DonorAmount(donor))
            .unwrap_or(0)
    }

    /// Full list of unique donor addresses.
    pub fn donors(env: Env) -> Vec<Address> {
        env.storage()
            .persistent()
            .get(&DataKey::DonorList)
            .unwrap_or(Vec::new(&env))
    }

    /// Number of unique donors.
    pub fn donor_count(env: Env) -> u32 {
        let donors: Vec<Address> = env.storage()
            .persistent()
            .get(&DataKey::DonorList)
            .unwrap_or(Vec::new(&env));
        donors.len()
    }

    /// Whether the donation box is currently open.
    pub fn is_open(env: Env) -> bool {
        env.storage().instance().get(&DataKey::BoxOpen).unwrap_or(false)
    }

    /// Current admin / beneficiary address.
    pub fn admin(env: Env) -> Address {
        env.storage()
            .instance()
            .get(&DataKey::Admin)
            .expect("not initialised")
    }

    /// Transfer admin role to a new address.
    pub fn set_admin(env: Env, new_admin: Address) {
        Self::require_admin(&env);
        env.storage().instance().set(&DataKey::Admin, &new_admin);
    }

    // ── Internal helpers ─────────────────────────────────────────────────────

    fn require_admin(env: &Env) {
        let admin: Address = env.storage()
            .instance()
            .get(&DataKey::Admin)
            .expect("not initialised");
        admin.require_auth();
    }
}

mod test;
