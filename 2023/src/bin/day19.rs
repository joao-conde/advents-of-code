use regex::Regex;
use std::{collections::HashMap, fmt::Display};

const PART_RE: &str = r"x=(?P<x>\d*),m=(?P<m>\d*),a=(?P<a>\d*),s=(?P<s>\d*)";
const WORKFLOW_RE: &str = r"(?P<name>.*)\{(?P<rules>.*)\}";

#[derive(Debug)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

#[derive(Debug)]
struct Workflow {
    name: String,
    rules: Vec<Rule>,
}

#[derive(Debug)]
enum Rule {
    Test {
        operand1: String,
        operator: Operator,
        operand2: usize,
        success: String,
    },
    Result(String),
}

#[derive(Debug)]
enum Operator {
    GreaterThan,
    LesserThan,
}

impl Workflow {
    fn test(&self, part: &Part) -> String {
        for rule in &self.rules {
            if let Some(result) = rule.test(part) {
                return result;
            }
        }
        unreachable!()
    }
}

impl Part {
    fn get(&self, component: &str) -> usize {
        match component {
            "x" => self.x,
            "m" => self.m,
            "a" => self.a,
            "s" => self.s,
            _ => unreachable!(),
        }
    }

    fn rating(&self) -> usize {
        self.x + self.m + self.a + self.s
    }
}

impl Operator {
    fn test(&self, operand1: usize, operand2: usize) -> bool {
        match self {
            Operator::GreaterThan => operand1 > operand2,
            Operator::LesserThan => operand1 < operand2,
        }
    }
}

impl Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operator::GreaterThan => f.write_str(">"),
            Operator::LesserThan => f.write_str("<"),
        }
    }
}

impl From<&str> for Part {
    fn from(value: &str) -> Self {
        let re = Regex::new(PART_RE).unwrap();
        let captures = re.captures(value).unwrap();
        Part {
            x: captures["x"].parse().unwrap(),
            m: captures["m"].parse().unwrap(),
            a: captures["a"].parse().unwrap(),
            s: captures["s"].parse().unwrap(),
        }
    }
}

impl From<&str> for Workflow {
    fn from(value: &str) -> Self {
        let re = Regex::new(WORKFLOW_RE).unwrap();
        let captures = re.captures(value).unwrap();

        let name = captures["name"].to_string();
        let rules = captures["rules"]
            .split(",")
            .map(|r| {
                if let Some((condition, success)) = r.split_once(":") {
                    let operator = if condition.contains(">") {
                        Operator::GreaterThan
                    } else {
                        Operator::LesserThan
                    };

                    let (operand1, operand2) = condition.split_once(&operator.to_string()).unwrap();
                    let operand1 = operand1.to_string();
                    let operand2 = operand2.parse().unwrap();
                    let success = success.to_string();

                    Rule::Test {
                        operand1,
                        operator,
                        operand2,
                        success,
                    }
                } else {
                    Rule::Result(r.to_string())
                }
            })
            .collect();

        Workflow { name, rules }
    }
}

impl Rule {
    fn test(&self, part: &Part) -> Option<String> {
        match self {
            Rule::Test {
                operand1,
                operator,
                operand2,
                success,
            } => {
                if operator.test(part.get(operand1), *operand2) {
                    Some(success.to_string())
                } else {
                    None
                }
            }
            Rule::Result(result) => Some(result.to_string()),
        }
    }
}

fn main() {
    let input = std::fs::read_to_string("input/day19").unwrap();

    let (workflows, parts) = input.split_once("\n\n").unwrap();

    let parts: Vec<Part> = parts.lines().map(Part::from).collect();
    let workflows: HashMap<String, Workflow> =
        workflows.lines().fold(HashMap::new(), |mut acc, line| {
            let workflow = Workflow::from(line);
            acc.insert(workflow.name.clone(), workflow);
            acc
        });

    let p1: usize = parts
        .iter()
        .filter(|p| accepted(&workflows, p))
        .map(|p| p.rating())
        .sum();
    dbg!(p1);
}

fn accepted(workflows: &HashMap<String, Workflow>, part: &Part) -> bool {
    let mut workflow = &workflows["in"];
    loop {
        let result = workflow.test(part);
        match result.as_str() {
            "A" => return true,
            "R" => return false,
            next_workflow => workflow = &workflows[next_workflow],
        }
    }
}
