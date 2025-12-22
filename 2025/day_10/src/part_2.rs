// Thanks to u/tenthmascot for this cool solution.
// Post: https://www.reddit.com/r/adventofcode/comments/1pk87hl/2025_day_10_part_2_bifurcate_your_way_to_victory/

use std::collections::HashMap;

type Joltage = Vec<u16>;
type Button = Vec<usize>;

#[derive(Debug)]
struct Machine {
    _lights: (),
    buttons: Vec<Button>,
    joltage: Joltage,
}

fn parse_input(input: &str) -> anyhow::Result<Vec<Machine>> {
    input
        .lines()
        .filter_map(|s| {
            let s = s.trim();
            if !s.is_empty() { Some(s) } else { None }
        })
        .map(|s| {
            let mut _lights = ();
            let mut buttons = vec![];
            let mut joltage = vec![];

            for s in s.split_whitespace() {
                if s.len() < 3 {
                    return Err(anyhow::anyhow!("Unexpected string: `{s}`"));
                }

                let mut chars = s.chars();

                let first = chars.next().unwrap();
                let last = chars.next_back().unwrap();
                let content = chars.collect::<String>();

                match (first, last) {
                    // We don't this this for now
                    ('[', ']') => (),
                    ('(', ')') => buttons.push(
                        content
                            .split(',')
                            .map(|s| Ok(s.parse::<usize>()?))
                            .collect::<anyhow::Result<Vec<_>>>()?,
                    ),
                    ('{', '}') => {
                        joltage.extend_from_slice(
                            &content
                                .split(',')
                                .map(|s| Ok(s.parse()?))
                                .collect::<anyhow::Result<Vec<_>>>()?,
                        );
                    }
                    _ => return Err(anyhow::anyhow!("Unexpected string: `{s}`")),
                }
            }

            Ok(Machine {
                _lights,
                buttons,
                joltage,
            })
        })
        .collect::<anyhow::Result<Vec<_>>>()
}

pub fn solve(input: &str) -> anyhow::Result<()> {
    let machines = parse_input(input)?;

    let button_presses = machines
        .into_iter()
        .filter_map(|m| {
            solve_machine(
                m.joltage,
                &m.buttons,
                &mut HashMap::new(),
                &mut HashMap::new(),
            )
        })
        .sum::<u32>();

    println!("Answer: {}", button_presses);

    Ok(())
}

fn solve_machine(
    joltage: Joltage,
    buttons: &[Button],
    memo: &mut HashMap<Joltage, u32>,
    parity_cache: &mut HashMap<u32, Vec<u32>>,
) -> Option<u32> {
    // Base case
    if joltage.iter().all(|&x| x == 0) {
        return Some(0);
    }

    // Check for memoization
    if let Some(&v) = memo.get(&joltage) {
        return Some(v);
    }

    // Assuming the buttons still toggle the lights, even presses cancel out
    // whereas odd presses toggle the light.
    // So create light indicators for the joltage (based on parity)
    let indicators = joltage
        .iter()
        .enumerate()
        .filter(|&(_, &x)| !x.is_multiple_of(2))
        .fold(0, |acc, (i, _)| acc | 1 << i);

    let button_masks = buttons
        .iter()
        .map(|btn| btn.iter().fold(0, |acc, e| acc | (1 << e)))
        .collect::<Vec<_>>();

    // Find all the possible button presses that give the above
    // light indicators and memoize for later use.
    let presses = if let Some(p) = parity_cache.get(&indicators) {
        p
    } else {
        let mut p = vec![];

        {
            let mut stack = vec![(0u32, 0u32, 0usize)];

            while let Some((state, pressed_buttons, curr_index)) = stack.pop() {
                if curr_index >= buttons.len() {
                    if state == indicators {
                        p.push(pressed_buttons);
                    }

                    continue;
                }

                // Do not press the button
                stack.push((state, pressed_buttons, curr_index + 1));

                // Press the button
                let new_state = state ^ button_masks[curr_index];
                let pressed_buttons = pressed_buttons | (1 << curr_index);
                stack.push((new_state, pressed_buttons, curr_index + 1));
            }
        }

        parity_cache.insert(indicators, p);
        parity_cache.get(&indicators).unwrap()
    };

    // For each solution, we have to press an unknown (but even) number of some other buttons as well.
    // So lets first reduce the joltage requirement for each of the above solutions.
    let new_joltages = presses
        .iter()
        .filter_map(|&btns| {
            let mut joltage = joltage.clone();

            for (index, btn) in buttons.iter().enumerate() {
                if btns & (1u32 << index) != 0 {
                    for &index in btn {
                        joltage[index] = match joltage[index].checked_sub(1) {
                            Some(v) => v,
                            None => return None,
                        }
                    }
                }
            }

            // Since all the joltage should be even now, we can divide by 2 to reduce the problem more
            for joltage in &mut joltage {
                *joltage /= 2;
            }

            Some((btns.count_ones(), joltage))
        })
        .collect::<Vec<_>>();

    // Recurse with the new joltages and find minimum
    let result = new_joltages
        .into_iter()
        .filter_map(|(steps, joltage)| {
            Some(2 * solve_machine(joltage, buttons, memo, parity_cache)? + steps)
        })
        .min();

    if let Some(v) = result {
        memo.insert(joltage, v);
    }

    result
}
