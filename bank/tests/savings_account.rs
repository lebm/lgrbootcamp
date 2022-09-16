#[allow(unused_imports)]
use bank::SavingsAccount;

mod utils;

#[test]
fn should_have_a_starting_balance_of_0_() {
    utils::common_setup();
    let account = bank::SavingsAccount::new();
    assert_eq!(account.get_balance(), 0);
}
