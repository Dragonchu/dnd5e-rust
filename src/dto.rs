use std::collections::HashMap;

use crate::engine::util::convert;
use crate::{bo::Character, engine::Subject};

#[derive(Debug)]
pub struct CharacterDTO {
    pub name: String,
    pub level: u8,
    pub xp: u16,
    pub level_hp: [u8; 20],
    pub age: u8,
    pub abilities: HashMap<String, u8>,
    pub skills: HashMap<String, u8>,
    pub immunities: Vec<String>,
    pub class: u8,
}

impl CharacterDTO {
    pub fn new(
        name: String,
        level: u8,
        xp: u16,
        level_hp: [u8; 20],
        age: u8,
        abilities: HashMap<String, u8>,
        skills: HashMap<String, u8>,
        immunities: Vec<String>,
        class: u8,
    ) -> Self {
        Self {
            name,
            level,
            xp,
            level_hp,
            age,
            abilities,
            skills,
            immunities,
            class,
        }
    }
}

impl From<Character> for CharacterDTO {
    fn from(value: Character) -> Self {
        Self {
            name: value.name,
            level: value.level,
            xp: value.xp,
            level_hp: value.level_hp,
            age: value.age,
            abilities: convert(&value.abilities),
            skills: convert(&value.skills),
            immunities: value.immunities.get_value().clone(),
            class: value.class,
        }
    }
}
