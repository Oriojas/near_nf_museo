use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{near_bindgen, setup_alloc};

setup_alloc!();

// estructura, atate del contrato e inicialización de valores
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct SimpleMemeMuseum {

}

// inicialización del state del contrato
impl Default for SimpleMemeMuseum {
    fn default() -> Self {
        Self {  
            
        }
    }
    
}