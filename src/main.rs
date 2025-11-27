use std::collections::{HashMap, HashSet};

use crate::{bo::Character, po::CharacterPO};

pub mod bo;
pub mod po;

fn main() {
    let po = CharacterPO::new("john".to_string(), 1, 1, [0;20], 0, HashMap::new(), HashMap::new(), Vec::new(), 1);
    let bo: Character = po.into();
    println!("{:?}", bo);
}
