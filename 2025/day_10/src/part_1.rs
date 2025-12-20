use std::collections::{HashSet, VecDeque};

#[derive(Debug)]
struct Machine {
    lights: u16,
    buttons: Vec<u16>,
    _joltage: (),
}

fn parse_input(input: &str) -> anyhow::Result<Vec<Machine>> {
    input
        .lines()
        .filter_map(|s| {
            let s = s.trim();
            if !s.is_empty() { Some(s) } else { None }
        })
        .map(|s| {
            let mut lights = 0;
            let mut buttons = vec![];
            let mut _joltage = ();

            for s in s.split_whitespace() {
                if s.len() < 3 {
                    return Err(anyhow::anyhow!("Unexpected string: `{s}`"));
                }

                let mut chars = s.chars();

                let first = chars.next().unwrap();
                let last = chars.next_back().unwrap();
                let content = chars.collect::<String>();

                match (first, last) {
                    ('[', ']') => {
                        lights = content
                            .chars()
                            .map(|c| match c {
                                '.' => Ok(0),
                                '#' => Ok(1),
                                other => Err(anyhow::anyhow!("Unexpected character: {other}")),
                            })
                            .enumerate()
                            .try_fold(0, |acc, (index, e)| {
                                Ok::<_, anyhow::Error>(acc | (e? << index))
                            })?;
                    }
                    ('(', ')') => buttons.push(
                        content
                            .split(',')
                            .map(|s| s.parse::<u16>())
                            .try_fold(0, |acc, e| Ok::<_, anyhow::Error>(acc | 1 << e?))?,
                    ),
                    // We don't need this for now
                    ('{', '}') => (),
                    _ => return Err(anyhow::anyhow!("Unexpected string: `{s}`")),
                }
            }

            Ok(Machine {
                lights,
                buttons,
                _joltage,
            })
        })
        .collect::<anyhow::Result<Vec<_>>>()
}

pub fn solve(input: &str) -> anyhow::Result<()> {
    let machines = parse_input(input)?;

    let button_presses = machines
        .into_iter()
        .map(|m| {
            let Machine {
                lights, buttons, ..
            } = m;

            let mut steps = 0;
            let mut queue = VecDeque::from([(0, steps)]);
            let mut visited = HashSet::new();

            while let Some((state, step)) = queue.pop_front() {
                if state == lights {
                    steps = step;
                    break;
                }

                for &button in &buttons {
                    let next = state ^ button;

                    if visited.insert(next) {
                        queue.push_back((next, step + 1));
                    }
                }
            }

            steps
        })
        .sum::<u32>();

    println!("Answer: {}", button_presses);

    Ok(())
}
