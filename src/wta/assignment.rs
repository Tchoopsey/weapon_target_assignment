use std::{collections::HashMap, io};

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
        weapon_list: &HashMap<Weapon, u8>,
        target_list: &Vec<Target>,
    ) -> Vec<(Weapon, Target, f32)> {
        let mut ctd_list: Vec<(Weapon, Target, f32)> = Vec::new();
        for weapon in weapon_list.keys() {
            for target in target_list {
                let mut rng = rand::thread_rng();
                let ctd: f32 = rng.gen_range(0.0..1.0);
                ctd_list.push((weapon.clone(), target.clone(), ctd));
            }
        }
        ctd_list
    }

    pub fn get_chance_to_destroy(
        ctd: &Vec<(Weapon, Target, f32)>,
        weapon: &Weapon,
        target: &Target,
    ) -> f32 {
        let chance = ctd.iter().find(|(w, t, _c)| w == weapon && t == target);
        chance.unwrap().2
    }

    pub fn wta(
        t: Target,
        target_list: &mut Vec<Target>,
        w: Weapon,
        ctd: &Vec<(Weapon, Target, f32)>,
    ) -> Assignment {
        let chance = Assignment::get_chance_to_destroy(ctd, &w, &t);
        let mut t = t;
        t.value *= chance;
        Target::replace_target(target_list, t.clone());
        Assignment::new(w, Target::get_target(t.get_name(), target_list).unwrap())
    }

    pub fn calculate_remains(target_list: &Vec<Target>) -> f32 {
        let mut sum = 0.0;
        for t in target_list {
            sum += t.value;
        }
        sum
    }

    pub fn manual_assign(weapon_list: &mut HashMap<Weapon, u8>, target_list: &mut Vec<Target>) {
        let mut total_weapons = Weapon::total_weapons(weapon_list);
        let chance_to_destroy = Assignment::generate_chance_to_destroy(weapon_list, target_list);
        println!(
            "\n\nChance each weapon has to destroy each target:\n{:#?}",
            chance_to_destroy
        );

        let mut assignment_list: Vec<Assignment> = Vec::new();

        while total_weapons != 0 {
            println!(
                "\n\nChance each weapon has to destroy each target:\n{:#?}",
                chance_to_destroy
            );
            println!("\nList of available weapons:\n{:?}", weapon_list);
            println!("\nList of available targets:\n{:?}", target_list);

            let mut weapon = String::new();
            io::stdin()
                .read_line(&mut weapon)
                .expect("Unable to read line");
            let w: Weapon =
                Weapon::get_weapon(weapon.strip_suffix("\n").unwrap().to_string(), weapon_list)
                    .unwrap();

            let mut target = String::new();
            io::stdin()
                .read_line(&mut target)
                .expect("Unable to read line");
            let t: Target =
                Target::get_target(target.strip_suffix("\n").unwrap().to_string(), target_list)
                    .unwrap();

            total_weapons -= 1;

            assignment_list.push(Assignment::wta(t, target_list, w, &chance_to_destroy));
        }
        println!("{:#?}", assignment_list);
        println!("Remaining sum of targets values:");
        println!("{}", Assignment::calculate_remains(target_list));
    }
}
