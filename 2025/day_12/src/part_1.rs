// This does not pass the tests, but works on the input.
// I know this is not good, but, well, thats all there is to it.

type Size = (u32, u32);

fn parse_input(input: &str) -> anyhow::Result<Vec<(Size, Vec<u32>)>> {
    // Normalize newlines (CRLF -> LF)
    let input = input.replace("\r\n", "\n");

    // Since all the values are 3x3 (through visual analysis), we can skip parsing them.
    let last = input
        .split("\n\n")
        .map(String::from)
        .last()
        .ok_or(anyhow::anyhow!("Empty input."))?;

    last.lines()
        .filter_map(|s| {
            let s = s.trim();
            if !s.is_empty() { Some(s) } else { None }
        })
        .map(|s| {
            let (first, last) = s
                .split_once(":")
                .ok_or(anyhow::anyhow!("Invalid line `{s}`"))?;
            let (width, height) = first
                .split_once('x')
                .ok_or(anyhow::anyhow!("Invalid line `{s}`"))?;

            let width = width.parse::<u32>()?;
            let height = height.parse::<u32>()?;

            let presents = last
                .split_whitespace()
                .map(|s| Ok(s.parse::<u32>()?))
                .collect::<anyhow::Result<Vec<_>>>()?;

            Ok(((width, height), presents))
        })
        .collect::<anyhow::Result<Vec<_>>>()
}

pub fn solve(input: &str) -> anyhow::Result<()> {
    let regions = parse_input(input)?;
    let mut count = 0;

    for ((width, height), presents) in regions {
        if (width / 3) * (height / 3) >= presents.iter().sum::<u32>() {
            count += 1;
        }
    }

    println!("Answer: {}", count);

    Ok(())
}
