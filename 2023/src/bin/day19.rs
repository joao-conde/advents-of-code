use regex::Regex;
use std::collections::HashMap;

const PART_RE: &str = r"x=(?P<x>\d*),m=(?P<m>\d*),a=(?P<a>\d*),s=(?P<s>\d*)";
const WORKFLOW_RE: &str = r"(?P<name>.*)\{(?P<rules>.*)\}";

type WorkflowId = String;

struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
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

impl Part {
    fn rating(&self, category: &Category) -> usize {
        match category {
            Category::X => self.x,
            Category::M => self.m,
            Category::A => self.a,
            Category::S => self.s,
        }
    }

    fn total_rating(&self) -> usize {
        self.x + self.m + self.a + self.s
    }
}

enum Category {
    X,
    M,
    A,
    S,
}

impl From<&str> for Category {
    fn from(value: &str) -> Self {
        match value {
            "x" => Category::X,
            "m" => Category::M,
            "a" => Category::A,
            "s" => Category::S,
            _ => unreachable!(),
        }
    }
}

struct Workflow {
    name: WorkflowId,
    rules: RuleTree,
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

impl Workflow {
    fn test(&self, workflows: &HashMap<WorkflowId, Workflow>, part: &Part) -> Verdict {
        self.rules.test(workflows, part)
    }
}

enum RuleTree {
    Test {
        operand1: Category,
        operator: Operator,
        operand2: usize,
        true_branch: Box<RuleTree>,
        false_branch: Box<RuleTree>,
    },
    Workflow(WorkflowId),
    Verdict(Verdict),
}

impl From<&str> for RuleTree {
    fn from(value: &str) -> Self {
        if value == "A" {
            return RuleTree::Verdict(Verdict::Accepted);
        }

        if value == "R" {
            return RuleTree::Verdict(Verdict::Rejected);
        }

        let Some((rule, other_rules)) = value.split_once(',') else {
            return RuleTree::Workflow(value.to_string());
        };

        let (test, true_branch) = rule.split_once(':').unwrap();

        let operator = if test.contains('>') {
            Operator::GreaterThan
        } else if test.contains('<') {
            Operator::LesserThan
        } else {
            return RuleTree::Workflow(test.to_string());
        };

        let (operand1, operand2) = test.split_once(&operator.to_string()).unwrap();
        let operand1 = Category::from(operand1);
        let operand2 = operand2.parse().unwrap();
        let true_branch = Box::new(RuleTree::from(true_branch));
        let false_branch = Box::new(RuleTree::from(other_rules));

        RuleTree::Test {
            operand1,
            operator,
            operand2,
            true_branch,
            false_branch,
        }
    }
}

impl RuleTree {
    fn test(&self, workflows: &HashMap<WorkflowId, Workflow>, part: &Part) -> Verdict {
        match self {
            RuleTree::Test {
                operand1,
                operator,
                operand2,
                true_branch,
                false_branch,
            } => {
                if operator.test(part.rating(operand1), *operand2) {
                    true_branch.test(workflows, part)
                } else {
                    false_branch.test(workflows, part)
                }
            }
            RuleTree::Workflow(name) => workflows[name].rules.test(workflows, part),
            RuleTree::Verdict(Verdict::Accepted) => Verdict::Accepted,
            RuleTree::Verdict(Verdict::Rejected) => Verdict::Rejected,
        }
    }
}

#[derive(PartialEq)]
enum Verdict {
    Accepted,
    Rejected,
}

enum Operator {
    GreaterThan,
    LesserThan,
}

impl ToString for Operator {
    fn to_string(&self) -> String {
        match self {
            Operator::GreaterThan => ">".to_string(),
            Operator::LesserThan => "<".to_string(),
        }
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

fn main() {
    let input = std::fs::read_to_string("input/day19").unwrap();
    let (workflows, parts) = input.split_once("\n\n").unwrap();

    let parts: Vec<Part> = parts.lines().map(Part::from).collect();
    let workflows = workflows.lines().fold(HashMap::new(), |mut acc, line| {
        let workflow = Workflow::from(line);
        acc.insert(workflow.name.clone(), workflow);
        acc
    });

    let p1: usize = parts
        .iter()
        .filter(|p| accepted(&workflows, p))
        .map(|p| p.total_rating())
        .sum();
    println!("Part1: {p1}");

    let p2 = combinations(&workflows);
    println!("Part2: {p2}");

    assert!(p1 == 383682);
    assert!(p2 == 117954800808317);
}

fn accepted(workflows: &HashMap<WorkflowId, Workflow>, part: &Part) -> bool {
    workflows["in"].test(workflows, part) == Verdict::Accepted
}

fn combinations(workflows: &HashMap<WorkflowId, Workflow>) -> usize {
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
                match operand1 {
                    Category::X => tx = true_bounds(tx),
                    Category::M => tm = true_bounds(tm),
                    Category::A => ta = true_bounds(ta),
                    Category::S => ts = true_bounds(ts),
                }

                let (mut fx, mut fm, mut fa, mut fs) = (x, m, a, s);
                match operand1 {
                    Category::X => fx = false_bounds(fx),
                    Category::M => fm = false_bounds(fm),
                    Category::A => fa = false_bounds(fa),
                    Category::S => fs = false_bounds(fs),
                }

                queue.push((&true_branch, tx, tm, ta, ts));
                queue.push((&false_branch, fx, fm, fa, fs));
            }
            RuleTree::Workflow(name) => queue.push((&workflows[name].rules, x, m, a, s)),
            RuleTree::Verdict(Verdict::Accepted) => {
                let path_combinations =
                    (x.1 + 1 - x.0) * (m.1 + 1 - m.0) * (a.1 + 1 - a.0) * (s.1 + 1 - s.0);
                combinations += path_combinations;
            }
            RuleTree::Verdict(Verdict::Rejected) => (),
        }
    }

    combinations
}
