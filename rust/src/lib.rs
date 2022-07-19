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

pub struct Robot {
    name: Option<String>,
}

impl Robot {
    pub fn new() -> Self {
        Self { name: None }
    }

    pub fn name(&self) -> &Option<String> {
        &self.name
    }

    pub fn stop(&mut self) {}

    pub fn start(&mut self) {
        if self.name.is_none() {
            self.name = Some(generate_random_name())
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
        let name1 = robot.name().to_owned();
        robot.stop();
        robot.start();
        let name2 = robot.name().to_owned();

        assert_eq!(name1, name2);
    }
    #[test]
    fn name_changes_after_a_reset() {
        let mut robot = Robot::new();
        robot.start();
        let name1 = robot.name().to_owned();

        robot.reset();

        let name2 = robot.name();
        assert_ne!(&name1, name2);
    }
}
