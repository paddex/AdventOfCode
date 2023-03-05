use std::fs;

pub trait Solution {
    fn name(&self) -> String;
    fn part_1(&self) -> String;
    fn part_2(&self) -> String;
}

pub fn load(year: u32, day: u32) -> String {
    let file = format!("problems/{year}/{:02}", day);
    fs::read_to_string(&file).unwrap_or_else(|_| panic!("Error reading file {}", file))
}
