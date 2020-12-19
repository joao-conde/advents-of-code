use std::collections::VecDeque;
use std::fs;

type ProcessOpFn = fn(Token, &mut Vec<Token>, &mut VecDeque<Token>);

#[derive(Debug)]
enum Token {
    Val(usize),
    Add,
    Mul,
    LeftP,
}

fn main() {
    let input = fs::read_to_string("input/day18").expect("failure opening input file");

    let p1 = input.lines().map(|exp| shunting_yard(exp, process_p1)).sum::<usize>();
    println!("Part1: {}", p1);

    let p2 = input.lines().map(|exp| shunting_yard(exp, process_p2)).sum::<usize>();
    println!("Part2: {}", p2);
}

fn shunting_yard(exp: &str, process_op_fn: ProcessOpFn) -> usize {
    let tokens = exp.chars().filter(|c| !c.is_whitespace());
    let mut stack = vec![];
    let mut dequeue = VecDeque::new();
    for token in tokens {
        match token {
            '+' => process_op_fn(Token::Add, &mut stack, &mut dequeue),
            '*' => process_op_fn(Token::Mul, &mut stack, &mut dequeue),
            '(' => stack.push(Token::LeftP),
            ')' => loop {
                match stack.pop() {
                    Some(Token::LeftP) | None => break,
                    Some(top) => dequeue.push_back(top),
                }
            },
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

fn process_p1(token: Token, stack: &mut Vec<Token>, dequeue: &mut VecDeque<Token>) {
    while let Some(Token::Mul) | Some(Token::Add) = stack.last() {
        dequeue.push_back(stack.pop().unwrap())
    }
    stack.push(token);
}

fn process_p2(token: Token, stack: &mut Vec<Token>, dequeue: &mut VecDeque<Token>) {
    if let Token::Mul = token {
        while let Some(Token::Add) = stack.last() {
            dequeue.push_back(stack.pop().unwrap())
        }
    }
    stack.push(token);
}
