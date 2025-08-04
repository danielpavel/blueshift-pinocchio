use core::mem::size_of;

use pinocchio::{
    account_info::AccountInfo, program_error::ProgramError, pubkey::find_program_address,
    ProgramResult,
};
use pinocchio_system::instructions::Transfer;

/*
 * =============================
 * Accounts Context
 * =============================
 */
pub struct DepositAccounts<'a> {
    pub owner: &'a AccountInfo,
    pub vault: &'a AccountInfo,
}

impl<'a> TryFrom<&'a [AccountInfo]> for DepositAccounts<'a> {
    type Error = ProgramError;

    fn try_from(accounts: &'a [AccountInfo]) -> Result<Self, Self::Error> {
        let [owner, vault, _] = accounts else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };

        // Account Checks
        if !owner.is_signer() {
            return Err(ProgramError::InvalidAccountOwner);
        }

        if !vault.is_owned_by(&pinocchio_system::ID) {
            return Err(ProgramError::InvalidAccountOwner);
        }

        // Must have 0 lamports => ensures fresh deposit.
        if vault.lamports().ne(&0) {
            return Err(ProgramError::InvalidAccountOwner);
        }

        let (vault_key, _) = find_program_address(&[b"vault", owner.key()], &crate::ID);
        if vault_key.ne(vault.key()) {
            return Err(ProgramError::InvalidAccountOwner);
        }

        Ok(Self { owner, vault })
    }
}

/*
 * ==========================
 * Instruction Data Context
 * ==========================
 */
pub struct DepositInstructionData {
    pub amount: u64,
}

impl<'a> TryFrom<&'a [u8]> for DepositInstructionData {
    type Error = ProgramError;

    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        if data.len() != size_of::<u64>() {
            return Err(ProgramError::InvalidInstructionData);
        }

        let amount = u64::from_le_bytes(data.try_into().unwrap());

        //Instruction Checks
        if amount.eq(&0) {
            return Err(ProgramError::InvalidInstructionData);
        }

        Ok(Self { amount })
    }
}

/*
 * ==========================
 * Instruction
 * ==========================
 */
pub struct Deposit<'a> {
    pub accounts: DepositAccounts<'a>,
    pub instruction_data: DepositInstructionData,
}

impl<'a> TryFrom<(&'a [u8], &'a [AccountInfo])> for Deposit<'a> {
    type Error = ProgramError;

    fn try_from((data, accounts): (&'a [u8], &'a [AccountInfo])) -> Result<Self, Self::Error> {
        let accounts = DepositAccounts::try_from(accounts)?;
        let instruction_data = DepositInstructionData::try_from(data)?;

        Ok(Self {
            accounts,
            instruction_data,
        })
    }
}

impl<'a> Deposit<'a> {
    pub const DISCRIMINATOR: &'a u8 = &0;

    pub fn process(&mut self) -> ProgramResult {
        Transfer {
            from: self.accounts.owner,
            to: self.accounts.vault,
            lamports: self.instruction_data.amount,
        }
        .invoke()?;

        Ok(())
    }
}
