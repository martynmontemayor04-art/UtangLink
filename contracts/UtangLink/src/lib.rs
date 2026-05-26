#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short,
    Address, Env, Symbol, Vec,
};

#[contracttype]
#[derive(Clone)]
pub struct Debt {
    pub debtor: Address,
    pub creditor: Address,
    pub amount: i128,
    pub paid: bool,
}

#[contracttype]
pub enum DataKey {
    Debt(u64),
    Counter,
}

#[contract]
pub struct UtangLinkContract;

#[contractimpl]
impl UtangLinkContract {

    // Create a new debt entry
    pub fn create_debt(
        env: Env,
        debtor: Address,
        creditor: Address,
        amount: i128,
    ) -> u64 {

        creditor.require_auth();

        let mut counter: u64 = env
            .storage()
            .instance()
            .get(&DataKey::Counter)
            .unwrap_or(0);

        counter += 1;

        let debt = Debt {
            debtor,
            creditor,
            amount,
            paid: false,
        };

        env.storage()
            .instance()
            .set(&DataKey::Debt(counter), &debt);

        env.storage()
            .instance()
            .set(&DataKey::Counter, &counter);

        counter
    }

    // Repay an existing debt
    pub fn repay_debt(env: Env, debt_id: u64, debtor: Address) {

        debtor.require_auth();

        let mut debt: Debt = env
            .storage()
            .instance()
            .get(&DataKey::Debt(debt_id))
            .unwrap();

        if debt.paid {
            panic!("Debt already paid");
        }

        if debt.debtor != debtor {
            panic!("Unauthorized debtor");
        }

        debt.paid = true;

        env.storage()
            .instance()
            .set(&DataKey::Debt(debt_id), &debt);
    }

    // Fetch debt details
    pub fn get_debt(env: Env, debt_id: u64) -> Debt {
        env.storage()
            .instance()
            .get(&DataKey::Debt(debt_id))
            .unwrap()
    }
}