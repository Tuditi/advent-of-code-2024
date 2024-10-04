use std::fmt::Debug;
use std::str::FromStr;

use rayon::iter::*;

pub fn parse_line<'a, T>(line: &'a str) -> impl Iterator<Item = T> + 'a
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    let maps: Vec<&str> = line.split(' ').collect();
    maps.into_iter()
        .map(|x| x.parse::<T>())
        .filter(|x| x.is_ok())
        .map(|x| x.unwrap())
}

pub fn par_parse_line<'a>(line: &'a str) -> impl rayon::iter::ParallelIterator<Item = i64> + 'a {
    let maps: Vec<&str> = line.split(' ').collect();
    maps.into_par_iter()
        .map(|x| x.parse::<i64>())
        .filter(|x| x.is_ok())
        .map(|x| x.unwrap())
}
