use std::collections::VecDeque;
use std::fs;

#[derive(Debug)]
enum Token {
    Val(usize),
    Add,
    Mul,
    LeftP,
}

fn main() {
    let input = fs::read_to_string("input/day18").expect("failure opening input file");

    // let p1 = input.lines().map(|exp| shunting_yard(exp)).sum::<usize>();
    // println!("{}", p1);

    let p2 = input.lines().map(|exp| shunting_yard2(exp)).sum::<usize>();
    println!("{}", p2);
}

fn shunting_yard(exp: &str) -> usize {
    let mut tokens = exp.chars().filter(|c| !c.is_whitespace());
    let mut stack = vec![];
    let mut dequeue = VecDeque::new();
    while let Some(token) = tokens.next() {
        match token {
            '+' => {
                if let Some(Token::Mul) | Some(Token::Add) = stack.last() {
                    dequeue.push_back(stack.pop().unwrap())
                };
                stack.push(Token::Add)
            }
            '*' => {
                if let Some(Token::Mul) | Some(Token::Add) = stack.last() {
                    dequeue.push_back(stack.pop().unwrap())
                };
                stack.push(Token::Mul)
            }
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
                let lhs = match stack.pop() {
                    Some(Token::Val(lhs)) => lhs,
                    _ => 0,
                };
                let rhs = match stack.pop() {
                    Some(Token::Val(rhs)) => rhs,
                    _ => 0,
                };
                stack.push(Token::Val(lhs + rhs));
            }
            Token::Mul => {
                let lhs = match stack.pop() {
                    Some(Token::Val(lhs)) => lhs,
                    _ => 0,
                };
                let rhs = match stack.pop() {
                    Some(Token::Val(rhs)) => rhs,
                    _ => 0,
                };
                stack.push(Token::Val(lhs * rhs));
            }
            val => stack.push(val),
        }
    }

    match stack.pop() {
        Some(Token::Val(val)) => val,
        _ => 0,
    }
}

fn shunting_yard2(exp: &str) -> usize {
    let mut tokens = exp.chars().filter(|c| !c.is_whitespace());
    let mut stack = vec![];
    let mut dequeue = VecDeque::new();
    while let Some(token) = tokens.next() {
        match token {
            '+' => {
                // if let Some(Token::Mul) | Some(Token::Add) = stack.last() {
                //     dequeue.push_back(stack.pop().unwrap())
                // };
                stack.push(Token::Add)
            }
            '*' => {
                while let Some(Token::Add) = stack.last() {
                    dequeue.push_back(stack.pop().unwrap())
                }
                stack.push(Token::Mul)
            }
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
                let lhs = match stack.pop() {
                    Some(Token::Val(lhs)) => lhs,
                    _ => 0,
                };
                let rhs = match stack.pop() {
                    Some(Token::Val(rhs)) => rhs,
                    _ => 0,
                };
                stack.push(Token::Val(lhs + rhs));
            }
            Token::Mul => {
                let lhs = match stack.pop() {
                    Some(Token::Val(lhs)) => lhs,
                    _ => 0,
                };
                let rhs = match stack.pop() {
                    Some(Token::Val(rhs)) => rhs,
                    _ => 0,
                };
                stack.push(Token::Val(lhs * rhs));
            }
            val => stack.push(val),
        }
    }

    match stack.pop() {
        Some(Token::Val(val)) => val,
        _ => 0,
    }
}
