use std::{collections::HashMap, time::SystemTime};

use uuid::{Timestamp, Uuid};

use crate::po::CharacterPO;

#[derive(Debug)]
pub struct Character {
    pub id: String,
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

impl From<CharacterPO> for Character {
    fn from(value: CharacterPO) -> Self {
        let uuid = Uuid::new_v4();
        Self {
            id: uuid.to_string(),
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
