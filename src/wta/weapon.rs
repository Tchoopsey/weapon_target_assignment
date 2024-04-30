#[derive(Debug, PartialEq, Clone)]
pub struct Weapon {
    weapon_type: String,
}

#[allow(dead_code)]
impl Weapon {
    /// Create a new Weapon with a weapon_type
    pub fn new(weapon_type: String) -> Weapon {
        Weapon { weapon_type }
    }

    pub fn add_to_weapon_list(weapon: Weapon, weapon_list: &mut Vec<Weapon>) {
        weapon_list.push(weapon);
    }

    pub fn get_weapon(weapon_type: String, weapon_list: &mut Vec<Weapon>) -> Weapon {
        let weapon = weapon_list.iter().find(|&w| w.weapon_type == weapon_type).unwrap().clone();
        if let Some(idx) = weapon_list.iter().position(|w| w.weapon_type == weapon_type) {
            weapon_list.remove(idx);
        }
        weapon
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_weapon() {
        let mut wlist: Vec<Weapon> = Vec::new();
        Weapon::add_to_weapon_list(Weapon::new(String::from("Tank")), &mut wlist);
        assert_eq!(
            *wlist.get(0).unwrap(),
            Weapon {
                weapon_type: String::from("Tank")
            }
        );
    }
}
