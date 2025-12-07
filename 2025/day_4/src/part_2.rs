fn can_access(grid: &[Vec<bool>], target_row: usize, target_col: usize) -> bool {
    let mut nearby_rolls = 0;

    let row_start = target_row.saturating_sub(1);
    let row_end = (target_row + 1).min(grid.len() - 1);

    for (row_idx, row) in grid
        .iter()
        .enumerate()
        .skip(row_start)
        .take(row_end - row_start + 1)
    {
        let col_start = target_col.saturating_sub(1);
        let col_end = (target_col + 1).min(row.len() - 1);

        for (col_idx, &value) in row
            .iter()
            .enumerate()
            .skip(col_start)
            .take(col_end - col_start + 1)
        {
            // Ignore the middle (current) cell.
            if value && (row_idx != target_row || col_idx != target_col) {
                nearby_rolls += 1;
            }
        }
    }

    nearby_rolls < 4
}

fn parse_input(input: &str) -> anyhow::Result<Vec<Vec<bool>>> {
    input
        .lines()
        .filter_map(|s| {
            let s = s.trim();
            if !s.is_empty() { Some(s) } else { None }
        })
        .map(|s| {
            s.chars()
                .map(|c| match c {
                    // This is a paper roll
                    '@' => Ok(true),
                    // This is not a paper roll
                    '.' => Ok(false),
                    // Tf is this?
                    other => Err(anyhow::anyhow!("Invalid character `{other}`.")),
                })
                .collect::<anyhow::Result<Vec<bool>>>()
        })
        .collect::<anyhow::Result<Vec<_>>>()
}

pub fn solve(input: &str) -> anyhow::Result<()> {
    let mut grid = parse_input(input)?;
    let mut can_be_removed = 0;

    loop {
        let mut to_remove = Vec::new();

        for row in 0..grid.len() {
            for col in 0..grid[row].len() {
                if grid[row][col] && can_access(&grid, row, col) {
                    to_remove.push((row, col));
                }
            }
        }

        // No more rolls can be removed
        if to_remove.is_empty() {
            break;
        }

        can_be_removed += to_remove.len();

        // Remove paper rolls
        for (row, col) in to_remove {
            grid[row][col] = false;
        }
    }

    println!("Answer: {}", can_be_removed);

    Ok(())
}
