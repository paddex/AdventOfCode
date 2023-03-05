use crate::problem;

pub struct Day01;

impl problem::Solution for Day01 {
    fn name(&self) -> String {
        "Calorie Counting".to_string()
    }

    fn part_1(&self) -> String {
        let prob = problem::load(2022, 1);
        let elfs = get_elfs(&prob);

        elfs.last().unwrap().to_string()
    }

    fn part_2(&self) -> String {
        let prob = problem::load(2022, 1);
        let elfs = get_elfs(&prob);

        elfs.iter().rev().take(3).sum::<u32>().to_string()
    }
}

fn get_elfs(data: &str) -> Vec<u32> {
    let mut out: Vec<u32> = data
        .split("\n\n")
        .map(|x| x.lines().map(|x| x.parse::<u32>().unwrap()).sum())
        .collect();

    out.sort();
    out
}
