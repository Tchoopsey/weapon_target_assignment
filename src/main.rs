use weapon_target_assignment::{assignment::Assignment, target::Target, weapon::Weapon};

fn main() {
    let mut weapon_list: Vec<Weapon> = Vec::new();
    Weapon::add_to_weapon_list(Weapon::new(String::from("Tank")), &mut weapon_list);
    Weapon::add_to_weapon_list(Weapon::new(String::from("Tank")), &mut weapon_list);
    Weapon::add_to_weapon_list(Weapon::new(String::from("Plane")), &mut weapon_list);
    println!("\nList of available weapons:\n{:#?}", weapon_list);

    let mut target_list: Vec<Target> = Vec::new();
    Target::add_to_target_list(Target::new(10.0, &mut target_list), &mut target_list);
    Target::add_to_target_list(Target::new(30.0, &mut target_list), &mut target_list);
    Target::add_to_target_list(Target::new(40.0, &mut target_list), &mut target_list);
    println!("\nList of available targets:\n{:#?}", target_list);

    // let ctd = Assignment::generate_chance_to_destroy(
    //     &Weapon::get_weapon("Plane".to_string(), &mut weapon_list).unwrap(),
    //     &Target::get_target("T1".to_string(), &mut target_list).unwrap()
    // );

    let a = Assignment::new(
        Weapon::get_weapon("Plane".to_string(), &mut weapon_list).unwrap(),
        Target::get_target("T1".to_string(), &mut target_list).unwrap(),
    );
    println!("{:#?}", a);
    println!("\nList of available weapons:\n{:#?}", weapon_list);
    println!("\nList of available targets:\n{:#?}", target_list);

    let chances_to_destroy = Assignment::generate_chance_to_destroy(&weapon_list, &target_list);
    println!(
        "\n\nChance each weapon has to destroy each target:\n{:#?}",
        chances_to_destroy
    );
}
