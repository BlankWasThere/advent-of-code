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
    let total = parse_input(input)?
        .into_iter()
        .map(|bank| {
            bank.into_iter()
                .map(Into::into)
                .reduce(|acc, digit: u32| {
                    acc.max((acc % 10) * 10 + digit)
                        .max((acc / 10) * 10 + digit)
                })
                .unwrap_or(0)
        })
        .sum::<u32>();

    println!("Answer: {}", total);
    Ok(())
}
