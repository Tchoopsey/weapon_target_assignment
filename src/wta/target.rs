#[derive(Debug, PartialEq)]
pub struct Target {
    name: String,
    value: f32,
}

#[allow(dead_code)]
impl Target {
    /// Create a new Target with a name "T+target_list.len()" and health-like value
    pub fn new(value: f32, target_list: &Vec<Target>) -> Target {
        let mut name = String::from("T");
        name += (target_list.len() + 1).to_string().as_str();
        Target { name, value }
    }

    pub fn add_to_target_list(self, target_list: &mut Vec<Target>) {
        target_list.push(self);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut tlist: Vec<Target> = Vec::new();
        let t = Target::new(10.0, &tlist);
        let t2 = Target::new(20.0, &tlist);

        t.add_to_target_list(&mut tlist);
        t2.add_to_target_list(&mut tlist);

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
