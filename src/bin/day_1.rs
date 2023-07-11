use advent_of_code::load_input_file;
fn main() {
    let file_as_string = load_input_file("inputs/2022_day_1_input.txt").unwrap();
    //println!("{}",file_as_string);

    let elf_parts = file_as_string.split("\n\n");
    let elves = elf_parts.collect::<Vec<&str>>();
    let mut totals: Vec<u64> = Vec::new();

    for (index, elf) in elves.iter().enumerate() {
        // println!("\n{} \nindex:{}\n---------", elf, index);
        let mut elf_cals: Vec<u64> = Vec::new();
        for cal in elf.split("\n") {
            elf_cals.push(cal.parse().unwrap_or(0));
        }
        let total_to_push = elf_cals.iter().sum();
        totals.push(total_to_push);
        println!("Elf {} : {}", index+1, total_to_push );
    }

    //assume 1 elf only can have the maximum number of calories
    let max_calories = totals.iter().max();
    match max_calories {
        Some(max) => println!("Elf {} was the highest with <{}> calories in total.", totals.iter().position(|&x| x == *max).unwrap() + 1, max),
        None => println!("No elves"),
    }

    // Part 2- get total calories of top three calorie holders
    totals.sort_by(|a, b| b.cmp(a));

    let top_three_totals: Vec<u64> = totals[0..3].to_vec();
    let top_three_sum: u64= top_three_totals.iter().sum();

    println!("\nTotal calories of the top three Elves: {:?}",top_three_sum);

}

