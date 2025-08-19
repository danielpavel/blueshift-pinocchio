use bs58;
use core::str;
use pinocchio::pubkey::Pubkey;
use pinocchio_log::log;

pub fn log_pubkey(label: &str, pubkey: &Pubkey) {
    let mut buffer = [0u8; 44];
    let encoded_len = bs58::encode(pubkey.as_ref()).onto(&mut buffer[..]).unwrap();
    let pubkey_str = core::str::from_utf8(&buffer[..encoded_len]).unwrap();

    log!("{}: {}", label, pubkey_str);
}
