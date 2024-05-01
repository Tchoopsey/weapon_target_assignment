#[derive(Debug, PartialEq, Clone)]
pub struct Target {
    name: String,
    value: f32,
}

#[allow(dead_code)]
impl Target {
    /// Create a new Target with a name "T+target_list.len()" and health-like value
    pub fn new(value: f32, target_list: &mut Vec<Target>) -> Target {
        let mut name = String::from("T");
        name += (target_list.len() + 1).to_string().as_str();
        Target { name, value }
    }

    pub fn add_to_target_list(target: Target, target_list: &mut Vec<Target>) {
        target_list.push(target);
    }

    pub fn get_target(target_name: String, target_list: &mut Vec<Target>) -> Option<Target> {
        let target = target_list
            .iter()
            .find(|&t| t.name == target_name)?
            .clone();
        // if let Some(idx) = target_list.iter().position(|t| t.name == target_name) {
        //     target_list.remove(idx);
        // }

        Some(target)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut tlist: Vec<Target> = Vec::new();
        let t = Target::new(10.0, &mut tlist);
        let t2 = Target::new(20.0, &mut tlist);
        Target::add_to_target_list(t, &mut tlist);
        Target::add_to_target_list(t2, &mut tlist);

        assert_eq!(tlist.len(), 2);
        assert_eq!(
            *tlist.get(0).unwrap(),
            Target {
                name: String::from("T1"),
                value: 10.0
            }
        );
    }
}
