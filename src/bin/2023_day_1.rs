use std::collections::HashMap;
use std::str::Chars;
use advent_of_code::load_input_file;

fn main() {
    let file_as_string = load_input_file("inputs/2023_day_1_input.txt").unwrap();
    let lines = file_as_string.split("\n").collect::<Vec<&str>>();
    println!("Sum of calibration values: <{:?}>", solution(lines));
}

fn solution(lines: Vec<&str>) -> u32{
    // println!("{:?}", lines);
    let mut total: u32 = 0;
    let word_to_num: HashMap<&str, i8> =
        vec![
            ("one", 1),
            ("two", 2),
            ("three", 3),
            ("four", 4),
            ("five", 5),
            ("six", 6),
            ("seven", 7),
            ("eight", 8),
            ("nine", 9)
        ]
            .into_iter()
            .collect();

    //combine first and last digit in the line, may be the same digit
    for line in lines.iter(){
        let mut nums: String = String::new();
        for char in line.chars(){
            if char.is_numeric(){
                nums.push(char);
            }
                //not working, see last before/after pair, not picking up words
            else {
                let buf: String = line.chars().take_while(|&char| !char.is_numeric()).collect();
                println!("before:{:?}", nums);
                println!("buffer: {:?}",buf.as_str());
                if buf.len() > 0{

                    if let Some(&word) = word_to_num.get(buf.as_str()){
                        nums.push(word.to_string().parse().expect(format!("nums was {:?}", nums).as_str()));
                    }
                }
                println!("after:{:?}", nums);

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

fn replace_word_with_integer(string: String) -> String{
    for char in string.chars(){

    }

    String::new()
}