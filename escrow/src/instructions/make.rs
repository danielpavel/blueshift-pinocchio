use pinocchio::{
    account_info::AccountInfo, instruction::Seed, program_error::ProgramError,
    pubkey::find_program_address, ProgramResult,
};
use pinocchio_token::instructions::Transfer;

use crate::state::Escrow;

use super::{
    AccountCheck, AssociateTokenAccountInit, AssociatedTokenAccount, AssociatedTokenAccountCheck,
    MintAccount, ProgramAccount, ProgramAccountInit, SignerAccount,
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
    pub system_program: &'a AccountInfo,
    pub token_program: &'a AccountInfo,
}

impl<'a> TryFrom<&'a [AccountInfo]> for MakeAccounts<'a> {
    type Error = ProgramError;

    fn try_from(accounts: &'a [AccountInfo]) -> Result<Self, Self::Error> {
        let [maker, escrow, a_mint, b_mint, maker_ata, escrow_ata, system_program, token_program] =
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
    pub bump: [u8; 1],
}

impl<'a> TryFrom<(&'a [u8], &'a [AccountInfo])> for Make<'a> {
    type Error = ProgramError;

    fn try_from((data, accounts): (&'a [u8], &'a [AccountInfo])) -> Result<Self, Self::Error> {
        let instruction_data = MakeInstructionData::try_from(data)?;
        let accounts = MakeAccounts::try_from(accounts)?;

        let (_, bump) = find_program_address(
            &[
                b"escrow",
                accounts.maker.key(),
                &instruction_data.seed.to_le_bytes(),
            ],
            &crate::ID,
        );

        let seed_binding = instruction_data.seed.to_le_bytes();
        let bump_binding = [bump];

        let seeds = [
            Seed::from(b"escrow"),
            Seed::from(accounts.maker.key().as_ref()),
            Seed::from(&seed_binding),
            Seed::from(&bump_binding),
        ];

        ProgramAccount::init(accounts.maker, accounts.escrow, &seeds, Escrow::LEN)?;

        AssociatedTokenAccount::init(
            accounts.maker,
            accounts.escrow_ata,
            accounts.a_mint,
            accounts.system_program,
            accounts.token_program,
        )?;

        Ok(Self {
            accounts,
            instruction_data,
            bump: bump_binding,
        })
    }
}

impl<'a> Make<'a> {
    pub const DISCRIMINATOR: &'a u8 = &0;

    pub fn process(&mut self) -> ProgramResult {
        let mut data = self.accounts.escrow.try_borrow_mut_data()?;
        let escrow = Escrow::load_mut(data.as_mut())?;

        escrow.set_inner(
            *self.accounts.maker.key(),
            *self.accounts.a_mint.key(),
            *self.accounts.b_mint.key(),
            self.instruction_data.seed,
            self.instruction_data.receive,
            self.bump,
        );

        Transfer {
            from: self.accounts.maker_ata,
            to: self.accounts.escrow_ata,
            authority: self.accounts.maker,
            amount: self.instruction_data.amount,
        }
        .invoke()?;

        Ok(())
    }
}
