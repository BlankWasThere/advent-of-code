mod part_1;
mod part_2;

fn main() -> anyhow::Result<()> {
    const INPUT_FILE: &str = "input.txt";

    let input = std::fs::read_to_string(INPUT_FILE)?;

    // Solve part 1
    println!("+=== Part 1 ===+");
    part_1::solve(&input)?;

    println!();

    // Solve part 2
    println!("+=== Part 2 ===+");
    part_2::solve(&input)?;

    Ok(())
}
