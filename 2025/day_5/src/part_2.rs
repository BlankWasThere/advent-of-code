fn parse_input(input: &str) -> anyhow::Result<Vec<(u64, u64)>> {
    let mut ranges = Vec::new();

    for line in input.trim().lines() {
        let line = line.trim();

        // Previous separator between ranges and inputs. Now considered EOF
        if line.is_empty() {
            break;
        }

        let Some((start, end)) = line.split_once("-") else {
            return Err(anyhow::anyhow!("Invalid line `{line}`: not a range."));
        };

        ranges.push((start.parse()?, end.parse()?));

        if line.is_empty() {
            break;
        }
    }

    Ok(ranges)
}

pub fn solve(input: &str) -> anyhow::Result<()> {
    let mut ranges = parse_input(input)?;
    let mut fresh_veggies_count = 0;

    // Sort by start time
    ranges.sort_by(|a, b| a.0.cmp(&b.0));

    let mut last_end = 0;
    for (mut start, end) in ranges {
        if start <= last_end {
            // If previous range completely overlaps this one, then ignore this one.
            if end <= last_end {
                continue;
            } else {
                start = last_end + 1;
            }
        }

        fresh_veggies_count += end - start + 1;
        last_end = end;
    }

    println!("Answer: {fresh_veggies_count}");

    Ok(())
}
