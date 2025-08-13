use pinocchio::{
    account_info::AccountInfo,
    instruction::{Seed, Signer},
    program_error::ProgramError,
    pubkey::find_program_address,
    sysvars::{rent::Rent, Sysvar},
    ProgramResult,
};
use pinocchio_associated_token_account::instructions::Create;
use pinocchio_system::instructions::CreateAccount;
use pinocchio_token::instructions::{InitializeAccount3, InitializeMint2};

pub trait AccountCheck {
    fn check(account: &AccountInfo) -> Result<(), ProgramError>;
}

pub struct SignerAccount;

impl AccountCheck for SignerAccount {
    fn check(account: &AccountInfo) -> Result<(), ProgramError> {
        if !account.is_signer() {
            return Err(ProgramError::MissingRequiredSignature);
        }

        Ok(())
    }
}

pub struct MintAccount;

impl AccountCheck for MintAccount {
    fn check(account: &AccountInfo) -> Result<(), ProgramError> {
        if !account.is_owned_by(&pinocchio_token::ID) {
            return Err(ProgramError::InvalidAccountOwner);
        }

        if account.data_len().ne(&pinocchio_token::state::Mint::LEN) {
            return Err(ProgramError::InvalidAccountData);
        }

        Ok(())
    }
}

pub trait MintInit {
    fn init(
        account: &AccountInfo,
        payer: &AccountInfo,
        decimals: u8,
        mint_authority: &[u8; 32],
        freeze_authority: Option<&[u8; 32]>,
    ) -> Result<(), ProgramError>;
    fn init_if_needed(
        account: &AccountInfo,
        payer: &AccountInfo,
        decimals: u8,
        mint_authority: &[u8; 32],
        freeze_authority: Option<&[u8; 32]>,
    ) -> Result<(), ProgramError>;
}

impl MintInit for MintAccount {
    fn init(
        account: &AccountInfo,
        payer: &AccountInfo,
        decimals: u8,
        mint_authority: &[u8; 32],
        freeze_authority: Option<&[u8; 32]>,
    ) -> Result<(), ProgramError> {
        // Get required lamports for rent
        let rent = Rent::get()?.minimum_balance(pinocchio_token::state::Mint::LEN);

        // Fund the account with required Lamports
        CreateAccount {
            from: payer,
            to: account,
            lamports: rent,
            space: pinocchio_token::state::Mint::LEN as u64,
            owner: &pinocchio_token::ID,
        }
        .invoke()?;

        InitializeMint2 {
            mint: account,
            decimals,
            mint_authority,
            freeze_authority,
        }
        .invoke()
    }

    fn init_if_needed(
        account: &AccountInfo,
        payer: &AccountInfo,
        decimals: u8,
        mint_authority: &[u8; 32],
        freeze_authority: Option<&[u8; 32]>,
    ) -> Result<(), ProgramError> {
        match Self::check(account) {
            Ok(_) => Ok(()),
            Err(_) => Self::init(account, payer, decimals, mint_authority, freeze_authority),
        }
    }
}

pub struct TokenAccount;

impl AccountCheck for TokenAccount {
    fn check(account: &AccountInfo) -> Result<(), ProgramError> {
        if !account.is_owned_by(&pinocchio_token::ID) {
            return Err(ProgramError::InvalidAccountOwner);
        }

        if account
            .data_len()
            .ne(&pinocchio_token::state::TokenAccount::LEN)
        {
            return Err(ProgramError::InvalidAccountData);
        }

        Ok(())
    }
}

pub trait TokenAccountInit {
    fn init(
        account: &AccountInfo,
        mint: &AccountInfo,
        payer: &AccountInfo,
        owner: &[u8; 32],
    ) -> Result<(), ProgramError>;
    fn init_if_needed(
        account: &AccountInfo,
        mint: &AccountInfo,
        payer: &AccountInfo,
        owner: &[u8; 32],
    ) -> Result<(), ProgramError>;
}

impl TokenAccountInit for TokenAccount {
    fn init(
        account: &AccountInfo,
        mint: &AccountInfo,
        payer: &AccountInfo,
        owner: &[u8; 32],
    ) -> Result<(), ProgramError> {
        let rent = Rent::get()?.minimum_balance(pinocchio_token::state::TokenAccount::LEN);

        CreateAccount {
            from: payer,
            to: account,
            lamports: rent,
            space: pinocchio_token::state::TokenAccount::LEN as u64,
            owner: &pinocchio_token::ID,
        }
        .invoke()?;

        InitializeAccount3 {
            account,
            mint,
            owner,
        }
        .invoke()
    }

