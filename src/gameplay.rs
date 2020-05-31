use rand::Rng;
use crate::points_calculation::calculate_field_points;
use crate::computer_turn::make_single_turn;
use crate::user_turn::get_user_turn;

fn field_to_char(number: u8) -> String {
    if number == 0 {
        return String::from("0 ");
    }
    if number == 1 {
        return String::from("1 ");
    }
    if number == 2 {
        return String::from("2 ");
    }
    return String::from("3 ");
}

fn print_board(board: &[[u8; 15]; 15]) {
    let mut ind = 0;
    println!("   0 1 2 3 4 5 6 7 8 9 10 11 12 13 14");
    println!("____________________________________");
    for row in board {
        let mut row_str = String::new();
        for field in row {
            row_str.push_str(&field_to_char(*field));
        }
        println!("{} | {}", ind, row_str);
        ind += 1;
    }
}

fn duel(board: &mut[[u8; 15]; 15]) -> u8 {
    let user_turn = get_user_turn();
    println!("{:?}", user_turn);
    board[user_turn[0]][user_turn[1]] = 1;
    if calculate_field_points(board, user_turn[0], user_turn[1], 1) >= 5 {
        return 1;
    }
    let computer_turn =  make_single_turn(board);
    if calculate_field_points(board, computer_turn[0], computer_turn[1], 2) >= 5 {
        return 2;
    }
    return duel(board);
}

pub fn start_new_game() {
    let mut board = [[0u8; 15]; 15];
    println!("Computer starts");
    let computer_code = 2;
    let starting_point_x = rand::thread_rng().gen_range(0,15) as usize;
    let starting_point_y = rand::thread_rng().gen_range(0,15) as usize;
    board[starting_point_x][starting_point_y] = computer_code;
    print_board(&board);
    let winner = duel(&mut board);
}