use mollusk_svm_programs_token::token::{
    create_account_for_mint, create_account_for_token_account,
};
use solana_account::Account;
use solana_pubkey::Pubkey;
use spl_associated_token_account::get_associated_token_address_with_program_id;
use spl_token::state::{Account as TokenAccount, AccountState, Mint};

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
