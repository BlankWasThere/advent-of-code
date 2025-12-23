use std::collections::{HashMap, HashSet};

fn parse_input(input: &str) -> anyhow::Result<HashMap<String, Vec<String>>> {
    input
        .lines()
        .filter_map(|s| {
            let s = s.trim();
            if !s.is_empty() { Some(s) } else { None }
        })
        .map(|s| {
            let (key, values) = s
                .split_once(':')
                .ok_or(anyhow::anyhow!("Invalid line `{s}`."))?;
            let values = values
                .split_whitespace()
                .map(String::from)
                .collect::<Vec<_>>();

            Ok((key.to_owned(), values))
        })
        .collect::<anyhow::Result<HashMap<_, _>>>()
}

pub fn solve(input: &str) -> anyhow::Result<()> {
    let devices = parse_input(input)?;

    let count = find_path_recursive(
        &devices,
        &"svr".to_owned(),
        &mut HashSet::new(),
        &mut HashMap::new(),
    )?;

    println!("Answer: {}", count);

    Ok(())
}

fn find_path_recursive<'a>(
    device_map: &'a HashMap<String, Vec<String>>,
    curr: &'a String,
    visited: &mut HashSet<&'a String>,
    memo: &mut HashMap<(bool, bool, &'a String), u64>,
) -> anyhow::Result<u64> {
    let visited_dac = visited.contains(&"dac".to_owned());
    let visited_fft = visited.contains(&"fft".to_owned());

    if curr == "out" {
        if visited_dac && visited_fft {
            return Ok(1);
        } else {
            return Ok(0);
        }
    }

    if let Some(&v) = memo.get(&(visited_dac, visited_fft, curr)) {
        return Ok(v);
    }

    let mut count = 0;
    if visited.insert(curr) {
        let outputs = device_map
            .get(curr)
            .ok_or(anyhow::anyhow!("Key `{curr}` not found in list."))?;

        for next in outputs {
            count += find_path_recursive(device_map, next, visited, memo)?
        }

        visited.remove(curr);
    }

    memo.insert((visited_dac, visited_fft, curr), count);
    Ok(count)
}
