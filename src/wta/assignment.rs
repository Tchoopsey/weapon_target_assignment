use crate::{target::Target, weapon::Weapon};

#[derive(Debug, PartialEq)]
pub struct Assignment<'a> {
    weapon: &'a Weapon,
    target: &'a Target,
}

impl<'a> Assignment<'a> {
    /// Create a new Assignment weapon -> target
    pub fn new(weapon: &'a Weapon, target: &'a Target) -> Assignment<'a> {
        let target = target;
        let weapon = weapon;
        Assignment { weapon, target }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_assignment() {
        let t = Target::new(10.0, &vec![]);
        let w = Weapon::new("Tank".to_string());
        let a = Assignment::new(&w, &t);

        assert_eq!(
            a,
            Assignment::new(
                &Weapon::new("Tank".to_string()),
                &Target::new(10.0, &vec![])
            )
        );
    }
}
