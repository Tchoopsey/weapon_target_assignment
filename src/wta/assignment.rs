use rand::Rng;

use crate::{target::Target, weapon::Weapon};

#[derive(Debug, PartialEq)]
pub struct Assignment {
    weapon: Weapon,
    target: Target,
}

impl Assignment {
    /// Create a new Assignment weapon -> target
    pub fn new(weapon: Weapon, target: Target) -> Assignment {
        let target = target;
        let weapon = weapon;
        Assignment { weapon, target }
    }

    pub fn generate_chance_to_destroy(
        weapon_list: &Vec<Weapon>,
        target_list: &Vec<Target>,
    ) -> Vec<(Weapon, Target, f32)> {
        let mut ctd_list: Vec<(Weapon, Target, f32)> = Vec::new();
        for weapon in weapon_list {
            for target in target_list {
                let mut rng = rand::thread_rng();
                let ctd: f32 = rng.gen_range(0.0..1.0);
                ctd_list.push((weapon.clone(), target.clone(), ctd));
            }
        }
        ctd_list
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_assignment() {}
}
