use num_traits;
use rustyline::error::ReadlineError;
use rustyline::Editor;
use colored::Colorize;

fn evaluate<T>(expression: &str) -> Result<T, String>
where
    T: num_traits::Num,
    T: std::str::FromStr,
    T: std::fmt::Display,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
    T: Copy,
    T: std::fmt::Debug
{
    let mut stack = Vec::new();

    for word in expression.split_whitespace() {
        match word {
            "+" => {
                let operand2 = stack
                    .pop()
                    .ok_or(format!("2 operands are needed before '{word}'"))?;
                let operand1 = stack
                    .pop()
                    .ok_or(format!("1 operand is needed before '{word}'"))?;
                let result = operand1 + operand2;
                println!("\t{operand1} {word} {operand2} = {result}");
                stack.push(result);
            },
            "-" => {
                let operand2 = stack
                    .pop()
                    .ok_or(format!("2 operands are needed before '{word}'"))?;
                let operand1 = stack
                    .pop()
                    .ok_or(format!("1 operand is needed before '{word}'"))?;
                let result = operand1 - operand2;
                println!("\t{operand1} {word} {operand2} = {result}");
                stack.push(result);
            },
            "*" => {
                let operand2 = stack
                    .pop()
                    .ok_or(format!("2 operands are needed before '{word}'"))?;
                let operand1 = stack
                    .pop()
                    .ok_or(format!("1 operand is needed before '{word}'"))?;
                let result = operand1 * operand2;
                println!("\t{operand1} {word} {operand2} = {result}");
                stack.push(result);
            },
            "/" => {
                let operand2 = stack
                    .pop()
                    .ok_or(format!("2 operands are needed before '{word}'"))?;
                let operand1 = stack
                    .pop()
                    .ok_or(format!("1 operand is needed before '{word}'"))?;
                let result = operand1 / operand2;
                println!("\t{operand1} {word} {operand2} = {result}");
                stack.push(result);
            },
            "%" => {
                let operand2 = stack
                    .pop()
                    .ok_or(format!("2 operands are needed before '{word}'"))?;
                let operand1 = stack
                    .pop()
                    .ok_or(format!("1 operand is needed before '{word}'"))?;
                let result = operand1 % operand2;
                println!("\t{operand1} {word} {operand2} = {result}");
                stack.push(result);
            }
            _ => {
                stack.push(word.parse::<T>().map_err(|_| format!("'{word}' cannot be parsed into a number!"))?);
            }
        }
    }

    if stack.len() > 1 {
        let s = format!("Skipped the operand(s) {:?}", &stack[..stack.len()-1]);
        println!("{}", s.yellow());
    }

    Ok(stack.pop().unwrap_or(T::zero()))
}

fn main() -> rustyline::Result<()> {
    // `()` can be used when no completer is required
    let mut rl = Editor::<()>::new()?;
    if rl.load_history("history.txt").is_err() {
        // println!("No previous history.");
    }

    println!("{}", "CODOTAKU RPN 2022".blue());

    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                if line.is_empty() {
                    continue;
                }
                rl.add_history_entry(line.as_str());
                let (typ, expression) = line.split_once(':').unwrap_or(("float", &line));
                match typ {
                    "float" => {
                        let result = evaluate::<f64>(expression);
                        match result {
                            Ok(result) => println!("\t= {result}"),
                            Err(error_message) => println!("Error: {}", error_message.red())
                        }
                    }
                    "int" => {
                        let result = evaluate::<i64>(expression);
                        match result {
                            Ok(result) => println!("\t= {result}"),
                            Err(error_message) => println!("Error: {}", error_message.red())
                        }
                    }
                    _ => {
                        println!("Unimplemented type: {}", typ);
                    }
                };
            }
            Err(ReadlineError::Interrupted) => {
                // println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                // println!("CTRL-D");
                break;
            }
            Err(_err) => {
                // println!("Error: {:?}", err);
                break;
            }
        }
    }
    rl.save_history("history.txt")
}
