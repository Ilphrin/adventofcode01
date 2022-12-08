use itertools::Itertools;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let args: Vec<String> = std::env::args().collect_vec();

    let lines = include_str!("../input.txt")
        .lines()
        .map(|v| v.parse::<u64>().ok())
        .batching(|it| {
            let mut sum = None;
            while let Some(Some(v)) = it.next() {
                sum = Some(sum.unwrap_or(0) + v);
            }
            sum
        })
        .sorted_by_key(|&v| std::cmp::Reverse(v))
        .take(args[1].parse::<usize>().unwrap())
        .sum::<u64>();

    println!("{lines:?}");

    Ok(())
}
