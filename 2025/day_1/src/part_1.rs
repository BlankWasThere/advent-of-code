enum Direction {
    L(i32),
    R(i32),
}

impl std::str::FromStr for Direction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();

        match chars.next() {
            Some('L') => Ok(Direction::L(chars.collect::<String>().parse()?)),
            Some('R') => Ok(Direction::R(chars.collect::<String>().parse()?)),
            Some(c) => Err(anyhow::anyhow!("Invalid instruction `{}`.", c)),
            None => Err(anyhow::anyhow!("Cannot parse empty string.")),
        }
    }
}

fn parse_input(input: &str) -> anyhow::Result<Vec<Direction>> {
    input
        .lines()
        .filter_map(|s| {
            let s = s.trim();
            if !s.is_empty() { Some(s) } else { None }
        })
        .map(|s| s.parse::<Direction>())
        .collect::<anyhow::Result<Vec<_>>>()
}

pub fn solve(input: &str) -> anyhow::Result<()> {
    const MAX: i32 = 99;

    let instructions = parse_input(input)?;

    // Solving logic
    let mut zeros_count = 0;
    let mut current = 50;
    for instruction in instructions {
        match instruction {
            Direction::L(count) => {
                current = (current + (MAX + 1) - (count % (MAX + 1))) % (MAX + 1)
            }
            Direction::R(count) => current = (current + count) % (MAX + 1),
        }

        if current == 0 {
            zeros_count += 1;
        }
    }

    println!("Answer: {}", zeros_count);

    Ok(())
}
