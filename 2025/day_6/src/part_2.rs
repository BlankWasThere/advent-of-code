//! # Warning
//! Make sure to disable trimming of input.txt file as it would disrupt the pattern.

#[derive(Debug)]
struct Column {
    operation: Operation,
    length: u8,
    values: Vec<Vec<Option<u8>>>,
}

#[derive(Debug)]
enum Operation {
    Multiply,
    Add,
}

fn parse_input(input: &str) -> anyhow::Result<Vec<Column>> {
    let mut lines = input.lines();
    let mut lengths = Vec::new();

    // From the last line parse operations
    let operations = lines
        .rfind(|l| !l.trim().is_empty())
        .ok_or(anyhow::anyhow!("Missing operation line."))?
        .split(' ')
        .filter_map(|s| {
            if s != "" {
                // Basically push(1u8) but more idiomatic
                lengths.push(s.len() as u8);
            }

            match s {
                "*" => Some(Ok(Operation::Multiply)),
                "+" => Some(Ok(Operation::Add)),
                "" => {
                    if let Some(last) = lengths.last_mut() {
                        *last += 1;
                    }
                    None
                }
                other => Some(Err(anyhow::anyhow!(
                    "Invalid string `{other}`: not a valid operator."
                ))),
            }
        })
        .collect::<anyhow::Result<Vec<_>>>()?;

    let mut columns = operations
        .into_iter()
        .zip(lengths.into_iter())
        .map(|(operation, length)| Column {
            operation,
            length: length,
            values: Vec::new(),
        })
        .collect::<Vec<_>>();

    // Now parse values
    for line in lines {
        if line.trim().is_empty() {
            continue;
        }

        let mut chars = line.chars();
        for &mut Column {
            length,
            ref mut values,
            ..
        } in columns.iter_mut()
        {
            let mut v = Vec::new();
            for _ in 0..length {
                v.push(match chars.next() {
                    Some(' ') => None,
                    Some(c) => c
                        .to_digit(10)
                        .map(|d| Some(d as u8))
                        .ok_or(anyhow::anyhow!("Invalid char `{c}`: not a digit."))?,
                    None => None,
                });
            }

            // Skip one space
            _ = chars.next();

            values.push(v);
        }
    }

    Ok(columns)
}

pub fn solve(input: &str) -> anyhow::Result<()> {
    let columns = parse_input(input)?;

    let total = columns
        .into_iter()
        .map(|column| {
            column
                .values
                .into_iter()
                .fold(vec![None; column.length as usize], |acc, e| {
                    acc.into_iter()
                        .map(|e| e.map(|v| v as u64))
                        .zip(e.into_iter().map(|e| e.map(|v| v as u64)))
                        .map(|(a, b)| match (a, b) {
                            (Some(a), Some(b)) => Some(a * 10 + b),
                            (Some(a), None) => Some(a),
                            (None, Some(b)) => Some(b),
                            (None, None) => None,
                        })
                        .collect::<Vec<_>>()
                })
                .into_iter()
                .filter_map(|e| if let Some(v) = e { Some(v) } else { None })
                .reduce(|acc, e| match column.operation {
                    Operation::Multiply => acc * e,
                    Operation::Add => acc + e,
                })
                .unwrap_or(0)
        })
        .sum::<u64>();

    println!("Answer: {}", total);
    Ok(())
}
