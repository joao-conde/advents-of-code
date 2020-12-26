use std::collections::{HashMap, VecDeque};
use std::fs;

type Precedences = HashMap<Token, u8>;

#[derive(Debug, PartialEq, Eq, Hash)]
enum Token {
    Val(usize),
    Add,
    Mul,
    LeftP,
}

fn main() {
    let input = fs::read_to_string("input/day18").expect("failure opening input file");

    let mut precedences = HashMap::new();
    precedences.insert(Token::Add, 1);
    precedences.insert(Token::Mul, 1);
    precedences.insert(Token::LeftP, 0);
    let p1 = input.lines().map(|exp| shunting_yard(exp, &precedences)).sum::<usize>();
    println!("Part1: {}", p1);

    precedences.insert(Token::Add, 1);
    precedences.insert(Token::Mul, 0);
    precedences.insert(Token::LeftP, 0);
    let p2 = input.lines().map(|exp| shunting_yard(exp, &precedences)).sum::<usize>();
    println!("Part2: {}", p2);
}

fn shunting_yard(exp: &str, precedences: &Precedences) -> usize {
    let tokens = exp.chars().filter(|c| !c.is_whitespace());
    let mut stack = vec![];
    let mut dequeue = VecDeque::new();
    for token in tokens {
        match token {
            '(' => stack.push(Token::LeftP),
            ')' => loop {
                match stack.pop() {
                    Some(Token::LeftP) | None => break,
                    Some(top) => dequeue.push_back(top),
                }
            },
            '+' => process_op(Token::Add, &mut stack, &mut dequeue, &precedences),
            '*' => process_op(Token::Mul, &mut stack, &mut dequeue, &precedences),
            num => dequeue.push_back(Token::Val(num.to_digit(10).unwrap() as usize)),
        }
    }

    while let Some(top) = stack.pop() {
        dequeue.push_back(top)
    }

    while let Some(front) = dequeue.pop_front() {
        match front {
            Token::Add => {
                if let (Some(Token::Val(lhs)), Some(Token::Val(rhs))) = (stack.pop(), stack.pop()) {
                    stack.push(Token::Val(lhs + rhs));
                }
            }
            Token::Mul => {
                if let (Some(Token::Val(lhs)), Some(Token::Val(rhs))) = (stack.pop(), stack.pop()) {
                    stack.push(Token::Val(lhs * rhs));
                }
            }
            val => stack.push(val),
        }
    }

    match stack.pop() {
        Some(Token::Val(val)) => val,
        _ => 0,
    }
}

fn process_op(token: Token, stack: &mut Vec<Token>, dequeue: &mut VecDeque<Token>, precedences: &Precedences) {
    let precedence = precedences.get(&token).unwrap();
    while let Some(token) = stack.last() {
        match token {
            Token::Add | Token::Mul if precedence <= precedences.get(token).unwrap() => dequeue.push_back(stack.pop().unwrap()),
            _ => break,
        }
    }
    stack.push(token);
}
