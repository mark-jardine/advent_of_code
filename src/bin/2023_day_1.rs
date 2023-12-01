use advent_of_code::load_input_file;

fn main() {
    let file_as_string = load_input_file("inputs/2023_day_1_input.txt").unwrap();
    let lines = file_as_string.split("\n").collect::<Vec<&str>>();
    println!("Sum of calibration values: <{:?}>", solution(lines));
}

fn solution(lines: Vec<&str>) -> u32{
    // println!("{:?}", lines);
    let mut total: u32 = 0;

    //combine first and last digit in the line, may be the same digit
    for line in lines.iter(){
        let mut nums: String = String::new();
        for char in line.chars(){
            if char.is_numeric(){
                nums.push(char);
            }
        }
        //handle a single digit
        if nums.len() == 1{
            nums = format!("{}{}", nums, nums);
        }
        //otherwise get the first and last value in the string
        else {
            if let (Some(first), Some(last)) = (nums.chars().nth(0), nums.chars().last()){
                nums = format!("{}{}", first, last);
            }
        }

        let parsed_num: Result<u32, _> = nums.parse();
        total += parsed_num.unwrap();
    }
    total
}