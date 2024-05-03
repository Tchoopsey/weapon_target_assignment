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

    /// Randomly generates f32 chance each Weapon has to destroy each Target
    /// Returns a vector that conatins tuple (Weapon, Target, chance to destroy)
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

    /// Returns a chance particular Weapon has to destroy particular Target
    /// from the chance to destroy vector
    pub fn get_chance_to_destroy(
        ctd: &Vec<(Weapon, Target, f32)>,
        weapon: &Weapon,
        target: &Target,
    ) -> Option<f32> {
        ctd.iter().find(|(w, t, _)| w == weapon && t == target)
            .map(|(_, _, c)| *c)
    }


    /// Assigns a Weapon to a Target, fetches a chance to destroy for that pair
    /// Initial chance that target will survive is 100% (1.0); when we substract
    /// chance a Weapon has to destroy a Target, we get Targets real chance to survive,
    /// therefore, 1.0 - chance
    pub fn wta(
        t: Target,
        target_list: &mut Vec<Target>,
        w: Weapon,
        ctd: &Vec<(Weapon, Target, f32)>,
    ) -> Assignment {
        let chance = Assignment::get_chance_to_destroy(ctd, &w, &t).unwrap();
        let mut t = t;
        t.value *= 1.0 - chance;
        Target::replace_target(target_list, t.clone());
        Assignment::new(w, Target::get_target(t.get_name(), target_list).unwrap())
    }

    /// Returns sum of values ("health") all of Targets have after weapon-target
    /// assignment has been completed
    pub fn calculate_remains(target_list: &Vec<Target>) -> f32 {
        let mut sum = 0.0;
        for t in target_list {
            sum += t.value;
        }
        sum
    }

    /// We type in a weapon type and target name, function creates an assignment,
    /// applies weapon-target assignment result to the target value and prints out
    /// the list of created assignments (TODO: should return)
    pub fn manual_assign(weapon_list: &mut HashMap<Weapon, u8>, target_list: &mut Vec<Target>) {
        let mut total_weapons = Weapon::total_weapons(weapon_list);
        let chance_to_destroy = Assignment::generate_chance_to_destroy(weapon_list, target_list);

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
            let w: Weapon;
            if let Some(wp) = Weapon::get_weapon(weapon.strip_suffix("\n").unwrap().to_string(), weapon_list) {
                w = wp;
                println!("{w}");
            } else {
                println!("{} does not exist in weapon list\n", weapon);
                continue;
            }

            let mut target = String::new();
            io::stdin()
                .read_line(&mut target)
                .expect("Unable to read line");
            let t: Target;
            if let Some(tar) = Target::get_target(target.strip_suffix("\n").unwrap().to_string(), target_list) {
                t = tar;
                println!("{t}");
            } else {
                println!("{} does not exist in target list\n", target);
                continue;
            }

            total_weapons -= 1;

            assignment_list.push(Assignment::wta(t, target_list, w, &chance_to_destroy));
        }
        println!("{:#?}", assignment_list);
        println!("Remaining sum of targets values:");
        println!("{}", Assignment::calculate_remains(target_list));
    }

    /// Randomly choses the Weapon from the weapon_list and the Target from the target_list
    /// applies the weapon-target assignment result to the Targets value, creates an assignment
    /// and prints out the list of randomly generated assignments (TODO: return that list)
    pub fn random_assign(weapon_list: &mut HashMap<Weapon, u8>, target_list: &mut Vec<Target>) {
        let mut total_weapons = Weapon::total_weapons(weapon_list);
        let chance_to_destroy = Assignment::generate_chance_to_destroy(weapon_list, target_list);

        let mut assignment_list: Vec<Assignment> = Vec::new();

        while total_weapons != 0 {
            println!(
                "\n\nChance each weapon has to destroy each target:\n{:#?}",
                chance_to_destroy
            );
            println!("\nList of available weapons:\n{:?}", weapon_list);
            println!("\nList of available targets:\n{:?}", target_list);

            let weapon = Weapon::get_random_weapon(weapon_list);

            let target = Target::get_random_target(target_list);

            total_weapons -= 1;

            assignment_list.push(Assignment::wta(target, target_list, weapon, &chance_to_destroy));
        }
        println!("{:#?}", assignment_list);
        println!("Remaining sum of targets values:");
        println!("{}", Assignment::calculate_remains(target_list));
    }
}
