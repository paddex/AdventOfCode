use crate::problem;

pub struct Day03;

impl problem::Solution for Day03 {
    fn name(&self) -> String {
        "Calorie Counting".to_string()
    }

    fn part_1(&self) -> String {
        let p = problem::load(2022, 1);
        "blah".to_string()
    }

    fn part_2(&self) -> String {
        "blub".to_string()
    }
}

fn get_elfs(data: &str) -> Vec<u32> {
    let mut out = data.split("\n\n");

    for val in out {
        println!("{}", val);
        println!("\n\n\n");
    }
    vec![0]
}
