use std::{cmp::Ordering, collections::HashMap};

use rayon::iter::split;
use regex::{Matches, Regex};

advent_of_code::solution!(19);

type Rule = dyn Fn(usize) -> Destination;

#[derive(Debug)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl Part {
    fn new(line: &str, re: &Regex) -> Self {
        let mut it = re
            .find_iter(line)
            .map(|m| m.as_str().parse::<usize>().unwrap());

        Self {
            x: it.next().unwrap(),
            m: it.next().unwrap(),
            a: it.next().unwrap(),
            s: it.next().unwrap(),
        }
    }
}

#[derive(Clone)]
struct Destination {
    name: String,
}

struct Workflow<'a> {
    x_rule: Option<&'a Rule>,
    m_rule: Option<&'a Rule>,
    a_rule: Option<&'a Rule>,
    s_rule: Option<&'a Rule>,
}

impl Workflow<'_> {
    pub fn new() -> Self {
        Self {
            x_rule: None,
            m_rule: None,
            a_rule: None,
            s_rule: None,
        }
    }
}

fn parse_inputs(input: &str) -> (Vec<Part>, HashMap<&str, Workflow>) {
    let mut workflows: HashMap<&str, Workflow> = HashMap::new();
    let mut parts: Vec<Part> = Vec::new();
    let digit_re = Regex::new(r"\d+").unwrap();

    let mut finished_parsing_rulse = false;
    for line in input.lines() {
        if line.is_empty() {
            finished_parsing_rulse = true;
            continue;
        }

        if finished_parsing_rulse {
            parts.push(Part::new(line, &digit_re));
        } else {
            add_workflow(line, &mut workflows);
        }
    }
    println!("parts: {:?}", parts);
    (parts, workflows)
}

// px{a<2006:qkq,m>2090:A,rfg}
fn add_workflow(line: &str, workflows: &mut HashMap<&str, Workflow>) {
    let mut it = line.split_terminator('{');
    let (name, rule) = (it.next().unwrap(), it.next().unwrap());
    println!("Name: {name}");
    rule.split_terminator(',').for_each(|r| println!("R: {r}"));
}

fn parse_comparison(comparison: &str) -> Ordering {
    match comparison {
        "<" => Ordering::Less,
        ">" => Ordering::Greater,
        _ => panic!("Unknown sign"),
    }
}

fn parse_rule(rule: &str) -> Box<Rule> {
    if rule.contains(':') {
        let mut it = rule.split_terminator(':');
        let (variable_name_comparison, digit_str) = it.next().unwrap().split_at(2);
        let digit = digit_str.parse::<usize>().unwrap();
        let destination = Destination {
            name: it.next().unwrap().to_string(),
        };
        let (variable_name, comparison) = variable_name_comparison.split_at(1);
        let ordering = parse_comparison(comparison);
        let function = move |val: usize| -> Destination {
            if val.cmp(&digit) == ordering {
                return destination.clone();
            } else {
                return Destination {
                    name: String::from("R"),
                };
            }
        };
        Box::new(function)
    } else {
        panic!("Problem")
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let _ = parse_inputs(input);
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
