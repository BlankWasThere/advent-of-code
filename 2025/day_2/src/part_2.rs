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
            let num_vec = num.to_string().chars().collect::<Vec<_>>();

            // Create LPS table
            let mut lps = vec![0; num_vec.len()];
            let mut i = 1;
            let mut j = 0;

            while i < num_vec.len() {
                if num_vec[i] == num_vec[j] {
                    j += 1;
                    lps[i] = j;
                    i += 1;
                } else if j == 0 {
                    i += 1;
                } else {
                    j = lps[j - 1];
                }
            }

            // Find pattern length
            let last = lps[num_vec.len() - 1];
            let pattern_len = num_vec.len() - last;

            // Verification
            if pattern_len != num_vec.len() && num_vec.len().is_multiple_of(pattern_len) {
                invalid_ids.push(num);
            }
        }
    }

    println!("Answer: {}", invalid_ids.iter().sum::<u64>());

    Ok(())
}
