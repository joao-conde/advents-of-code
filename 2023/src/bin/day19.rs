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
    rules: RuleTree,
}

#[derive(Debug)]
enum RuleTree {
    Test {
        operand1: String,
        operator: Operator,
        operand2: usize,
        true_branch: Box<RuleTree>,
        false_branch: Box<RuleTree>,
    },
    Accepted,
    Rejected,
    Workflow(String),
}

#[derive(Debug)]
enum Operator {
    GreaterThan,
    LesserThan,
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
        let rules = RuleTree::from(&captures["rules"]);

        Workflow { name, rules }
    }
}

impl From<&str> for RuleTree {
    fn from(value: &str) -> Self {
        if value == "A" {
            RuleTree::Accepted
        } else if value == "R" {
            RuleTree::Rejected
        } else {
            let Some((rule, others)) = value.split_once(',') else {
                return RuleTree::Workflow(value.to_string());
            };

            let (test, true_branch) = rule.split_once(':').unwrap();

            if !test.contains(">") && !test.contains("<") {
                RuleTree::Workflow(test.to_string())
            } else {
                let operator = if test.contains('>') {
                    Operator::GreaterThan
                } else {
                    Operator::LesserThan
                };

                let (operand1, operand2) = test.split_once(&operator.to_string()).unwrap();
                let operand1 = operand1.to_string();
                let operand2 = operand2.parse().unwrap();
                let true_branch = Box::new(RuleTree::from(true_branch));
                let false_branch = Box::new(RuleTree::from(others));

                RuleTree::Test {
                    operand1,
                    operator,
                    operand2,
                    true_branch,
                    false_branch,
                }
            }
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

    // px{a<2006:qkq,m>2090:A,rfg}
    //       a < 2006
    //      qkq      m>2090
    //              A       rfg

    dbg!(combinations(&workflows));
}

fn combinations(workflows: &HashMap<String, Workflow>) -> usize {
    let mut combinations = 0;

    let mut queue = vec![(
        &workflows["in"].rules,
        (1, 4000),
        (1, 4000),
        (1, 4000),
        (1, 4000),
    )];

    while let Some((tree, x, m, a, s)) = queue.pop() {
        match tree {
            RuleTree::Test {
                operand1,
                operator,
                operand2,
                true_branch,
                false_branch,
            } => {
                let true_bounds = |(lb, ub)| match operator {
                    Operator::GreaterThan => (usize::max(lb, *operand2) + 1, ub),
                    Operator::LesserThan => (lb, usize::min(ub, *operand2) - 1),
                };

                let false_bounds = |(lb, ub)| match operator {
                    Operator::LesserThan => (usize::max(lb, *operand2), ub),
                    Operator::GreaterThan => (lb, usize::min(ub, *operand2)),
                };

                let (mut tx, mut tm, mut ta, mut ts) = (x, m, a, s);
                match operand1.as_str() {
                    "x" => tx = true_bounds(tx),
                    "m" => tm = true_bounds(tm),
                    "a" => ta = true_bounds(ta),
                    "s" => ts = true_bounds(ts),
                    _ => unreachable!(),
                }

                let (mut fx, mut fm, mut fa, mut fs) = (x, m, a, s);
                match operand1.as_str() {
                    "x" => fx = false_bounds(fx),
                    "m" => fm = false_bounds(fm),
                    "a" => fa = false_bounds(fa),
                    "s" => fs = false_bounds(fs),
                    _ => unreachable!(),
                }

                queue.push((&true_branch, tx, tm, ta, ts));
                queue.push((&false_branch, fx, fm, fa, fs));
            }
            RuleTree::Accepted => {
                combinations +=
                    ((x.1 + 1 - x.0) * (m.1 + 1 - m.0) * (a.1 + 1 - a.0) * (s.1 + 1 - s.0));
            }
            RuleTree::Rejected => (),
            RuleTree::Workflow(name) => queue.push((&workflows[name].rules, x, m, a, s)),
        }
    }

    combinations
}
