use std::path::Prefix;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{near_bindgen, setup_alloc};

setup_alloc!(); // paa saber la dirección de WASM en memoria

#[derive(Serialize, Deserialize, BorshSerialize, BorshDeserialize)]
#[serde(crate="near_sdk::serde")] // para serializar algo que venga originalmente en formato json

pub struct Meme {
    pub id: u64,
    pub creado_por: String,
    pub titulo: String,
    pub museo: String,
    pub url: String,
    pub donaciones: u128
}

// implementación del trait Default para iniciar la structura
impl Default for Meme {
    fn default() -> Self {
        Meme { id: 0,
            creado_por: String::from(""), // string vacío
            titulo: String::from(""), // string vacío
            museo: String::from(""), // string vacío
            url: String::from(""), // string vacío
            donaciones: 0,
        }
    }
}
// implementación del metódo new para crear nuevos memes
impl Meme {
    pub fn new(titulo: String, url: String, museo: String) -> Self {
        Self { id: 0,
            creado_por: String::from(""),
            titulo,
            museo,
            url,
            donaciones: 0,
        }
    }
    
}

// estructura, atate del contrato e inicialización de valores
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct SimpleMemeMuseum {
    // se guardan solo los ID para evitar tener que editar en ambos lugares
    museos: UnorderedMap<String, Vec<u64>>, // esta estructura requiere dos valores la clave y el valor
    nemes: UnorderedMap<u64, Meme>, // esta estructura requiere dos valores la clave y el valor

}

// inicialización del state del contrato
impl Default for SimpleMemeMuseum {
    fn default() -> Self {
        Self {  
            // inicialización de colecciones
            museos: UnorderedMap::new(prefix: b"u".to_vect()),
            memes: UnorderedMap::new(prefix: b"u".to_vect()),
        }
    }
    
}