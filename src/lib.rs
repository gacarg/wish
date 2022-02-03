use near_sdk::{AccountId, BorshStorageKey, env, near_bindgen, PanicOnDefault,/*log, setup_alloc, Timestamp*/};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{ UnorderedMap};

//use near_sdk::serde::{Deserialize, Serialize};

pub type GameId = u128;
#[derive(BorshSerialize, BorshStorageKey)]
pub enum StorageKey {
    WishList,//0 bit
}
#[near_bindgen]
#[derive( BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct WishContract {
    wish_list: UnorderedMap<AccountId,String>,
}

#[near_bindgen]
impl WishContract {
    #[init]
  pub fn new() -> Self {
      Self{
          wish_list: UnorderedMap::new(StorageKey::WishList),
      }
  }
  pub fn get_wish(&self,account_id: AccountId) -> String{
      self.wish_list.get(&account_id).unwrap()
  }
  pub fn get_list_wish(&self) -> Vec<(AccountId, String)> {
        let accounts = self.wish_list.keys_as_vector();
        let wishes = self.wish_list.values_as_vector();
        (0..(self.wish_list.len()))
            .map(|index| (accounts.get(index).unwrap(), wishes.get(index).unwrap()))
            .collect()
    }
    #[payable]
    pub fn add_wish(&mut self,wish: String) -> String {
        let tip =  env::attached_deposit();
        let account =env::predecessor_account_id();
        if tip!=0 {
            self.wish_list.insert(&account,&(wish+&" with ".to_string()+&tip.to_string()+&" tips".to_string()))
        }
        else {
           self.wish_list.insert(&account,&wish)
        };
        self.wish_list.get(&account).unwrap()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};
    //use near_sdk::env::input;

    fn get_context(input: Vec<u8>, is_view: bool) -> VMContext {
        VMContext {
            current_account_id: "alice_near".to_string(),
            signer_account_id: "bob_near".to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id: "carol_near".to_string(),
            input,
            block_index: 0,
            block_timestamp: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage: 0,
            attached_deposit: 1,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view,
            output_data_receivers: vec![],
            epoch_height: 0,
        }
    }
#[test]
fn test_new_contract() {
    let context = get_context(vec![],false);
    testing_env!(context);
    let _contract = WishContract::new();
}
#[test]
fn test_save_get() {
    let context = get_context(vec![],false);
    testing_env!(context.clone());
    let mut contract = WishContract::new();
    let add_wish = contract.add_wish("my wish".to_string());
    testing_env!(get_context(vec![],true));
    let get_wish = contract.get_wish("carol_near".to_string());
    assert_eq!(add_wish,get_wish);
    assert_eq!(get_wish,"my wish with 1 tips".to_string());
}
#[test]
fn test_save_get_all(){
        let context = get_context(vec![],false);
        testing_env!(context.clone());
        let mut contract = WishContract::new();
        contract.add_wish("my wish".to_string());
        testing_env!(get_context(vec![],true));
        contract.get_list_wish();
}
}
