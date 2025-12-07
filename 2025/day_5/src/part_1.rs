type Ranges = Vec<(u64, u64)>;
type RangesInputsTuple = (Ranges, Vec<u64>);

fn parse_input(input: &str) -> anyhow::Result<RangesInputsTuple> {
    let mut state = false; // false = ranges | true = values
    let mut ranges = Vec::new();
    let mut values = Vec::new();

    for line in input.trim().lines() {
        let line = line.trim();

        // Separator for switching state
        if line.is_empty() {
            state = true;
            continue;
        }

        match state {
            // Read ranges
            false => {
                let Some((start, end)) = line.split_once("-") else {
                    return Err(anyhow::anyhow!("Invalid line `{line}`: not a range."));
                };

                ranges.push((start.parse()?, end.parse()?));
            }
            true => {
                values.push(line.parse()?);
            }
        }
    }

    Ok((ranges, values))
}

pub fn solve(input: &str) -> anyhow::Result<()> {
    let (ranges, values) = parse_input(input)?;

    let fresh_veggies_count = values
        .into_iter()
        .filter(|&n| ranges.iter().any(|&(start, end)| n >= start && n <= end))
        .count();

    println!("Answer: {fresh_veggies_count}");

    Ok(())
}
