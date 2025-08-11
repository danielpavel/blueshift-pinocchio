use std::collections::HashMap;

use mollusk_svm::{
    account_store::AccountStore,
    program::keyed_account_for_system_program,
    result::{Check, ContextResult},
    Mollusk, MolluskContext,
};
use solana_account::Account;
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    native_token::LAMPORTS_PER_SOL,
    pubkey::Pubkey,
};

// const ID: Pubkey = solana_sdk::pubkey!("37knY9pFnowzNYwKiPhDPWJwid633Zzd2YsDhoiMKLUg");
const ID: [u8; 32] = [
    0x1f, 0x72, 0x5d, 0xe0, 0x3e, 0xce, 0x52, 0xfa, 0x1d, 0x13, 0xd7, 0x63, 0xd4, 0xab, 0x49, 0x6d,
    0xd7, 0x62, 0x1e, 0x40, 0xa6, 0x0e, 0x00, 0xac, 0x88, 0x2d, 0x40, 0x1e, 0x6d, 0x40, 0x7a, 0x21,
];

const PROGRAM_ID: &Pubkey = &Pubkey::new_from_array(ID);

// Simple in-memory account store implementation
#[derive(Default)]
struct InMemoryAccountStore {
    accounts: HashMap<Pubkey, Account>,
}

impl AccountStore for InMemoryAccountStore {
    fn get_account(&self, pubkey: &Pubkey) -> Option<Account> {
        self.accounts.get(pubkey).cloned()
    }

    fn store_account(&mut self, pubkey: Pubkey, account: Account) {
        self.accounts.insert(pubkey, account);
    }
}

#[test]
fn test_deposit() {
    let mollusk = Mollusk::new(PROGRAM_ID, "./target/deploy/vault");

    let (system_program, system_program_account) = keyed_account_for_system_program();

    let starting_lamports = 10 * LAMPORTS_PER_SOL;

    let payer = Pubkey::new_unique();
    let payer_account = Account::new(starting_lamports, 0, &system_program);

    let (vault_pubkey, _) = Pubkey::find_program_address(&[b"vault", payer.as_ref()], PROGRAM_ID);

    let accounts = [
        (payer, payer_account),
        (vault_pubkey, Account::default()), // vault does not yet exist - no account
        (system_program, system_program_account),
    ];

    let mut store = InMemoryAccountStore::default();
    for (pubkey, account) in &accounts {
        store.store_account(pubkey.clone(), account.clone());
    }
    let context = mollusk.with_context(store);

    let amount = LAMPORTS_PER_SOL;

    let deposit_checks = &[
        Check::success(),
        Check::account(&payer)
            .lamports(starting_lamports - amount) // Payer pays
            .build(),
        Check::account(&vault_pubkey)
            .lamports(amount) // Vault receives
            .build(),
    ];

    let account_pubkeys = &accounts.map(|a| a.0);
    let _deposit_result = deposit(&context, account_pubkeys, amount, deposit_checks);

    let withdraw_checks = &[
        Check::success(),
        Check::account(&payer)
            .lamports(starting_lamports) // Payer receives
            .build(),
        Check::account(&vault_pubkey)
            .lamports(0) // Vault pays
            .build(),
    ];

    // Test withdrraw
    let _withdraw_result = withdraw(&context, account_pubkeys, withdraw_checks);
}

/*
 * Utils
 */
fn deposit(
    context: &MolluskContext<InMemoryAccountStore>,
    accounts: &[Pubkey],
    amount: u64,
    checks: &[Check],
) -> ContextResult {
    let [payer, vault, system_program] = accounts else {
        panic!("Could not unpack accounts in deposit")
    };

    let mut instruction_data = vec![];
    instruction_data.extend_from_slice(&[0]); // deposit instruction DISCRIMINATOR
    instruction_data.extend_from_slice(amount.to_le_bytes().as_ref());

    let instruction = Instruction::new_with_bytes(
        PROGRAM_ID.into(),
        instruction_data.as_slice(),
        vec![
            AccountMeta::new(payer.into(), true),           // owner
            AccountMeta::new(vault.into(), false),          // vault
            AccountMeta::new(system_program.into(), false), // system_program
        ],
    );

    let result = context.process_and_validate_instruction(&instruction, checks);

    result
}

fn withdraw(
    context: &MolluskContext<InMemoryAccountStore>,
    accounts: &[Pubkey],
    checks: &[Check],
) -> ContextResult {
    let [payer, vault, system_program] = accounts else {
        panic!("Could not unpack accounts in withdraw")
    };

    let mut instruction_data = vec![];
    instruction_data.extend_from_slice(&[1]); // withdraw instruction DISCRIMINATOR

    let instruction = Instruction::new_with_bytes(
        PROGRAM_ID.into(),
        instruction_data.as_slice(),
        vec![
            AccountMeta::new(payer.into(), true),           // owner
            AccountMeta::new(vault.into(), false),          // vault
            AccountMeta::new(system_program.into(), false), // system_program
        ],
    );

    let result = context.process_and_validate_instruction(&instruction, checks);

    result
}
