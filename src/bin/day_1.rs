use advent_of_code::load_input_file;
use advent_of_code::get_number;
fn main() {
    let file_as_string = load_input_file("inputs\\2022_day_1_input.txt").unwrap();
    //println!("{}",file_as_string);

    let elf_parts = file_as_string.split("\n\n");
    let elves = elf_parts.collect::<Vec<&str>>();
    let mut totals: Vec<u16> = Vec::new();

    for (index, elf) in elves.iter().enumerate() {
        println!("elf{} -- index{}", elf, index);
        let elf_cals: Vec<u16> ;
        for s in elf {
            let num: Option<u32> = get_number(s, '\n');

        }

        // totals.push( elf_cal_parts );
        //  totals[index] += cal as u16;
    }
    // let mut index = 0;
    // for total in &totals {
    //     index += 1;
    //     println!("index: {}, total: {}", total, totals[index])
    // }
}

