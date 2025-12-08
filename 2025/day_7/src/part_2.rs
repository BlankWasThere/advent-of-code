use std::collections::HashMap;

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

    // Memoization for optimization
    let mut memo = HashMap::new();
    let total_splits = 1 + split_timeline(spawn, &splitters, 0, &mut memo);
    println!("Answer: {}", total_splits);

    Ok(())
}

fn split_timeline(
    beam: usize,
    splitters: &[Vec<usize>],
    curr_index: usize,
    memo: &mut HashMap<(usize, usize), u64>,
) -> u64 {
    if let Some(row) = splitters.get(curr_index) {
        if let Some(&v) = memo.get(&(curr_index, beam)) {
            return v;
        }

        let res: u64 = if row.binary_search(&beam).is_ok() {
            1 + split_timeline(beam - 1, splitters, curr_index + 1, memo)
                + split_timeline(beam + 1, splitters, curr_index + 1, memo)
        } else {
            // No splitting, continue...
            split_timeline(beam, splitters, curr_index + 1, memo)
        };

        // Save result for later
        memo.insert((curr_index, beam), res);

        res
    } else {
        // No more rows
        0
    }
}
