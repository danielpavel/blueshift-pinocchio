use mollusk_svm::{account_store::AccountStore, result::Check, MolluskContext};
use mollusk_svm_programs_token::token::{
    create_account_for_mint, create_account_for_token_account,
};
// use pinocchio::program_error::ProgramError;
use solana_account::Account;
use solana_pubkey::Pubkey;
use solana_sdk::{program_error::ProgramError, program_pack::Pack};
use spl_associated_token_account::get_associated_token_address_with_program_id;
use spl_token::state::{Account as TokenAccount, AccountState, Mint};

use super::InMemoryAccountStore;

pub fn keyed_account_for_mint_default(
    mint_authority: &Pubkey,
    pubkey: Option<Pubkey>,
    decimals: u8,
) -> (Pubkey, Account) {
    let mint_data = Mint {
        mint_authority: Some(*mint_authority).into(),
        supply: 0,
        decimals,
        is_initialized: true,
        freeze_authority: None.into(),
    };

    let mint_account = create_account_for_mint(mint_data);

    (pubkey.unwrap_or(Pubkey::new_unique()), mint_account)
}

pub fn keyed_account_for_token_account_default(
    pubkey: Option<Pubkey>,
    mint: &Pubkey,
    owner: &Pubkey,
    amount: u64,
) -> (Pubkey, Account) {
    let token_account_data = TokenAccount {
        mint: *mint,
        owner: *owner,
        amount,
        delegate: None.into(),
        state: AccountState::Initialized,
        is_native: None.into(),
        delegated_amount: 0,
        close_authority: None.into(),
    };

    let token_account = create_account_for_token_account(token_account_data);

    (pubkey.unwrap_or(Pubkey::new_unique()), token_account)
}

pub fn keyed_account_for_associated_token_account(
    mint: &Pubkey,
    owner: &Pubkey,
    amount: u64,
    token_program: Option<Pubkey>,
) -> (Pubkey, Account) {
    let pubkey = get_associated_token_address_with_program_id(
        owner,
        mint,
        &token_program.unwrap_or(spl_token::ID),
    );

    keyed_account_for_token_account_default(Some(pubkey), mint, owner, amount)
}

// TODO: Currently broken!
// fn token_amount_check<'a>(
//     context: &MolluskContext<InMemoryAccountStore>,
//     token_account: &Pubkey,
//     expected_amount: &u64,
// ) -> Result<Check, ProgramError> {
//     let store = context.account_store.borrow();
//     if let Some(account) = store.get_account(token_account) {
//         let mut token_account_data = TokenAccount::unpack(&account.data)?;
//
//         token_account_data.amount = *expected_amount;
//
//         let mut expected_data = vec![0u8; TokenAccount::LEN];
//         TokenAccount::pack(token_account_data, &mut expected_data).unwrap();
//
//         Ok(Check::account(token_account)
//             .data(expected_data.as_slice()) // Pass the expected data directly
//             .build())
//     } else {
//         Err(ProgramError::InvalidAccountData)
//     }
// }

pub fn validate_token_amount(
    context: &MolluskContext<InMemoryAccountStore>,
    token_account: &Pubkey,
    expected_amount: u64,
) -> bool {
    let store = context.account_store.borrow();
    if let Some(account) = store.get_account(token_account) {
        if account.data.len() >= 72 {
            let actual_amount = u64::from_le_bytes([
                account.data[64],
                account.data[65],
                account.data[66],
                account.data[67],
                account.data[68],
                account.data[69],
                account.data[70],
                account.data[71],
            ]);
            return actual_amount == expected_amount;
        }
    }
    false
}
