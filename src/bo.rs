use std::collections::HashMap;

use crate::engine::Attribute;
use crate::engine::util::restore;
use uuid::Uuid;

use crate::dto::CharacterDTO;

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
    pub class: u8,
}

impl From<CharacterDTO> for Character {
    fn from(value: CharacterDTO) -> Self {
        let uuid = Uuid::new_v4();
        Self {
            id: uuid.to_string(),
            name: value.name,
            level: value.level,
            xp: value.xp,
            level_hp: value.level_hp,
            age: value.age,
            abilities: restore(&value.abilities),
            skills: restore(&value.skills),
            immunities: Attribute::new(value.immunities),
            class: value.class,
        }
    }
}
