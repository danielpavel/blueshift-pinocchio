use mollusk_svm::Mollusk;
use solana_account::Account;
use solana_pubkey::Pubkey;
use solana_sdk::program_pack::Pack;
use spl_associated_token_account::get_associated_token_address_with_program_id;
use spl_token::state::{Account as TokenAccount, AccountState, Mint};

pub fn keyed_account_for_mint_default(
    mollusk: &Mollusk,
    mint_authority: &Pubkey,
    pubkey: Option<Pubkey>,
    token_program: Option<Pubkey>,
    decimals: u8,
) -> (Pubkey, Account) {
    let mint_data = Mint {
        mint_authority: Some(*mint_authority).into(),
        supply: 0,
        decimals,
        is_initialized: true,
        freeze_authority: None.into(),
    };

    let mut data = vec![0u8; Mint::LEN];
    Mint::pack(mint_data, data.as_mut_slice()).unwrap();

    let account = Account {
        lamports: mollusk.sysvars.rent.minimum_balance(Mint::LEN),
        data,
        owner: token_program.unwrap_or(spl_token::ID),
        executable: false,
        rent_epoch: 0,
    };

    (pubkey.unwrap_or(Pubkey::default()), account)
}

pub fn keyed_account_for_token_account_default(
    mollusk: &Mollusk,
    pubkey: Option<Pubkey>,
    mint: &Pubkey,
    owner: &Pubkey,
    amount: u64,
    token_program: Option<Pubkey>,
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

    let mut data = vec![0u8; TokenAccount::LEN];
    TokenAccount::pack(token_account_data, data.as_mut_slice()).unwrap();

    let account = Account {
        lamports: mollusk.sysvars.rent.minimum_balance(Mint::LEN),
        data,
        owner: token_program.unwrap_or(spl_token::ID),
        executable: false,
        rent_epoch: 0,
    };

    (pubkey.unwrap_or(Pubkey::default()), account)
}

pub fn keyed_account_for_associated_token_account(
    mollusk: &Mollusk,
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

    keyed_account_for_token_account_default(
        mollusk,
        Some(pubkey),
        mint,
        owner,
        amount,
        token_program,
    )
}
