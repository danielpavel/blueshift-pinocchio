use mollusk_svm::program::keyed_account_for_system_program;
use solana_account::Account;
use solana_pubkey::Pubkey;
use solana_sdk::native_token::LAMPORTS_PER_SOL;

pub mod helpers;
pub use helpers::*;

#[test]
fn test_make() {
    // Prelude
    let (system_program, system_program_account) = keyed_account_for_system_program();

    let starting_lamports = 10 * LAMPORTS_PER_SOL;
    let (maker, maker_account) = generate_payer(starting_lamports, &system_program);
}

fn generate_payer(lamports: u64, owner: &Pubkey) -> (Pubkey, Account) {
    let payer = Pubkey::new_unique();
    let payer_account = Account::new(lamports, 0, owner);

    (payer, payer_account)
}
