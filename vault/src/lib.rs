#![no_std]

use pinocchio::{
    account_info::AccountInfo, entrypoint, nostd_panic_handler, program_error::ProgramError,
    pubkey::Pubkey, ProgramResult,
};

entrypoint!(process_instruction);
nostd_panic_handler!();

pub mod instructions;
pub use instructions::*;

// Program ID
pub const ID: Pubkey = [
    0x1f, 0x72, 0x5d, 0xe0, 0x3e, 0xce, 0x52, 0xfa, 0x1d, 0x13, 0xd7, 0x63, 0xd4, 0xab, 0x49, 0x6d,
    0xd7, 0x62, 0x1e, 0x40, 0xa6, 0x0e, 0x00, 0xac, 0x88, 0x2d, 0x40, 0x1e, 0x6d, 0x40, 0x7a, 0x21,
];

fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    match instruction_data.split_first() {
        Some((Deposit::DISCRIMINATOR, data)) => Deposit::try_from((data, accounts))?.process(),
        Some((Withdraw::DISCRIMINATOR, _)) => Withdraw::try_from(accounts)?.process(),
        _ => Err(ProgramError::InvalidInstructionData),
    }
}
