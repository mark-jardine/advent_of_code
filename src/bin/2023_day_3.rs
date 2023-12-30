use advent_of_code::load_input_file;

fn main() {
    let input: String = load_input_file("inputs/2023_day_3_input.txt").expect("Could not load file.");
    print!("{}", &input)
}

fn part_1(input: &str){
    let lines: Vec<&str> = input.lines().collect();
    let current: u8 = 0;
    for line in lines.iter(){
        if current == 0 {
            // Don't get previous line as there is none
        }

    }
}

fn part_2(input: &str){

}