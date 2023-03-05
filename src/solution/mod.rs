use crate::problem;

mod year_2022;

pub fn get_year(year: u32) -> &'static [&'static dyn problem::Solution] {
    match year {
        2022 => &year_2022::ALL,
        _ => &[],
    }
}
