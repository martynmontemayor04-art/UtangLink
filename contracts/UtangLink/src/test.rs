#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, Address, Env};

mod tests {

    use super::*;

    #[test]
    fn test_happy_path() {
        let env = Env::default();

        let debtor = Address::generate(&env);
        let creditor = Address::generate(&env);

        let id = UtangLinkContract::create_debt(
            env.clone(),
            debtor.clone(),
            creditor.clone(),
            120,
        );

        UtangLinkContract::repay_debt(
            env.clone(),
            id,
            debtor.clone(),
        );

        let debt = UtangLinkContract::get_debt(env, id);

        assert_eq!(debt.paid, true);
    }

    #[test]
    #[should_panic(expected = "Unauthorized debtor")]
    fn test_wrong_debtor() {
        let env = Env::default();

        let debtor = Address::generate(&env);
        let fake = Address::generate(&env);
        let creditor = Address::generate(&env);

        let id = UtangLinkContract::create_debt(
            env.clone(),
            debtor,
            creditor,
            200,
        );

        UtangLinkContract::repay_debt(env, id, fake);
    }

    #[test]
    fn test_state_verification() {
        let env = Env::default();

        let debtor = Address::generate(&env);
        let creditor = Address::generate(&env);

        let id = UtangLinkContract::create_debt(
            env.clone(),
            debtor.clone(),
            creditor.clone(),
            500,
        );

        let debt = UtangLinkContract::get_debt(env, id);

        assert_eq!(debt.amount, 500);
        assert_eq!(debt.paid, false);
    }

    #[test]
    #[should_panic(expected = "Debt already paid")]
    fn test_double_payment() {
        let env = Env::default();

        let debtor = Address::generate(&env);
        let creditor = Address::generate(&env);

        let id = UtangLinkContract::create_debt(
            env.clone(),
            debtor.clone(),
            creditor,
            300,
        );

        UtangLinkContract::repay_debt(
            env.clone(),
            id,
            debtor.clone(),
        );

        UtangLinkContract::repay_debt(
            env,
            id,
            debtor,
        );
    }

    #[test]
    fn test_multiple_debts() {
        let env = Env::default();

        let debtor = Address::generate(&env);
        let creditor = Address::generate(&env);

        let id1 = UtangLinkContract::create_debt(
            env.clone(),
            debtor.clone(),
            creditor.clone(),
            100,
        );

        let id2 = UtangLinkContract::create_debt(
            env.clone(),
            debtor,
            creditor,
            250,
        );

        assert_eq!(id1, 1);
        assert_eq!(id2, 2);
    }
}