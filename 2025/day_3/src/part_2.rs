fn parse_input(input: &str) -> anyhow::Result<Vec<Vec<u8>>> {
    input
        .lines()
        .filter_map(|s| {
            let s = s.trim();
            if !s.is_empty() { Some(s) } else { None }
        })
        .map(|s| {
            s.chars()
                .map(|c| {
                    c.to_digit(10)
                        .map(|d| d as u8) // To save some space :)
                        .ok_or(anyhow::anyhow!("`{c}` is not a valid digit"))
                })
                .collect::<anyhow::Result<Vec<_>>>()
        })
        .collect::<anyhow::Result<Vec<Vec<_>>>>()
}

pub fn solve(input: &str) -> anyhow::Result<()> {
    const DIGITS_PER_BANK: usize = 12;

    let total = parse_input(input)?
        .into_iter()
        .map(|bank| {
            let mut stack = Vec::new();
            let mut drops = bank.len().saturating_sub(DIGITS_PER_BANK);

            for digit in bank {
                let digit = digit as u64;

                // Check if current digit is greater than the last digit, and whether we can drop the the latter.
                while let Some(&last_digit) = stack.last()
                    && digit > last_digit
                    && drops > 0
                {
                    stack.pop();
                    drops -= 1;
                }

                stack.push(digit);
            }

            stack.truncate(DIGITS_PER_BANK);

            stack.into_iter().reduce(|acc, e| acc * 10 + e).unwrap_or(0)
        })
        .sum::<u64>();

    println!("Answer: {}", total);
    Ok(())
}
