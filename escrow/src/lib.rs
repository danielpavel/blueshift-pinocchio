#![no_std]

use instructions::Make;
use pinocchio::{
    account_info::AccountInfo, entrypoint, nostd_panic_handler, program_error::ProgramError,
    pubkey::Pubkey, ProgramResult,
};

entrypoint!(process_instruction);
nostd_panic_handler!();

pub mod instructions;
pub mod state;
pub mod utils;

// Program ID
// Hss68bDHZjSwXberD14diFDYXDiTBrSYdkUjED3vz31R
pub const ID: Pubkey = [
    0xfa, 0xc2, 0xaa, 0x9e, 0x82, 0x8c, 0xca, 0x97, 0x94, 0xac, 0x6a, 0x0b, 0x94, 0x33, 0x15, 0xa0,
    0xf6, 0xc9, 0xab, 0xcf, 0x21, 0x53, 0x12, 0xdc, 0x4b, 0x56, 0x5a, 0xd2, 0xfe, 0xd2, 0xc4, 0x58,
];

fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    match instruction_data.split_first() {
        Some((Make::DISCRIMINATOR, data)) => Make::try_from((data, accounts))?.process(),
        _ => Err(ProgramError::InvalidInstructionData),
    }
}
