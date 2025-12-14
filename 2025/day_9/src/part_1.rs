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
    for point1 @ &(x1, y1) in &points {
        for point2 @ &(x2, y2) in &points {
            if point1 != point2 {
                max_area = max_area.max((x1.abs_diff(x2) + 1) * (y1.abs_diff(y2) + 1));
            }
        }
    }

    println!("Answer: {}", max_area);

    Ok(())
}
