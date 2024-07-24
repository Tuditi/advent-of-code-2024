pub mod parsers {
    use rayon::iter::*;

    pub fn parse_line<'a>(line: &'a str) -> impl rayon::iter::ParallelIterator<Item = i64> + 'a {
        let maps: Vec<&str> = line.split(' ').collect();
        maps.into_par_iter()
            .map(|x| x.parse::<i64>())
            .filter(|x| x.is_ok())
            .map(|x| x.unwrap())
    }
}
