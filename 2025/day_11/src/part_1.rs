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

    let count = find_path_recursive(&devices, &"you".to_owned(), &mut HashSet::new())?;

    println!("Answer: {}", count);

    Ok(())
}

fn find_path_recursive<'a>(
    device_map: &'a HashMap<String, Vec<String>>,
    curr: &'a String,
    visited: &mut HashSet<&'a String>,
) -> anyhow::Result<u32> {
    let mut count = 0;

    if curr == "out" {
        return Ok(1);
    }

    if visited.insert(curr) {
        let outputs = device_map
            .get(curr)
            .ok_or(anyhow::anyhow!("Key `{curr}` not found in list."))?;

        for next in outputs {
            count += find_path_recursive(device_map, next, visited)?
        }

        visited.remove(curr);
    }

    Ok(count)
}
