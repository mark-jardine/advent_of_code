use advent_of_code::load_input_file;
#[derive(Debug, Clone)]
enum Move {
    Rock = 1, // A || X
    Paper = 2, // B || Y
    Scissors = 3, // C || Z

    // Rock < Paper < Scissors < Rock
}

fn parse_move(c: char) -> Option<Move>{
    match c {
        'A' | 'X' => Some(Move::Rock),
        'B' | 'Y' => Some(Move::Paper),
        'C' | 'Z' => Some(Move::Scissors),
        _ => None,
    }
}

// The winner of the whole tournament is the player with the highest score.
// Your total score is the sum of your scores for each round.
// The score for a single round is the score for the shape you selected (1 for Rock, 2 for Paper, and 3 for Scissors)
// plus the score for the outcome of the round (0 if you lost, 3 if the round was a draw, and 6 if you won).
fn main(){
    let file_as_string = load_input_file("inputs/2022_day_2_input.txt").unwrap();
    //println!("{}",file_as_string);

    let lines = file_as_string.split("\n").collect::<Vec<&str>>();
    let (p1_moves, p2_moves): (Vec<Move>, Vec<Move>) = lines
        .into_iter()
        .map(|s|{
            let chars : Vec<char> = s.chars().collect();
            (
                //Get player one's move
               chars.get(0).and_then(|c| parse_move(*c)).unwrap(),

                //Get player two's move, which is index=2
                chars.get(2).and_then(|c| parse_move(*c)).unwrap(),
            )
        })
        .unzip();

    //My total score
    let mut score: u16 = 0;

    // I am player 2 -- x, y, z moves
    for (p1_move, p2_move) in p1_moves.iter().zip(p2_moves.iter()){
        match p2_move{
            Move::Rock => match p1_move {
                Move::Rock => score += 3 + 1,
                Move::Paper => score += 0 + 1,
                Move::Scissors => score += 6 + 1
            },
            Move::Paper => match p1_move {
                Move::Rock => score += 6 + 2,
                Move::Paper => score += 3 + 2,
                Move::Scissors => score += 0 + 2
            },
            Move::Scissors => match p1_move {
                Move::Rock => score += 0 + 3,
                Move::Paper => score += 6 + 3,
                Move::Scissors => score += 3 + 3
            }
        }
        
    }

    for (p1, p2) in p1_moves.iter().zip(p2_moves.iter()) {
        println!("{:?} {:?}", *p1, *p2);
    }
    println!("Score: {}",score);

}