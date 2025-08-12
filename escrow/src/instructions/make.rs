use pinocchio::{
    account_info::AccountInfo, program_error::ProgramError, pubkey::find_program_address,
    ProgramResult,
};

use super::{
    AccountCheck, AssociatedTokenAccount, AssociatedTokenAccountCheck, MintAccount, SignerAccount,
};

/*
 * =============================
 * Accounts Context
 * =============================
 */
pub struct MakeAccounts<'a> {
    pub maker: &'a AccountInfo,
    pub escrow: &'a AccountInfo,
    pub a_mint: &'a AccountInfo,
    pub b_mint: &'a AccountInfo,
    pub maker_ata: &'a AccountInfo,
    pub escrow_ata: &'a AccountInfo, // vault
    pub seed: u64,
    pub system_program: &'a AccountInfo,
    pub token_program: &'a AccountInfo,
}

impl<'a> TryFrom<(&'a [AccountInfo], u64)> for MakeAccounts<'a> {
    type Error = ProgramError;

    fn try_from((accounts, seed): (&'a [AccountInfo], u64)) -> Result<Self, Self::Error> {
        let [maker, escrow, a_mint, b_mint, maker_ata, escrow_ata, system_program, token_program, associated_token_program] =
            accounts
        else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };

        SignerAccount::check(maker)?;
        MintAccount::check(a_mint)?;
        MintAccount::check(b_mint)?;
        AssociatedTokenAccount::check(maker_ata, a_mint, maker, token_program)?;

        Ok(Self {
            maker,
            escrow,
            a_mint,
            b_mint,
            maker_ata,
            escrow_ata,
            seed,
            system_program,
            token_program,
        })
    }
}

/*
 * ==========================
 * Instruction Data Context
 * ==========================
 */
pub struct MakeInstructionData {
    amount: u64,
    receive: u64,
    seed: u64,
}

impl<'a> TryFrom<&'a [u8]> for MakeInstructionData {
    type Error = ProgramError;

    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        if data.len() != size_of::<u64>() * 3 {
            return Err(ProgramError::InvalidInstructionData);
        }

        let amount = u64::from_le_bytes(data[0..7].try_into().unwrap());
        let receive = u64::from_le_bytes(data[8..15].try_into().unwrap());
        let seed = u64::from_le_bytes(data[16..23].try_into().unwrap());

        if amount.eq(&0) {
            return Err(ProgramError::InvalidInstructionData);
        }

        Ok(Self {
            amount,
            receive,
            seed,
        })
    }
}

/*
 * ==========================
 * Instruction
 * ==========================
 */
pub struct Make<'a> {
    pub accounts: MakeAccounts<'a>,
    pub instruction_data: MakeInstructionData,
}

impl<'a> TryFrom<(&'a [u8], &'a [AccountInfo])> for Make<'a> {
    type Error = ProgramError;

    fn try_from((data, accounts): (&'a [u8], &'a [AccountInfo])) -> Result<Self, Self::Error> {
        let instruction_data = MakeInstructionData::try_from(data)?;
        let accounts = MakeAccounts::try_from((accounts, instruction_data.seed))?;

        Ok(Self {
            accounts,
            instruction_data,
        })
    }
}

impl<'a> Make<'a> {
    pub const DISCRIMINATOR: &'a u8 = &0;

    pub fn process(&mut self) -> ProgramResult {
        //TODO: Implement process

        Ok(())
    }
}
