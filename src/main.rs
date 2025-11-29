use std::collections::{HashMap, HashSet};

use crate::{
    bo::Character,
    dto::CharacterDTO,
    engine::{Attribute, Subject},
};

pub mod bo;
pub mod dto;
pub mod engine;

fn main() {
    let po = CharacterDTO::new(
        "john".to_string(),
        1,
        1,
        [0; 20],
        0,
        HashMap::from([("strength".to_string(), 0 as u8)]),
        HashMap::new(),
        Vec::new(),
        1,
    );
    let mut bo: Character = po.into();
    let strength = bo.abilities.get("strength").unwrap();
    println!("strength: {:?}", strength.value);
    let po_aft: CharacterDTO = bo.into();
    println!("{:?}", po_aft);
}
