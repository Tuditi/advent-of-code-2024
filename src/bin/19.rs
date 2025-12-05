use std::{cmp::Ordering, collections::HashMap};

advent_of_code::solution!(19);

type Workflow<'a> = Vec<(char, Box<Rule<'a>>)>;

#[derive(Debug)]
struct Part {
    xmas: HashMap<char, usize>,
}

struct Rule<'a> {
    ordering: Ordering,
    pub value: Option<usize>,
    destination: &'a str,
}

#[derive(Debug, Clone)]
struct PartRange {
    xmas_ranges: HashMap<char, (usize, usize)>,
}

impl PartRange {
    fn new() -> Self {
        let range = (1usize, 4000usize);
        Self {
            xmas_ranges: HashMap::from([('x', range), ('m', range), ('a', range), ('s', range)]),
        }
    }

    fn total_elements(self) -> u64 {
        let mut result = 1u64;
        self.xmas_ranges.into_values().for_each(|r| {
            result *= (r.1 - r.0 + 1) as u64;
        });
        result
    }
}

impl Part {
    fn new(line: &str) -> Self {
        let mut values = [0; 4];
        let mut xmas: HashMap<char, usize> = HashMap::with_capacity(4);
        let mut num_start = None;
        let mut value_idx = 0;
        for (i, ch) in line.bytes().enumerate() {
            if ch.is_ascii_digit() {
                if num_start.is_none() {
                    num_start = Some(i);
                }
            } else if let Some(start) = num_start {
                let value = line[start..i].parse().unwrap();
                values[value_idx] = value;
                value_idx += 1;
                num_start = None;
            }
        }

        xmas.insert('x', values[0]);
        xmas.insert('m', values[1]);
        xmas.insert('a', values[2]);
        xmas.insert('s', values[3]);

        Self { xmas }
    }

    fn sum(&self) -> usize {
        self.xmas.values().sum()
    }

    fn get_destination<'a>(&self, category: &char, rule: &'a Rule) -> Option<&'a str> {
        // Default case
        if rule.value.is_none() {
            return Some(rule.destination);
        };

        let value = self.xmas.get(category).unwrap();
        if value.cmp(&rule.value.unwrap()) == rule.ordering {
            Some(rule.destination)
        } else {
            None
        }
    }
}

fn parse_inputs<'a>(input: &'a str) -> (Vec<Part>, HashMap<&'a str, Workflow<'a>>) {
    let mut workflows: HashMap<&'a str, Workflow<'a>> = HashMap::new();
    let mut parts: Vec<Part> = Vec::new();

    let mut finished_parsing_rulse = false;
    for line in input.lines() {
        if line.is_empty() {
            finished_parsing_rulse = true;
            continue;
        }

        if finished_parsing_rulse {
            parts.push(Part::new(line));
        } else {
            add_workflow(line, &mut workflows);
        }
    }
    (parts, workflows)
}

fn add_workflow<'a>(line: &'a str, workflows: &mut HashMap<&'a str, Workflow<'a>>) {
    let mut it = line.split_terminator('{');
    let (name, rule) = (it.next().unwrap(), it.next().unwrap());
    let mut workflow = Workflow::new();
    rule.split_terminator(',').for_each(|r| {
        let rule_fn = parse_rule(r);
        workflow.push(rule_fn);
    });
    workflows.insert(name, workflow);
}

fn parse_rule<'a>(rule: &'a str) -> (char, Box<Rule<'a>>) {
    if rule.contains(':') {
        let mut it = rule.split_terminator(':');
        let (variable_name_comparison, digit_str) = it.next().unwrap().split_at(2);
        let value = digit_str.parse::<usize>().unwrap();
        let destination = it.next().unwrap();
        let (variable_name, comparison) = variable_name_comparison.split_at(1);
        let ordering = parse_comparison(comparison);
        let rule = Rule {
            ordering,
            value: Some(value),
            destination,
        };

        (variable_name.chars().next().unwrap(), Box::new(rule))
    } else {
        let destination = rule.trim_end_matches('}');
        (
            'd',
            Box::new(Rule {
                ordering: Ordering::Equal,
                value: None,
                destination,
            }),
        )
    }
}

fn parse_comparison(comparison: &str) -> Ordering {
    match comparison {
        "<" => Ordering::Less,
        ">" => Ordering::Greater,
        _ => panic!("Unknown sign"),
    }
}

fn passes_workflow(part: &Part, rules: &HashMap<&str, Workflow>) -> bool {
    let mut current_rule = "in";
    loop {
        match current_rule {
            "R" => return false,
            "A" => return true,
            _ => {
                let workflow = rules.get(current_rule).unwrap();
                let mut destination: Option<&str> = None;
                for (category, rule) in workflow.iter() {
                    destination = part.get_destination(category, rule);
                    if destination.is_some() {
                        break;
                    }
                }
                current_rule = destination.unwrap();
            }
        }
    }
}

fn split_workflow<'a>(
    stack: &mut Vec<(&'a str, PartRange)>,
    rules: &'a HashMap<&str, Workflow>,
) -> Vec<PartRange> {
    let input = stack.pop().unwrap();
    let workflow = rules.get(input.0).unwrap();
    let mut input_reject = input.1.clone();
    let mut result = Vec::with_capacity(5);
    // Create stack = Vec::new(["in", partRange])
    // Pop input, split in parts and put new ranges back on stack for further processing.
    // When "A", put on result, When R, stop.

    for (cat, rule) in workflow.iter() {
        if rule.value.is_some() {
            let mut input_pass = input_reject.clone();
            let range = input_reject.xmas_ranges.get_mut(cat).unwrap();
            let mut new_range = *range;

            let value = rule.value.unwrap();
            if rule.ordering == Ordering::Greater {
                new_range.0 = value + 1;
                range.1 = value;
            } else {
                new_range.1 = value - 1;
                range.0 = value;
            }
            input_pass.xmas_ranges.insert(*cat, new_range);
            match rule.destination {
                "R" => {}
                "A" => result.push(input_pass),
                _ => stack.push((rule.destination, input_pass)),
            }
        } else if rule.destination == "R" {
            continue;
        } else if rule.destination == "A" {
            result.push(input_reject.clone())
        } else {
            stack.push((rule.destination, input_reject.clone()))
        }
    }
    result
}

pub fn part_one(input: &str) -> Option<usize> {
    let (parts, rules) = parse_inputs(input);
    let result = parts
        .iter()
        .filter(|&p| passes_workflow(p, &rules))
        .map(|p| p.sum())
        .sum();
    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (_, rules) = parse_inputs(input);
    let mut stack = vec![("in", PartRange::new())];
    let mut result: Vec<PartRange> = Vec::new();
    while !stack.is_empty() {
        let mut intermediate_result = split_workflow(&mut stack, &rules);
        result.append(&mut intermediate_result);
    }
    let res = result
        .into_iter()
        .fold(0u64, |acc, e| acc + e.total_elements());
    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_worfklow() {
        let part = Part::new("{x=787,m=2655,a=1222,s=2876}");
        let input = advent_of_code::template::read_file("examples", DAY);
        let (_, workflows) = parse_inputs(&input);
        let result = passes_workflow(&part, &workflows);
        assert_eq!(result, true);
    }

    #[test]
    fn test_part_sum() {
        let part = PartRange::new();
        let result = part.total_elements();
        assert_eq!(result, 256000000000000);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(19114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(167409079868000));
    }
}

// Part 1: 319062 (4.6ms)
// Part 2: 118638369682135 (4.8ms)
