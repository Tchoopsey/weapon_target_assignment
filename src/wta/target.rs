#[derive(Debug, PartialEq, Clone)]
pub struct Target {
    name: String,
    pub value: f32,
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

    pub fn replace_target(target_list: &mut Vec<Target>, target: Target) {
        let idx = target_list
            .iter()
            .position(|t| t.name == target.name)
            .unwrap();
        if let Some(t) = target_list.get_mut(idx) {
            *t = target;
        } else {
            println!("out of bounds");
        }
    }

    pub fn get_target(target_name: String, target_list: &mut Vec<Target>) -> Option<Target> {
        let target = target_list.iter().find(|&t| t.name == target_name)?.clone();

        Some(target)
    }

    pub fn get_name(self) -> String {
        self.name
    }
}
