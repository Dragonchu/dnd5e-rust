use std::collections::HashMap;

use crate::engine::Attribute;
use crate::engine::util::restore;
use uuid::Uuid;

use crate::dto::CharacterDTO;

pub enum Class {
    Barbarian = 1,
}

impl TryFrom<u8> for Class {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Class::Barbarian),
            _ => Err("Invalid integer value for class"),
        }
    }
}

pub struct Character {
    pub id: String,
    pub name: String,
    pub level: u8,
    pub xp: u16,
    pub level_hp: [u8; 20],
    pub age: u8,
    pub abilities: HashMap<String, Attribute<u8>>,
    pub skills: HashMap<String, Attribute<u8>>,
    pub immunities: Attribute<Vec<String>>,
    pub class: Class,
}

impl From<CharacterDTO> for Character {
    fn from(value: CharacterDTO) -> Self {
        let uuid = Uuid::new_v4();
        let mut character = Character {
            id: uuid.to_string(),
            name: value.name,
            level: value.level,
            xp: value.xp,
            level_hp: value.level_hp,
            age: value.age,
            abilities: restore(&value.abilities),
            skills: restore(&value.skills),
            immunities: Attribute::new(value.immunities),
            class: value.class.try_into().unwrap(),
        };

        character
    }
}
