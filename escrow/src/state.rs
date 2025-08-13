use core::mem::size_of;
use pinocchio::{program_error::ProgramError, pubkey::Pubkey};

#[repr(C)]
pub struct Escrow {
    pub maker: Pubkey,
    pub token_a_mint: Pubkey,
    pub token_b_mint: Pubkey,
    pub receive: u64,
    pub seed: u64,
    pub bump: [u8; 1],
}

impl Escrow {
    pub const LEN: usize = size_of::<Pubkey>() + // Maker
        size_of::<Pubkey>() +   // token_a_mint
        size_of::<Pubkey>() +   // token_b_mint
        size_of::<u64>() +      // receive
        size_of::<u64>() +      // seed
        size_of::<[u8;1]>(); // bump

    #[inline(always)]
    pub fn load(bytes: &[u8]) -> Result<&Self, ProgramError> {
        if bytes.len() != Escrow::LEN {
            return Err(ProgramError::InvalidAccountData);
        }

        Ok(unsafe { &*core::mem::transmute::<*const u8, *const Self>(bytes.as_ptr()) })
    }

    #[inline(always)]
    pub fn load_mut(bytes: &mut [u8]) -> Result<&mut Self, ProgramError> {
        if bytes.len() != Escrow::LEN {
            return Err(ProgramError::InvalidAccountData);
        }

        Ok(unsafe { &mut *core::mem::transmute::<*mut u8, *mut Self>(bytes.as_mut_ptr()) })
    }

    #[inline(always)]
    pub fn set_maker(&mut self, maker: Pubkey) {
        self.maker = maker;
    }

    #[inline(always)]
    pub fn set_token_a_mint(&mut self, token_a_mint: Pubkey) {
        self.token_a_mint = token_a_mint;
    }

    #[inline(always)]
    pub fn set_token_b_mint(&mut self, token_b_mint: Pubkey) {
        self.token_b_mint = token_b_mint;
    }

    #[inline(always)]
    pub fn set_seed(&mut self, seed: u64) {
        self.seed = seed;
    }

    #[inline(always)]
    pub fn set_receive(&mut self, receive: u64) {
        self.receive = receive;
    }

    #[inline(always)]
    pub fn set_bump(&mut self, bump: [u8; 1]) {
        self.bump = bump;
    }

    pub fn set_inner(
        &mut self,
        maker: Pubkey,
        token_a_mint: Pubkey,
        token_b_mint: Pubkey,
        seed: u64,
        receive: u64,
        bump: [u8; 1],
    ) {
        self.maker = maker;
        self.token_a_mint = token_a_mint;
        self.token_b_mint = token_b_mint;
        self.receive = receive;
        self.seed = seed;
        self.bump = bump;
    }
}
