use std::collections::HashMap;

use weapon_target_assignment::{assignment::Assignment, target::Target, weapon::Weapon};

fn main() {
    let mut weapon_list: HashMap<Weapon, u8> = HashMap::new();
    Weapon::add_to_weapon_list(Weapon::new(String::from("Tank")), &mut weapon_list);
    Weapon::add_to_weapon_list(Weapon::new(String::from("Tank")), &mut weapon_list);
    Weapon::add_to_weapon_list(Weapon::new(String::from("Plane")), &mut weapon_list);

    let mut target_list: Vec<Target> = Vec::new();
    Target::add_to_target_list(Target::new(10.0, &mut target_list), &mut target_list);
    Target::add_to_target_list(Target::new(30.0, &mut target_list), &mut target_list);
    Target::add_to_target_list(Target::new(40.0, &mut target_list), &mut target_list);

    // Assignment::manual_assign(&mut weapon_list, &mut target_list);
    Assignment::random_assign(&mut weapon_list, &mut target_list);
}
