use mollusk_svm::{
    account_store::AccountStore,
    program::keyed_account_for_system_program,
    result::{Check, ContextResult},
    Mollusk, MolluskContext,
};
use solana_account::Account;
use solana_pubkey::Pubkey;
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    native_token::LAMPORTS_PER_SOL,
    pubkey,
};

pub mod helpers;
pub use helpers::*;

use std::time::{SystemTime, UNIX_EPOCH};

fn random_u64() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos() as u64
}

pub const PROGRAM_ID: Pubkey = pubkey!("Hss68bDHZjSwXberD14diFDYXDiTBrSYdkUjED3vz31R");

const DECIMALS: u8 = 6;

fn prelude() -> (MolluskContext<InMemoryAccountStore>, Vec<(Pubkey, Account)>) {
    let mollusk = Mollusk::new(&PROGRAM_ID, "./target/deploy/escrow");
    let mut context = mollusk.with_context(InMemoryAccountStore::default());

    // Add the SPL Token Program
    mollusk_svm_programs_token::token::add_program(&mut context.mollusk);

    // Add the Associated Token Program
    mollusk_svm_programs_token::associated_token::add_program(&mut context.mollusk);

    let v = vec![
        mollusk_svm_programs_token::token::keyed_account(),
        mollusk_svm_programs_token::associated_token::keyed_account(),
        keyed_account_for_system_program(),
    ];

    (context, v)
}

#[test]
fn test_make() {
    let prelude = prelude();

    let context = prelude.0;
    let prelude_accounts = prelude.1;

    let [token_program, associated_token_program, system_program] = prelude_accounts.as_slice()
    else {
        panic!("Could not fetch prelude accounts");
    };

    let starting_lamports = 10 * LAMPORTS_PER_SOL;
    let (maker, maker_account) =
        keyed_account_for_system_account_with_lamports(starting_lamports, &system_program.0);

    // Create mint_a, mint_b
    let (mint_a_pubkey, mint_a_account) = keyed_account_for_mint_default(&maker, None, DECIMALS);
    let (mint_b_pubkey, mint_b_account) = keyed_account_for_mint_default(&maker, None, DECIMALS);

    // Create maker_ata for a_mint
    let starting_tokens_amount: u64 = (100 * 10u32.pow(DECIMALS as u32)) as u64;
    let (maker_ata_pubkey, maker_ata_account) = keyed_account_for_associated_token_account(
        &mint_a_pubkey,
        &maker,
        starting_tokens_amount,
        Some(token_program.0),
    );

    let seed = random_u64();
    let escrow_pubkey = Pubkey::find_program_address(
        &[b"escrow", maker.as_ref(), seed.to_le_bytes().as_ref()],
        &PROGRAM_ID,
    );

    // Find Associated Token Account
    let escrow_ata_pubkey = Pubkey::find_program_address(
        &[
            escrow_pubkey.0.as_ref(),
            token_program.0.as_ref(),
            mint_a_pubkey.as_ref(),
        ],
        &associated_token_program.0,
    );

    println!("Associated addr: {}", &escrow_ata_pubkey.0);

    let accounts = [
        (maker, maker_account),
        (escrow_pubkey.0, Account::default()),
        (mint_a_pubkey, mint_a_account),
        (mint_b_pubkey, mint_b_account),
        (maker_ata_pubkey, maker_ata_account),
        (escrow_ata_pubkey.0, Account::default()),
        (system_program.0.clone(), system_program.1.clone()),
        (token_program.0.clone(), token_program.1.clone()),
        (
            associated_token_program.0.clone(),
            associated_token_program.1.clone(),
        ),
    ];

    {
        let mut store = context.account_store.borrow_mut();
        for (pubkey, account) in &accounts {
            store.store_account(pubkey.clone(), account.clone());
        }
    } // store is dropped here, releasing the mutable borrow

    let amount = starting_tokens_amount / 10;
    let receive = 1000 * 10u32.pow(DECIMALS as u32);
    let make_checks = &[Check::success()];

    let account_pubkeys = &accounts.map(|a| a.0);
    let _make_result = make(
        &context,
        account_pubkeys,
        amount,
        receive as u64,
        seed,
        make_checks,
    );
}

fn keyed_account_for_system_account_with_lamports(
    lamports: u64,
    owner: &Pubkey,
) -> (Pubkey, Account) {
    let payer = Pubkey::new_unique();
    let payer_account = Account::new(lamports, 0, owner);

    (payer, payer_account)
}

fn make(
    context: &MolluskContext<InMemoryAccountStore>,
    accounts: &[Pubkey],
    amount: u64,
    receive: u64,
    seed: u64,
    checks: &[Check],
) -> ContextResult {
    let [maker, escrow, a_mint, b_mint, maker_ata, escrow_ata, system_program, token_program, associated_token_program] =
        accounts
    else {
        panic!("Could not unpack accounts in make")
    };

    let mut instruction_data = vec![];
    instruction_data.extend_from_slice(&[0]); // make instruction DISCRIMINATOR
    instruction_data.extend_from_slice(amount.to_le_bytes().as_ref());
    instruction_data.extend_from_slice(receive.to_le_bytes().as_ref());
    instruction_data.extend_from_slice(seed.to_le_bytes().as_ref());

    let instruction = Instruction::new_with_bytes(
        PROGRAM_ID.into(),
        instruction_data.as_slice(),
        vec![
            AccountMeta::new(maker.into(), true),
            AccountMeta::new(escrow.into(), false),
            AccountMeta::new(a_mint.into(), false),
            AccountMeta::new(b_mint.into(), false),
            AccountMeta::new(maker_ata.into(), false),
            AccountMeta::new(escrow_ata.into(), false),
            AccountMeta::new(system_program.into(), false),
            AccountMeta::new(token_program.into(), false),
            AccountMeta::new(associated_token_program.into(), false),
        ],
    );

    let result = context.process_and_validate_instruction(&instruction, checks);

    result
}
