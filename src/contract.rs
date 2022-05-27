use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    near_bindgen, BorshStorageKey, PanicOnDefault,
};

#[derive(BorshStorageKey, BorshSerialize)]
enum StorageKey {
    Item,
}

#[near_bindgen]
#[derive(PanicOnDefault, BorshDeserialize, BorshSerialize)]
pub struct Contract {
    pub counter: u32,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new() -> Self {
        Self {
            counter: 0,
        }
    }

    pub fn increment_counter(mut self) {
        self.counter +=1;
    }

    pub fn decrement_counter(mut self) {
        self.counter -=1;
    }

    pub fn get_counter(self) -> u32 {
        return self.counter;
    }
}
