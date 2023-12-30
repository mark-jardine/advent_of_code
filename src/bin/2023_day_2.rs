use std::cmp::{max, min};
use advent_of_code::load_input_file;

fn main() {
    let input= load_input_file("inputs/2023_day_2_input.txt").expect("ERROR: could not read input.");
    // println!("{}", input);
    part_1(&input);
    part_2(&input);
}

//Determine which games would have been possible
// if the bag had been loaded with only
//
// 12 red cubes,
// 13 green cubes,
// and 14 blue cubes.
//
// What is the sum of the IDs of those games?
fn part_1(input: &str){
    let max_red: u32 = 12;
    let max_green: u32 = 13;
    let max_blue: u32 = 14;

    // Ids of possible games
    let mut ids: Vec<u32> = Vec::new();

    // Split input by line, to get 1 game per line
    let games = input.lines();

    // Iterate over games
    for game in games{
        let mut valid_game = true;
        let current_id = get_id(game);
        let selections_str: Vec<&str> = get_selections(game);

        // selection = e.g. ["7 blue, 9 green, 2 red", " 5 red, 9 green", " 1 blue, 8 red, 13 green"]
        for selection in selections_str.iter() {
            let parts = selection.split(", ");
            // part = e.g. "7 blue"
            for part in parts {
                let num_and_colour: Vec<&str> = part.trim().split_whitespace().collect();

                let number: u32 = num_and_colour[0].parse().expect("Couldn't parse number");
                let colour: &str = num_and_colour[1];

                if colour.eq("red") {
                    if number > max_red {
                        valid_game = false;
                    }
                } else if colour.eq("blue") {
                    if number > max_blue {
                        valid_game = false;
                    }
                } else {
                    if number > max_green {
                        if number > max_green {
                            valid_game = false;
                        }
                    }
                }
            }
        }
        if valid_game { ids.push(current_id) }
    }

    let sum: u32 = ids.iter().sum();
    println!("Part 1\nSum of valid ids: <{sum}>\n");
}

//For each game,
// find the minimum set of cubes that must have been present.
//
// What is the sum of the power of these sets?
fn part_2(input: &str) {
    // Split input by line, to get 1 game per line
    let games = input.lines();
    let mut total: u32 = 0;

    // Iterate over games
    for game in games {
        let selections_str: Vec<&str> = get_selections(game);
        let mut min_red: u32 = 0;
        let mut min_blue: u32 = 0;
        let mut min_green: u32 = 0;

        // selection = e.g. ["7 blue, 9 green, 2 red", " 5 red, 9 green", " 1 blue, 8 red, 13 green"]
        for selection in selections_str.iter() {
            let parts = selection.split(", ");
            // part = e.g. "7 blue"
            for part in parts {
                let num_and_colour: Vec<&str> = part.trim().split_whitespace().collect();

                let number: u32 = num_and_colour[0].parse().expect("Couldn't parse number");
                let colour: &str = num_and_colour[1];

                match colour {
                    "red" => {
                        if min_red > 0 {
                            min_red = max(min_red, number)
                        }
                        else{
                            min_red = number;
                        }
                    },
                    "blue" => {
                        if min_blue > 0 {
                            min_blue = max(min_blue, number)
                        }
                        else{
                            min_blue = number;
                        }
                    },
                    "green" => {
                        if min_green > 0 {
                            min_green = max(min_green, number)
                        }
                        else{
                            min_green = number;
                        }
                    }
                    _ => {break;}
                }
            }
        }
        total += min_green * min_blue * min_red;
        println!("{min_red}r, {min_blue}b, {min_green}g, current total:{total}");
    }
    println!("Part 2\nSum of powers: <{total}>");
}

fn get_id(game: &str) -> u32{
    // descriptor = "Game <id>"
    let descriptor: &str = game.split(':').nth(0).expect("ERROR: error extracting game descriptor.");
    let id: u32 = descriptor.split(' ').nth(1).unwrap().parse().ok().unwrap();

    id
}

fn get_selections(game: &str) -> Vec<&str>{
    let selections: Vec<&str> = game
        .split(':')
        .nth(1).unwrap()
        .trim()
        .split(';')
        .collect();

    selections
}