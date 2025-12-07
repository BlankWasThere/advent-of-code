struct Problem {
    values: Vec<Vec<u64>>,
    operations: Vec<Operation>,
}
enum Operation {
    Multiply,
    Add,
}

fn parse_input(input: &str) -> anyhow::Result<Problem> {
    let mut lines = input
        .trim()
        .lines()
        .filter_map(|s| {
            let s = s.trim();
            if !s.is_empty() { Some(s) } else { None }
        })
        .rev();

    let operations = lines
        .next()
        .ok_or(anyhow::anyhow!("Empty input."))?
        .split_whitespace()
        .map(|s| match s {
            "*" => Ok(Operation::Multiply),
            "+" => Ok(Operation::Add),
            e => Err(anyhow::anyhow!("Invalid operator `{e}`.")),
        })
        .collect::<anyhow::Result<Vec<_>>>()?;

    // Parse remaining lines
    let values = lines
        .map(|l| {
            l.split_whitespace()
                .map(|s| s.parse::<u64>().map_err(Into::into))
                .collect::<anyhow::Result<Vec<_>>>()
        })
        .collect::<anyhow::Result<Vec<Vec<_>>>>()?;

    Ok(Problem { values, operations })
}

pub fn solve(input: &str) -> anyhow::Result<()> {
    let problem = parse_input(input)?;
    let mut columns = vec![0; problem.operations.len()];

    for row in problem.values {
        for (i, &v) in row.iter().enumerate() {
            match problem.operations.get(i) {
                Some(Operation::Add) => columns[i] += v,
                Some(Operation::Multiply) => {
                    columns[i] = if columns[i] == 0 { v } else { columns[i] * v };
                }
                None => return Err(anyhow::anyhow!("More operands than operators")),
            }
        }
    }

    let total = columns.iter().sum::<u64>();
    println!("Answer: {}", total);

    Ok(())
}
