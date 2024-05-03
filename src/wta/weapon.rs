use std::{collections::HashMap, fmt::Display};
use rand::seq::IteratorRandom;

#[derive(Debug, Eq, Clone, Hash)]
pub struct Weapon {
    weapon_type: String,
}

#[allow(dead_code)]
impl Weapon {
    /// Create a new Weapon with a weapon_type
    pub fn new(weapon_type: String) -> Weapon {
        Weapon { weapon_type }
    }

    pub fn add_to_weapon_list(weapon: Weapon, weapon_list: &mut HashMap<Weapon, u8>) {
        let count = weapon_list.entry(weapon).or_insert(0);
        *count += 1;
    }

    /// Fetches a weapon from the weapon_list and removes a counter of that weapon
    /// When the counter reaches 0, weapon gets removed from the list
    pub fn get_weapon(
        weapon_type: String,
        weapon_list: &mut HashMap<Weapon, u8>,
    ) -> Option<Weapon> {
        let weapon = weapon_list
            .keys()
            .find(|&w| w.weapon_type == weapon_type)?
            .clone();

        if let Some(val) = weapon_list.get_mut(&weapon) {
            if *val > 0 {
                *val -= 1;
            }
            if *val == 0 {
                weapon_list.remove(&weapon);
            }
        }
        Some(weapon)
    }

    pub fn get_random_weapon(weapon_list: &mut HashMap<Weapon, u8>) -> Option<Weapon> {
        let weapon = weapon_list.keys().choose(&mut rand::thread_rng()).unwrap().clone();

        if let Some(val) = weapon_list.get_mut(&weapon) {
            if *val > 0 {
                *val -= 1;
            }
            if *val == 0 {
                weapon_list.remove(&weapon);
            }
        }
        Some(weapon)
    }

    pub fn total_weapons(weapon_list: &mut HashMap<Weapon, u8>) -> u8 {
        weapon_list.values().sum()
    }
}

impl Display for Weapon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Weapon: {}", self.weapon_type)
    }
}

impl PartialEq for Weapon {
    fn eq(&self, other: &Self) -> bool {
        self.weapon_type == other.weapon_type
    }
}
