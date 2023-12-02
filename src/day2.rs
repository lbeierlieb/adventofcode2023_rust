//use nom::IResult;
//use nom::branch::alt;
//use nom::bytes::complete::tag;
//use nom::character::complete::digit1;
//use nom::error::{ParseError, ErrorKind};

#[derive(Debug)]
struct Round {
    blue: u64,
    green: u64,
    red: u64,
}

impl Round {
    fn new() -> Round {
        Round {
            blue: 0,
            green: 0,
            red: 0,
        }
    }
}

#[derive(Debug)]
struct Game {
    id: u64,
    rounds: Vec<Round>,
}

//fn parse_round(input: &str) -> IResult<&str, Round> {
//    let mut round = Round::new();
//    let mut start_input = input;
//    loop {
//        let (input, _) = tag(" ")(start_input)?;
//        let (input, count_str) = digit1(input)?;
//        let (input, _) = tag(" ")(input)?;
//        let (input, color) = alt((tag("blue"), tag("green"), tag("red")))(input)?;
//        let count: u64 = count_str.parse().unwrap();
//        match color {
//            "blue" => round.blue = count,
//            "green" => round.green = count,
//            "red" => round.red = count,
//            _ => {},
//        }
//        let res_comma = tag(",")(input);
//        if res_comma.is_ok() {
//            start_input = res_comma.unwrap().0;
//        } else {
//
//        }
//    }
//    Ok((input, round))
//}
//
//fn parse_game(input: &str) -> IResult<&str, Game> {
//    let (input, _) = tag("Game ")(input)?;
//    let (input, id_str) = digit1(input)?;
//    println!{"{}", id_str};
//    println!{"{}", input};
//    Ok(("", Game {id: 0, rounds: Vec::new()}))
//}
//
// nom parsing didn't work out
// current parsing is really ugly, please find something better

fn parse_round(input: &str) -> Round {
    let mut round = Round::new();
    for entry in input.split(",") {
        let mut spacesplit = entry.split(" ");
        spacesplit.next().unwrap();
        let count_str = spacesplit.next().unwrap();
        let count = count_str.parse::<u64>().unwrap();
        match spacesplit.next().unwrap() {
            "blue" => round.blue = count,
            "green" => round.green = count,
            "red" => round.red = count,
            _ => {}
        }
    }
    round
}

fn parse_game(input: &str) -> Game {
    let mut colonsplit = input.split(":");

    let gamestring = colonsplit.next().unwrap();
    let mut spacesplit = gamestring.split(" ");
    spacesplit.next().unwrap();
    let id_str = spacesplit.next().unwrap();
    let id = id_str.parse::<u64>().unwrap();

    let roundsstring = colonsplit.next().unwrap();
    let semicolonsplit = roundsstring.split(";");
    let roundstrings: Vec<_> = semicolonsplit.collect();
    let rounds: Vec<_> = roundstrings.iter().map(|p| parse_round(p)).collect();

    Game { id, rounds }
}

fn is_round_possible(round: &Round) -> bool {
    round.red <= 12 && round.green <= 13 && round.blue <= 14
}

fn is_game_possible(game: &Game) -> bool {
    game.rounds.iter().all(is_round_possible)
}

pub fn task_one(input: String) -> u64 {
    input
        .lines()
        .map(parse_game)
        .filter(|game| is_game_possible(game))
        .map(|game| game.id)
        .sum()
}

fn get_min_number_cubes(lookup: fn(&Round) -> u64, game: &Game) -> u64 {
    game.rounds.iter().map(|round| lookup(round)).max().unwrap()
}

fn calculate_power(game: &Game) -> u64 {
    get_min_number_cubes(|round| round.blue, game)
        * get_min_number_cubes(|round| round.green, game)
        * get_min_number_cubes(|round| round.red, game)
}

pub fn task_two(input: String) -> u64 {
    input
        .lines()
        .map(parse_game)
        .map(|game| calculate_power(&game))
        .sum()
}
