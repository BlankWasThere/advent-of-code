fn parse_input(input: &str) -> anyhow::Result<(usize, Vec<Vec<usize>>)> {
    let mut lines = input.trim().lines();
    let (spawn, _) = lines
        .next()
        .ok_or(anyhow::anyhow!("Empty input"))?
        .chars()
        .enumerate()
        .find(|&(_, c)| c == 'S')
        .ok_or(anyhow::anyhow!("Missing spawn in input"))?;

    let splitters = lines
        .skip(1)
        .step_by(2)
        .map(|s| s.match_indices('^').map(|(idx, _)| idx).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    Ok((spawn, splitters))
}

pub fn solve(input: &str) -> anyhow::Result<()> {
    let (spawn, splitters) = parse_input(input)?;

    let mut total_splits = 0;
    let mut current_beams = vec![spawn];

    for row in splitters {
        let mut next_beams = Vec::new();

        for beam in current_beams {
            if row.binary_search(&beam).is_ok() {
                // Beam gets split into two.
                let left_beam = beam - 1;
                let right_beam = beam + 1;

                // Only one beam per column.
                if Some(&left_beam) != next_beams.last() {
                    next_beams.push(left_beam);
                }

                next_beams.push(right_beam);
                total_splits += 1;
            } else {
                // Beam doesn't split and continues...
                if Some(&beam) != next_beams.last() {
                    next_beams.push(beam);
                }
            }
        }

        current_beams = next_beams;
    }

    println!("Answer: {}", total_splits);

    Ok(())
}
