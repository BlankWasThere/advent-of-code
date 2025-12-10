fn parse_input(input: &str) -> anyhow::Result<Vec<(u64, u64)>> {
    input
        .trim()
        .lines()
        .filter_map(|s| {
            let s = s.trim();
            if !s.is_empty() { Some(s) } else { None }
        })
        .map(|s| {
            let &[x, y] = s
                .splitn(2, ',')
                .map(|s| Ok(s.parse()?))
                .collect::<anyhow::Result<Vec<_>>>()?
                .as_slice()
            else {
                return Err(anyhow::anyhow!("Invalid grid index: `{s}`"));
            };
            Ok((x, y))
        })
        .collect()
}

pub fn solve(input: &str) -> anyhow::Result<()> {
    let points = parse_input(input)?;

    let mut max_area = 0;
    for &x in &points {
        for &y in &points {
            if x != y {
                max_area = max_area.max((x.0.abs_diff(y.0) + 1) * (x.1.abs_diff(y.1) + 1));
            }
        }
    }

    println!("Answer: {}", max_area);

    Ok(())
}
