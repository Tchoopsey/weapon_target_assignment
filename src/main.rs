use weapon_target_assignment::{assignment::Assignment, target::Target, weapon::Weapon};

fn main() {
    let w = Weapon::new(String::from("Tank"));
    println!("{:?}", w);

    let mut target_list: Vec<Target> = Vec::new();
    let t = Target::new(10.0, &target_list);
    t.add_to_target_list(&mut target_list);
    let t = Target::new(30.0, &target_list);
    t.add_to_target_list(&mut target_list);

    // println!("{:?}", t);
    println!("{:#?}", target_list);

    let a = Assignment::new(&w, &target_list.get(0).unwrap());
    println!("{:#?}", a);
    println!("{:#?}", target_list);
}