    fn init_if_needed(
        account: &AccountInfo,
        mint: &AccountInfo,
        payer: &AccountInfo,
        owner: &[u8; 32],
    ) -> Result<(), ProgramError> {
        match Self::check(account) {
            Ok(_) => Ok(()),
            Err(_) => Self::init(account, mint, payer, owner),
        }
    }
}

pub struct AssociatedTokenAccount;

pub trait AssociatedTokenAccountCheck {
    fn check(
        account: &AccountInfo,
        mint: &AccountInfo,
        authority: &AccountInfo,
        token_program: &AccountInfo,
    ) -> Result<(), ProgramError>;
}

impl AssociatedTokenAccountCheck for AssociatedTokenAccount {
    fn check(
        account: &AccountInfo,
        mint: &AccountInfo,
        authority: &AccountInfo,
        token_program: &AccountInfo,
    ) -> Result<(), ProgramError> {
        TokenAccount::check(account)?;

        let (ata, _) = find_program_address(
            &[authority.key(), token_program.key(), mint.key()],
            &pinocchio_associated_token_account::ID,
        );

        if ata.ne(account.key()) {
            return Err(ProgramError::InvalidAccountData);
        }

        Ok(())
    }
}

pub trait AssociateTokenAccountInit {
    fn init(
        payer: &AccountInfo,
        account: &AccountInfo,
        mint: &AccountInfo,
        system_program: &AccountInfo,
        token_program: &AccountInfo,
    ) -> ProgramResult;
    fn init_if_needed(
        payer: &AccountInfo,
        account: &AccountInfo,
        mint: &AccountInfo,
        system_program: &AccountInfo,
        token_program: &AccountInfo,
    ) -> ProgramResult;
}

impl AssociateTokenAccountInit for AssociatedTokenAccount {
    fn init(
        payer: &AccountInfo,
        account: &AccountInfo,
        mint: &AccountInfo,
        system_program: &AccountInfo,
        token_program: &AccountInfo,
    ) -> ProgramResult {
        Create {
            funding_account: payer,
            account,
            mint,
            wallet: payer,
            system_program,
            token_program,
        }
        .invoke()
    }

    fn init_if_needed(
        payer: &AccountInfo,
        account: &AccountInfo,
        mint: &AccountInfo,
        system_program: &AccountInfo,
        token_program: &AccountInfo,
    ) -> ProgramResult {
        match Self::check(account, mint, payer, token_program) {
            Ok(_) => Ok(()),
            Err(_) => Self::init(payer, account, mint, system_program, token_program),
        }
    }
}

pub struct ProgramAccount;

impl AccountCheck for ProgramAccount {
    fn check(account: &AccountInfo) -> Result<(), ProgramError> {
        if !account.is_owned_by(&crate::ID) {
            return Err(ProgramError::InvalidAccountOwner);
        }

        if !account.data_len().ne(&crate::state::Escrow::LEN) {
            return Err(ProgramError::InvalidAccountData);
        }

        Ok(())
    }
}

pub trait ProgramAccountInit {
    fn init<'a>(
        payer: &AccountInfo,
        account: &AccountInfo,
        seeds: &[Seed<'a>],
        space: usize,
    ) -> ProgramResult;
}

impl ProgramAccountInit for ProgramAccount {
    fn init<'a>(
        payer: &AccountInfo,
        account: &AccountInfo,
        seeds: &[Seed<'a>],
        space: usize,
    ) -> ProgramResult {
        let lamports = Rent::get()?.minimum_balance(space);

        let signer = [Signer::from(seeds)];
        CreateAccount {
            from: payer,
            to: account,
            lamports,
            space: space as u64,
            owner: &crate::ID,
        }
        .invoke_signed(&signer)
    }
}

pub trait AccountClose {
    fn close(account: &AccountInfo, destination: &AccountInfo) -> ProgramResult;
}

impl AccountClose for ProgramAccount {
    fn close(account: &AccountInfo, destination: &AccountInfo) -> ProgramResult {
        {
            let mut data = account.try_borrow_mut_data()?;
            data[0] = 0xff;
        }

        *destination.try_borrow_mut_lamports()? += *account.try_borrow_mut_lamports()?;
        account.resize(1)?;
        account.close()?;

        Ok(())
    }
}
