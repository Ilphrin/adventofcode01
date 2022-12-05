use std::fs;

fn main() {
    let mut most_calories = 0;
    let mut all_sum: Vec<i32> = Vec::new();

    let contents =
        fs::read_to_string("./input.txt").expect("Should have been able to read the file");

    let mut sum = 0;
    contents.lines().for_each(|x| {
        let val = x.parse::<i32>();
        match val {
            Result::Ok(val) => sum += val,
            Result::Err(_err) => {
                if sum >= most_calories {
                    most_calories = sum;
                }
                all_sum.push(sum);
                sum = 0;
            }
        }
    });
    all_sum.sort_by(|a, b| b.cmp(a));
    println!("Most calories: {most_calories}");
    println!("Top : {:?}", &all_sum[0..3]);
    println!("Sum of top: {:?}", &all_sum[0] + &all_sum[1] + &all_sum[2]);
}
