use mollusk_svm::{
    program::keyed_account_for_system_program,
    result::{Check, InstructionResult},
    Mollusk,
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

    let amount = LAMPORTS_PER_SOL;
    let deposit_result = deposit(&mollusk, &accounts, amount);

    // Deposit checks
    deposit_result.run_checks(
        &[
            Check::account(&payer)
                .lamports(starting_lamports - amount) // Payer pays
                .build(),
            Check::account(&vault_pubkey)
                .lamports(amount) // Vault receives
                .build(),
        ],
        &mollusk.config,
        &mollusk,
    );

    // Test withdrraw
    let withdraw_result = withdraw(&mollusk, &accounts);

    // withdraw checks
    withdraw_result.run_checks(
        &[
            Check::account(&payer)
                .lamports(starting_lamports) // Payer receives
                .build(),
            Check::account(&vault_pubkey)
                .lamports(0) // Vault pays
                .build(),
        ],
        &mollusk.config,
        &mollusk,
    );
}

/*
 * Utils
 */
fn deposit(mollusk: &Mollusk, accounts: &[(Pubkey, Account)], amount: u64) -> InstructionResult {
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
            AccountMeta::new(payer.0, true),           // owner
            AccountMeta::new(vault.0, false),          // vault
            AccountMeta::new(system_program.0, false), // system_program
        ],
    );

    let result =
        mollusk.process_and_validate_instruction(&instruction, &accounts, &[Check::success()]);

    result
}

fn withdraw(mollusk: &Mollusk, accounts: &[(Pubkey, Account)]) -> InstructionResult {
    let [payer, vault, system_program] = accounts else {
        panic!("Could not unpack accounts in withdraw")
    };

    let mut instruction_data = vec![];
    instruction_data.extend_from_slice(&[1]); // withdraw instruction DISCRIMINATOR

    let instruction = Instruction::new_with_bytes(
        PROGRAM_ID.into(),
        instruction_data.as_slice(),
        vec![
            AccountMeta::new(payer.0, true),           // owner
            AccountMeta::new(vault.0, false),          // vault
            AccountMeta::new(system_program.0, false), // system_program
        ],
    );

    let result =
        mollusk.process_and_validate_instruction(&instruction, &accounts, &[Check::success()]);

    result
}
