use std::collections::HashMap;

use crate::bo::Character;

#[derive(Debug)]
pub struct CharacterPO {
    pub name: String,
    pub level: u8,
    pub xp: u16,
    pub level_hp: [u8; 20],
    pub age: u8,
    pub ability: HashMap<String, u8>,
    pub skills: HashMap<String, u8>,
    pub immunities: Vec<String>,
    pub class: u8,
}

impl CharacterPO {
    pub fn new(
        name: String,
        level: u8,
        xp: u16,
        level_hp: [u8; 20],
        age: u8,
        ability: HashMap<String, u8>,
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
            ability,
            skills,
            immunities,
            class,
        }
    }
}

impl From<Character> for CharacterPO {
    fn from(value: Character) -> Self {
        Self {
            name: value.name,
            level: value.level,
            xp: value.xp,
            level_hp: value.level_hp,
            age: value.age,
            ability: value.ability,
            skills: value.skills,
            immunities: value.immunities,
            class: value.class,
        }
    }
}
