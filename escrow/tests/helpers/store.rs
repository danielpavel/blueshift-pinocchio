use std::collections::HashMap;

use mollusk_svm::account_store::AccountStore;
use solana_account::Account;
use solana_pubkey::Pubkey;

#[derive(Default)]
pub struct InMemoryAccountStore {
    accounts: HashMap<Pubkey, Account>,
}

impl AccountStore for InMemoryAccountStore {
    fn get_account(&self, pubkey: &Pubkey) -> Option<Account> {
        self.accounts.get(pubkey).cloned()
    }

    fn store_account(&mut self, pubkey: Pubkey, account: Account) {
        self.accounts.insert(pubkey, account);
    }
}
