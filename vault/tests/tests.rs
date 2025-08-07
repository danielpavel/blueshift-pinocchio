use mollusk_svm::{program::keyed_account_for_system_program, result::Check, Mollusk};
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

#[test]

fn test_deposit() {
    let program_id = &Pubkey::new_from_array(ID);

    let mollusk = Mollusk::new(program_id, "./target/deploy/vault");

    let (system_program, system_program_data) = keyed_account_for_system_program();

    let payer = Pubkey::new_unique();
    let payer_account = Account::new(10 * LAMPORTS_PER_SOL, 0, &system_program);

    // let vault = find_program_address(&[b"vault", payer], &ID);
    let (vault_pubkey, _) = Pubkey::find_program_address(&[b"vault", payer.as_ref()], program_id);

    let accounts = [
        (payer, payer_account),
        (vault_pubkey, Account::default()), // vault does not yet exist, hence it doesn't have an
        (system_program, system_program_data),
    ];

    let amount = LAMPORTS_PER_SOL;

    let mut instruction_data = vec![];
    instruction_data.extend_from_slice(&[0]); // deposit instruction DISCRIMINATOR
    instruction_data.extend_from_slice(amount.to_le_bytes().as_ref());

    let instruction = Instruction::new_with_bytes(
        program_id.into(),
        instruction_data.as_slice(),
        vec![
            AccountMeta::new(payer, true),           // owner
            AccountMeta::new(vault_pubkey, false),   // vault
            AccountMeta::new(system_program, false), // system_program
        ],
    );

    let _result =
        mollusk.process_and_validate_instruction(&instruction, &accounts, &[Check::success()]);
}
