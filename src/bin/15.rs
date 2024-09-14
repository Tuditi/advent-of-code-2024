use itertools::Itertools;

advent_of_code::solution!(15);

type Box = Vec<Vec<Lens>>;

#[derive(Debug, Clone)]
struct Lens {
    label: String,
    strength: usize,
}

fn parse_input(input: &str) -> Box {
    let mut lens_box: Box = vec![];
    input.split(',').for_each(|instr| {
        if instr.contains('=') {
            set_lens(&mut lens_box, instr);
        } else if instr.contains('-') {
            remove_lens(&mut lens_box, instr);
        }
    });
    lens_box
}

fn set_lens(lens_box: &mut Box, instr: &str) {
    let mut iter = instr.split('=');
    let label = iter.next().unwrap().to_string();
    let strength = iter.next().unwrap().parse::<usize>().unwrap();
    let lens = Lens { label, strength };

    let box_number = hash_function(&lens.label);

    let box_size = lens_box.len();
    if box_number >= box_size {
        lens_box.extend(vec![vec![]; (box_number - box_size) + 1]);
    }

    if let Some(i) = find_lens_index(&lens_box[box_number], &lens.label) {
        lens_box[box_number][i] = lens.clone();
    } else {
        lens_box[box_number].push(lens.clone());
    }
}

fn remove_lens(lens_box: &mut Box, instr: &str) {
    let label = instr.split('-').next().unwrap();
    let box_number = hash_function(label);
    if box_number >= lens_box.len() {
        return;
    }

    if let Some(i) = find_lens_index(&lens_box[box_number], label) {
        lens_box[box_number].remove(i);
    }
}

fn find_lens_index(lenses: &Vec<Lens>, label: &str) -> Option<usize> {
    let (idx, _) = lenses.iter().find_position(|lens| lens.label == label)?;
    Some(idx)
}

fn hash_function(input: &str) -> usize {
    let mut current_value = 0;
    for c in input.chars() {
        current_value += c as usize;
        current_value = 17 * current_value;
        current_value = current_value % 256;
    }
    current_value
}

fn focusing_power(lens_box: &Box) -> Option<usize> {
    let mut count = 0;
    lens_box.iter().enumerate().for_each(|(i, lenses)| {
        lenses.iter().enumerate().for_each(|(slot_i, lens)| {
            count += (i + 1) * (slot_i + 1) * lens.strength;
        })
    });
    Some(count)
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut sum = 0;
    input.split(',').for_each(|step| sum += hash_function(step));
    Some(sum)
}

pub fn part_two(input: &str) -> Option<usize> {
    let lens_box = parse_input(input);
    focusing_power(&lens_box)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_function() {
        let result = hash_function("HASH");
        assert_eq!(result, 52);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1320));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(145));
    }
}

// Part 1: 514281 (2.7ms) && Part 2: 244199 (7.2ms)
