use anyhow::{bail, Result};
use itertools::Itertools;
use std::{fmt, vec::Drain};

enum Token {
    Const(f64),
    Bool(bool),
    Str(String),
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Token::*;
        match self {
            Const(i) => write!(f, "{i}"),
            Bool(b) => write!(f, "{b}"),
            Str(s) => write!(f, "{s}"),
        }
    }
}

fn drain_args(stack: &mut Vec<Token>, n: usize) -> Result<Drain<Token>> {
    if stack.get((stack.len() - n)..).is_some() {
        Ok(stack.drain((stack.len() - n)..))
    } else {
        bail!("reached stack boundary, please check the expression");
    }
}

fn eval_exp(mut stack: Vec<Token>, s: &str) -> Result<Vec<Token>> {
    use Token::*;
    if let Some(i) = s.parse::<f64>().ok() {
        stack.push(Const(i));
    } else if let Some(b) = s.parse::<bool>().ok() {
        stack.push(Bool(b));
    } else {
        match s {
            op @ ("+" | "-" | "*" | "/" | "^" | "=" | ">" | "<" | ">=" | "<=" | "<>") => {
                let (arg1, arg2) = match drain_args(&mut stack, 2)?.collect_tuple().unwrap() {
                    (Const(i1), Const(i2)) => (i1, i2),
                    _ => bail!("illegal stack, please check the expression"),
                };

                match op {
                    "+" => stack.push(Const(arg1 + arg2)),
                    "-" => stack.push(Const(arg1 - arg2)),
                    "*" => stack.push(Const(arg1 * arg2)),
                    "/" => stack.push(Const(arg1 / arg2)),
                    "^" => stack.push(Const(arg1.powf(arg2))),
                    "=" => stack.push(Bool(arg1 == arg2)),
                    ">" => stack.push(Bool(arg1 > arg2)),
                    "<" => stack.push(Bool(arg1 < arg2)),
                    ">=" => stack.push(Bool(arg1 >= arg2)),
                    "<=" => stack.push(Bool(arg1 <= arg2)),
                    "<>" => stack.push(Bool(arg1 != arg2)),
                    _ => unreachable!(),
                }
            }
            "IFTE" => {
                let (arg1, arg2, arg3) = match drain_args(&mut stack, 3)?.collect_tuple().unwrap() {
                    (Bool(b), v1 @ _, v2 @ _) => (b, v1, v2),
                    _ => bail!("illegal stack, please check the expression"),
                };
                if arg1 {
                    stack.push(arg2);
                } else {
                    stack.push(arg3);
                }
            }
            s @ _ => stack.push(Str(s.to_owned())),
        }
    }

    Ok(stack)
}

pub fn eval(input: impl AsRef<str>) -> Result<String> {
    let ret = input
        .as_ref()
        .split_whitespace()
        .try_fold(Vec::new(), eval_exp)?;

    match ret.len() {
        1 => Ok(ret.first().unwrap().to_string()),
        _ => bail!("illegal finalized stack, please check the expression"),
    }
}
