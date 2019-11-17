use rand;
use rand::distributions::Distribution;

fn generate_random_name() -> String {
    let mut rng = rand::thread_rng();
    let mut res = String::new();
    let letter = rand::distributions::Uniform::from(0..26);
    let number = rand::distributions::Uniform::from(0..1000);
    for _ in 0..2 {
        let i: u8 = letter.sample(&mut rng);
        let c = (65 + i) as char;
        res.push(c);
    }
    let n = number.sample(&mut rng);
    format!("{}{:03}", res, n)
}

pub struct NamedRobot {
    name: String,
}

pub struct UnnamedRobot {}

pub fn new_robot() -> UnnamedRobot {
    UnnamedRobot {}
}

impl NamedRobot {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn stop(&self) {}

    pub fn start(&self) {}

    pub fn reset(self) -> UnnamedRobot {
        UnnamedRobot {}
    }
}

impl UnnamedRobot {
    pub fn start(self) -> NamedRobot {
        NamedRobot {
            name: generate_random_name(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn name_is_not_set_at_frst() {
        let _robot = new_robot();
        // robot.name() // does not compile: method name() not found for UnnamedRobot
    }

    #[test]
    fn started_robots_have_a_name() {
        let robot = new_robot();
        let robot = robot.start();
        assert!(robot.name() != "");
    }

    #[test]
    fn name_does_not_change_when_rebooted() {
        let robot = new_robot();
        let named_robot = robot.start();
        let name1 = named_robot.name();
        named_robot.stop();
        named_robot.start();
        let name2 = named_robot.name();
        assert_eq!(name1, name2);
    }
    #[test]
    fn name_changes_after_a_reset() {
        let robot = new_robot();
        let robot = robot.start();
        let name1 = robot.name().to_string();

        // robot.reset();
        // robot.name(); // does not compile: value moved in reset()

        let robot = robot.reset();
        // robot.name() does not compile: method name() not found for  UnnamedRobot
        let robot = robot.start();
        let name2 = robot.name().to_string();
        assert_ne!(name1, name2);
    }
}
