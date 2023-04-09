use std::path::Prefix;
use std::str::FromStr;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::env::attached_deposit;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{near_bindgen, env, Promise, AccountId};


#[derive(Serialize, Deserialize, BorshSerialize, BorshDeserialize)]
#[serde(crate="near_sdk::serde")] // para serializar algo que venga originalmente en formato json

pub struct Meme {
    pub id: u64,
    pub creado_por: AccountId,
    pub titulo: String,
    pub museo: String,
    pub url: String,
    pub donaciones: u128
}

// implementación del trait Default para iniciar la structura
impl Default for Meme {
    fn default() -> Self {
        Meme { id: 0,
            creado_por: "".parse().unwrap(), //String::from("")
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
        Self {  id: env::block_height(),
                creado_por: env::signer_account_id(),
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
    memes: UnorderedMap<u64, Meme>, // esta estructura requiere dos valores la clave y el valor

}

// inicialización del state del contrato
impl Default for SimpleMemeMuseum {
    fn default() -> Self {
        Self {  
            // inicialización de colecciones
            museos: UnorderedMap::new(b"u".to_vec()),
            memes: UnorderedMap::new(b"u".to_vec()),
        }
    }
    
}

// metodos del contrato
#[near_bindgen]
impl SimpleMemeMuseum {
    // función publica que crea el meme
    pub fn crear_meme(&mut self, titulo: String, url: String, nombre_museo: String) {
        // crear el objeto del meme
        let meme = Meme::new(
            String::from(&titulo),
            String::from(&url),
            String::from(&nombre_museo),
        );
    
    // se guarda en la colección de memes
    self.memes.insert(&meme.id, &meme);
    
    // buscamos si ne museo existe para depues añaadir el meme en él, el valor de un keydado como Option<Vec>64>>
    let museo = self.museos.get(&nombre_museo);

    // si existe el museo is_some() retornará true y agragamos el nuevo id
    if museo.is_some() {
        // m alamacena el valor de Some() del Option<> o da un panic en caso de ser Null
        let mut m = museo.unwrap();
        // m al ser un vector hacemos push para alamacenar el dato
        m.push(meme.id);     
        // incertamos la colección de museos le museo y el meme ligaod a él
        self.museos.insert(&nombre_museo, &m);
    // si no existe, creamos un museo nuevo    
    } else {
        let mut nuevo_museo = Vec::new();

        nuevo_museo.push(meme.id);
        self.museos.insert(&nombre_museo, &nuevo_museo); // incerta los atributos del objeto
    }

    // manda un mensaje a la terminal al ejecutar este metódo
    env::log(
        format!(
            "Nuevo muso añadido con éxito, Museo: {}, Id Meme: {}",
            &nombre_museo, meme.id
        )
        .as_bytes(),
        )
    }

    // metodo para obtener el meme
    pub fn obtener_meme(&self, id: u64) -> Option<Meme> {
        self.memes.get(&id)
    }

    // metodo de solo lectura retorna un vector con la colexión de memes
    pub fn obtener_lista_memes(&self) -> Vec<(u64, Meme)> {
        self.memes.to_vec()
    }

    // retorna la lista de museos, toma los key de la colección y lo convierte a vector
    pub fn obtener_lista_mueseos(&self) -> Vec<String> {
        self.museos.keys_as_vector().to_vec()
    }

    // retorna un vector con los memes del museo
    pub fn obtener_memes_museo(&self, nombre_museo: String) -> Vec<Meme>{
        // se obtiene el museo como un Option<Vec<64>>
        let museo = self.museos.get(&nombre_museo);

        // si el museo existe
        if museo.is_some() {
            // creamos un vector que almacene la lista de memes museo
            let mut lista_memes = Vec::new();

            // el for recorre cada elemento del museo
            for meme in &museo.unwrap() {
                // obtenemos el meme mediante su id
                let m = self.memes.get(meme);

                // si el meme existe
                if m.is_some() {
                    // guardamos el meme al vector de la lista memes
                    lista_memes.push(m.unwrap());
                }
            }
            // retornamos la lista de memes
            lista_memes
        // si no existe el museo
        } else {
            // creamos un vector vacío para memes
            Vec::new()
        }
    }

    #[payable]
    pub fn donar_a_meme(&mut self, id: u64) -> bool {
        assert!(
            env::attached_deposit() > 0,
            "Debes agragar NEAR para hacer una donación"
        );
        // buscamos el meme
        match self.memes.get(&id) { // match es como un swicht
            Some(mut meme) => {
                // si existe guardamos la donación del registro
                meme.donaciones += env::attached_deposit();
                self.memes.insert(&meme.id, &meme);
    
                // y le trasferimos al creador del meme la donación
                Promise::new(meme.creado_por).transfer(env::attached_deposit());
    
                true
            }
            None => false,
        }

    }
}