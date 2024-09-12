advent_of_code::solution!(14);

type RockColumn = Vec<char>;

trait Solution {
    fn slide_north(self) -> RockColumn;
    fn calculate_load(&self) -> usize;
}

impl Solution for RockColumn {
    fn slide_north(self) -> RockColumn {
        let mut sorted_col: Vec<char> = Vec::with_capacity(self.len());
        let free_cols = self.split(|c| *c == '#');
        free_cols.into_iter().for_each(|cols| {
            let mut col_vec = cols.to_vec();
            //println!("Before sorting: {:?}", col_vec);
            col_vec.sort_by(|a, b| b.cmp(a));
            sorted_col.extend(col_vec);
            sorted_col.push('#');
            //println!("After sorting {:?}", sorted_col);
        });
        sorted_col.pop();
        sorted_col
    }

    fn calculate_load(&self) -> usize {
        let mut load = 0;
        let length = self.len();
        self.iter().enumerate().for_each(|(i, rock)| {
            //println!("Rock: {rock}");
            if *rock == 'O' {
                //println!("Add length: {}", length - i);
                load += length - i;
            }
        });
        load
    }
}

fn parse_input(input: &str) -> Vec<RockColumn> {
    let lines = input.lines();
    let size = lines.clone().count();
    let mut rocks: Vec<RockColumn> = vec![Vec::new(); size];

    lines.for_each(|l| {
        l.chars().enumerate().for_each(|(i, c)| rocks[i].push(c));
    });
    rocks
}
pub fn part_one(input: &str) -> Option<usize> {
    let rocks = parse_input(input);
    let mut total_load = 0;
    rocks.into_iter().for_each(|rock| {
        let new_rock = rock.slide_north();
        total_load += new_rock.calculate_load();
    });
    Some(total_load)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sorting() {
        let rocks: RockColumn = vec!['O', 'O', '.', 'O', '.', 'O', '.', '.', '#', '#'];
        let result = rocks.slide_north();
        assert_eq!(
            result,
            vec!['O', 'O', 'O', 'O', '.', '.', '.', '.', '#', '#']
        )
    }

    #[test]
    fn test_load() {
        let rocks: RockColumn = vec!['O', 'O', 'O', 'O', '.', '.', '.', '.', '#', '#'];
        let result = rocks.calculate_load();
        assert_eq!(result, 34)
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(136));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
