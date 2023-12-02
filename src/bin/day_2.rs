mod read_input;

const MAX_TOTAL: i32 = 39;


fn max_for_color(color: &str) -> i32 {
    match color {
        "red" => 12,
        "green" => 13,
        "blue" => 14,
        &_ => todo!(),
    }
}

fn is_valid_round(raw_round: &str) -> bool {
    let mut sum = 0;
    for raw_color in raw_round.split(',') {
        let mut split = raw_color.trim().split(' ');
        let count: i32 = split.next().unwrap().trim().parse().unwrap();
        let color = split.next().unwrap();

        let max_for_color = max_for_color(&color);
        if count > max_for_color {
            return false;
        }

        sum += count;
    }

    sum <= MAX_TOTAL
}

fn get_score_for_game(line: String) -> i32 {
    let mut game_parts = line.split(':');
    let game_info = game_parts.next().unwrap();
    let id: i32 = game_info.split(' ').nth(1).unwrap().trim().parse().unwrap();

    let raw_rounds = game_parts.next().unwrap().split(';');

    for raw_round in raw_rounds {
        if !is_valid_round(raw_round) {
            return 0;
        }
    }
    id
}

fn main() {
    let lines = read_input::file_to_lines("./bin/day2.input").expect("uhoh");

    let mut sum = 0;
    for line in lines {
        let id = get_score_for_game(line.unwrap());
        sum += id;
    }
    println!("{}", sum);
}
