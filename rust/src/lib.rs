use std::str::FromStr;

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

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct RobotName(String);

impl FromStr for RobotName {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let len = s.len();
        if len != 5 {
            return Err(format!("Expecting string of size 6, got {len}"));
        }
        let chars: Vec<_> = s.chars().collect();
        for i in 0..2 {
            let c = &chars[i];
            if !c.is_ascii_uppercase() {
                return Err(format!(
                    "At index {i} - expecting ASCII upper case, got {c}"
                ));
            }
        }
        for i in 3..5 {
            let c = &chars[i];
            if !c.is_ascii_digit() {
                return Err(format!(
                    "At index {i} - expecting ASCII upper case, got {c}"
                ));
            }
        }
        Ok(Self(s.to_string()))
    }
}

impl AsRef<str> for RobotName {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

pub struct Robot {
    name: Option<RobotName>,
}

impl Robot {
    pub fn new() -> Self {
        Self { name: None }
    }

    pub fn name(&self) -> Option<&RobotName> {
        self.name.as_ref()
    }

    pub fn stop(&mut self) {}

    pub fn start(&mut self) {
        let random_string = generate_random_name();
        let name: RobotName = random_string.parse().expect("random names should be valid");
        if self.name.is_none() {
            self.name = Some(name)
        }
    }

    pub fn reset(&mut self) {
        self.name = None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn name_is_not_set_at_frst() {
        let robot = Robot::new();
        assert!(robot.name().is_none())
    }

    #[test]
    fn started_robots_have_a_name() {
        let mut robot = Robot::new();
        robot.start();
        assert!(robot.name().is_some());
    }

    #[test]
    fn name_does_not_change_when_rebooted() {
        let mut robot = Robot::new();

        robot.start();
        let name1 = robot.name().unwrap().clone();
        robot.stop();
        robot.start();
        let name2 = robot.name().unwrap();

        assert_eq!(&name1, name2);
    }
    #[test]
    fn name_changes_after_a_reset() {
        let mut robot = Robot::new();
        robot.start();
        let name1 = robot.name().unwrap().clone();

        robot.reset();
        robot.start();

        let name2 = robot.name().unwrap();
        assert_ne!(&name1, name2);
    }
}
