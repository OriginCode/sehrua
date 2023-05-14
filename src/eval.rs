use anyhow::{bail, Context, Result};

fn eval_exp(mut stack: Vec<f64>, ch: &str) -> Result<Vec<f64>> {
    match ch.parse::<f64>().ok() {
        Some(i) => stack.push(i),
        None => {
            let arg2 = stack
                .pop()
                .context("reached stack boundary, please check the expression")?;
            let arg1 = stack
                .pop()
                .context("reached stack boundary, please check the expression")?;
            match ch {
                "+" => stack.push(arg1 + arg2),
                "-" => stack.push(arg1 - arg2),
                "*" => stack.push(arg1 * arg2),
                "/" => stack.push(arg1 / arg2),
                "^" => stack.push(arg1.powf(arg2)),
                ch @ _ => panic!("Unsupported operand: {}", ch),
            }
        }
    }

    Ok(stack)
}

pub fn eval(input: impl AsRef<str>) -> Result<f64> {
    let ret = input
        .as_ref()
        .split_whitespace()
        .try_fold(Vec::new(), eval_exp)?;

    match ret.len() {
        1 => Ok(*ret.first().unwrap()),
        _ => bail!("illegal finalized stack, please check the expression"),
    }
}
