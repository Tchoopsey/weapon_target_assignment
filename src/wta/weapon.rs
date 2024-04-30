#[derive(Debug, PartialEq)]
pub struct Weapon {
    weapon_type: String,
}

#[allow(dead_code)]
impl Weapon {
    /// Create a new Weapon with a weapon_type
    pub fn new(weapon_type: String) -> Weapon {
        Weapon { weapon_type }
    }
}

#[derive(Debug, PartialEq)]
pub struct TypeCounter<'a> {
    weapon_type: &'a Weapon,
    counter: u8,
}
impl<'a> TypeCounter<'a> {
    pub fn new(weapon: &'a Weapon) -> TypeCounter {
        TypeCounter { weapon_type: weapon, counter: 1 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_weapon() {
        let w = Weapon::new(String::from("Tank"));
        let wtc: TypeCounter = TypeCounter::new(&w);
        assert_eq!(
            w,
            Weapon {
                weapon_type: String::from("Tank")
            }
        );
        assert_eq!(
            wtc,
            TypeCounter {
                weapon_type: &w, counter: 1
            }
        );
    }
}
