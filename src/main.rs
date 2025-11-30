use std::collections::HashMap;

use crate::{bo::Character, dto::CharacterDTO, engine::Subject};

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
    let strength = bo.abilities.get_mut("strength").unwrap();
    strength.register_observer(1, observer);
    strength.register_modifier(1, 1, |value| value + 1);
    println!("strength: {:?}", strength.value);
    strength.notify_observers();
    let po_aft: CharacterDTO = bo.into();
    println!("{:?}", po_aft);
}

fn observer(value: &u8) {
    println!("I got it: {:?}", value);
}
