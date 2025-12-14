use std::collections::HashSet;

macro_rules! stable_min_max {
    ($a:expr, $b:expr) => {
        if ($a <= $b) { ($a, $b) } else { ($b, $a) }
    };
}

fn parse_input(input: &str) -> anyhow::Result<Vec<(i64, i64)>> {
    input
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

    let (mut xs, mut ys): (Vec<i64>, Vec<i64>) = points.iter().copied().unzip();

    // Sort and remove duplicate points
    xs.sort_unstable();
    xs.dedup();

    ys.sort_unstable();
    ys.dedup();

    let mut grid = vec![vec![0u8; xs.len()]; ys.len()];

    // Fill red tiles using compressed coordinates
    for &(x, y) in &points {
        let tx = xs.binary_search(&x).unwrap();
        let ty = ys.binary_search(&y).unwrap();

        grid[ty][tx] = 1;
    }

    // Fill boundary green tiles
    for (point1 @ &(x1, y1), point2 @ &(x2, y2)) in points
        .iter()
        .zip(points.iter().cycle().skip(1).take(points.len()))
    {
        let cx1 = xs.binary_search(&x1).unwrap();
        let cy1 = ys.binary_search(&y1).unwrap();

        let cx2 = xs.binary_search(&x2).unwrap();
        let cy2 = ys.binary_search(&y2).unwrap();

        // Normalize the points
        let (cx1, cx2) = stable_min_max!(cx1, cx2);
        let (cy1, cy2) = stable_min_max!(cy1, cy2);

        if point1 != point2 {
            #[allow(clippy::needless_range_loop)]
            if cx1 == cx2 {
                for y in cy1 + 1..cy2 {
                    grid[y][cx1] = 1;
                }
            } else if cy1 == cy2 {
                for x in cx1 + 1..cx2 {
                    grid[cy1][x] = 1;
                }
            } else {
                return Err(anyhow::anyhow!(
                    "Invalid points: {point1:?} and {point2:?} cannot be connected."
                ));
            }
        }
    }

    // Get all outside points
    let mut outside = HashSet::new();
    let mut stack = vec![(-1, -1)];

    while let Some((x, y)) = stack.pop() {
        // One width border padding (always outside)
        if y < -1 || y > grid.len() as i64 || x < -1 || x > grid[0].len() as i64 {
            continue;
        }

        // If boundary, do not proceed
        if y >= 0
            && y < grid.len() as i64
            && x >= 0
            && x < grid[y as usize].len() as i64
            && grid[y as usize][x as usize] == 1
        {
            continue;
        }

        let directions = [(0, -1), (0, 1), (-1, 0), (1, 0)];

        for (dx, dy) in directions {
            let new_point = (x + dx, y + dy);

            if !outside.contains(&new_point) {
                stack.push(new_point);
            }
        }

        outside.insert((x, y));
    }

    // Fill inner points
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if !outside.contains(&(x as i64, y as i64)) {
                grid[y][x] = 1;
            }
        }
    }

    // Build psa for grid
    let mut psa = vec![vec![0u64; xs.len()]; ys.len()];
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            let left = if x > 0 { psa[y][x - 1] } else { 0 };
            let top = if y > 0 { psa[y - 1][x] } else { 0 };
            let topleft = if y > 0 && x > 0 { psa[y - 1][x - 1] } else { 0 };

            psa[y][x] = grid[y][x] as u64 + left + top - topleft;
        }
    }

    // Find max area
    let mut max_area = 0;
    for point1 @ &(x1, y1) in &points {
        for point2 @ &(x2, y2) in &points {
            if point1 == point2 {
                continue;
            }

            let cx1 = xs.binary_search(&x1).unwrap();
            let cy1 = ys.binary_search(&y1).unwrap();
            let cx2 = xs.binary_search(&x2).unwrap();
            let cy2 = ys.binary_search(&y2).unwrap();

            // Normalize the points
            let (cx1, cx2) = stable_min_max!(cx1, cx2);
            let (cy1, cy2) = stable_min_max!(cy1, cy2);

            let c_area = (cx1.abs_diff(cx2) + 1) * (cy1.abs_diff(cy2) + 1);
            let c_area_from_psa = psa[cy2][cx2]
                + if cx1 > 0 && cy1 > 0 {
                    psa[cy1 - 1][cx1 - 1]
                } else {
                    0
                }
                - if cy1 > 0 { psa[cy1 - 1][cx2] } else { 0 }
                - if cx1 > 0 { psa[cy2][cx1 - 1] } else { 0 };

            if c_area == c_area_from_psa as usize {
                max_area = max_area.max((x1.abs_diff(x2) + 1) * (y1.abs_diff(y2) + 1));
            }
        }
    }

    println!("Answer: {}", max_area);

    Ok(())
}
