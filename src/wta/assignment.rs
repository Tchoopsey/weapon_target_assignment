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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_assignment() {
    }
}
