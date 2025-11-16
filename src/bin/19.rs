use std::{cmp::Ordering, collections::HashMap};

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use regex::Regex;

advent_of_code::solution!(19);

type Rule = dyn Fn(usize) -> Option<Destination>;
type Workflow = Vec<(char, Box<Rule>)>;

#[derive(Debug)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl Part {
    fn new(line: &str) -> Self {
        let digit_re = Regex::new(r"\d+").unwrap();

        let mut it = digit_re
            .find_iter(line)
            .map(|m| m.as_str().parse::<usize>().unwrap());

        Self {
            x: it.next().unwrap(),
            m: it.next().unwrap(),
            a: it.next().unwrap(),
            s: it.next().unwrap(),
        }
    }

    fn sum(&self) -> usize {
        self.x + self.m + self.a + self.s
    }
}

#[derive(Clone, Debug)]
struct Destination {
    name: String,
}

fn parse_inputs(input: &str) -> (Vec<Part>, HashMap<&str, Workflow>) {
    let mut workflows: HashMap<&str, Workflow> = HashMap::new();
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

fn add_workflow<'a>(line: &'a str, workflows: &mut HashMap<&'a str, Workflow>) {
    let mut it = line.split_terminator('{');
    let (name, rule) = (it.next().unwrap(), it.next().unwrap());
    let mut workflow = Workflow::new();
    rule.split_terminator(',').for_each(|r| {
        let rule_fn = parse_rule(r);
        workflow.push(rule_fn);
    });
    workflows.insert(name, workflow);
}

fn parse_rule(rule: &str) -> (char, Box<Rule>) {
    if rule.contains(':') {
        let mut it = rule.split_terminator(':');
        let (variable_name_comparison, digit_str) = it.next().unwrap().split_at(2);
        let digit = digit_str.parse::<usize>().unwrap();
        let destination = Destination {
            name: it.next().unwrap().to_string(),
        };
        let (variable_name, comparison) = variable_name_comparison.split_at(1);
        let ordering = parse_comparison(comparison);
        let function = move |val: usize| -> Option<Destination> {
            if val.cmp(&digit) == ordering {
                return Some(destination.clone());
            } else {
                return None;
            }
        };
        (variable_name.chars().next().unwrap(), Box::new(function))
    } else {
        let destination = Destination {
            name: rule.trim_end_matches('}').to_string(),
        };
        ('d', Box::new(move |_| Some(destination.clone())))
    }
}

fn parse_comparison(comparison: &str) -> Ordering {
    match comparison {
        "<" => Ordering::Less,
        ">" => Ordering::Greater,
        _ => panic!("Unknown sign"),
    }
}

fn eval_workflow(
    part: &Part,
    rules: &HashMap<&str, Workflow>,
    next_destination: Option<&str>,
) -> usize {
    if next_destination.is_none() {
        eval_workflow(part, rules, Some("in"))
    } else if next_destination.unwrap() == "R" {
        return 0;
    } else if next_destination.unwrap() == "A" {
        return part.sum();
    } else {
        let dest = next_destination.unwrap();
        let workflows = rules.get(dest).unwrap();
        let mut destination: Option<Destination> = None;
        for (category, rule) in workflows.iter() {
            destination = match category {
                'x' => rule(part.x),
                'm' => rule(part.m),
                'a' => rule(part.a),
                's' => rule(part.s),
                'd' => rule(0), // Default case doesn't take into consideration number
                _ => panic!("FLD"),
            };

            if destination.is_some() {
                break;
            }
        }
        eval_workflow(part, rules, Some(destination.unwrap().name.as_str()))
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let (parts, rules) = parse_inputs(input);
    let result = parts
        .iter()
        .map(|part| eval_workflow(part, &rules, None))
        .sum::<usize>();
    Some(result as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_worfklow() {
        let part = Part::new("{x=787,m=2655,a=1222,s=2876}");
        let input = advent_of_code::template::read_file("examples", DAY);
        let (_, workflows) = parse_inputs(&input);
        let result = eval_workflow(&part, &workflows, None);
        assert_eq!(result, 7540);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(19114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
