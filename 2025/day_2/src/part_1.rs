struct Ranges {
    start: u64,
    end: u64,
}

fn parse_input(input: &str) -> anyhow::Result<Vec<Ranges>> {
    input
        .split(',')
        .filter_map(|s| {
            let s = s.trim();
            if !s.is_empty() { Some(s) } else { None }
        })
        .map(|s| {
            let (range_start, range_end) = s
                .split_once('-')
                .ok_or(anyhow::anyhow!("Invalid range: `{s}`"))?;
            Ok(Ranges {
                start: range_start.parse()?,
                end: range_end.parse()?,
            })
        })
        .collect::<anyhow::Result<Vec<_>>>()
}

pub fn solve(input: &str) -> anyhow::Result<()> {
    let ranges = parse_input(input)?;
    let mut invalid_ids = Vec::new();

    // Solving logic
    for range in ranges {
        let Ranges { start, end } = range;

        for num in start..=end {
            let num_str = num.to_string();

            if !num_str.len().is_multiple_of(2) {
                continue;
            }

            let (first, last) = num_str.split_at(num_str.len() / 2);

            // Validity check
            if first == last {
                invalid_ids.push(num);
            }
        }
    }

    println!("Answer: {}", invalid_ids.iter().sum::<u64>());

    Ok(())
}
