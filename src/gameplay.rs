use crate::computer_turn::get_single_turn;
use crate::points_calculation::calculate_field_points;
use crate::user_turn::get_user_turn;
use rand::Rng;
use std::time::Instant;

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
    println!("     0 1 2 3 4 5 6 7 8 9|10|11|12|13|14");
    println!("________________________________________");
    for row in board {
        let mut row_str = String::new();
        for field in row {
            row_str.push_str(&field_to_char(*field));
        }
        if ind < 10 {
            println!("0{} | {}", ind, row_str);
        } else {
            println!("{} | {}", ind, row_str);
        }
        ind += 1;
    }
}

fn duel(board: &mut [[u8; 15]; 15], mut computer_time: u128) -> u8 {
    print_board(board);
    let user_turn = get_user_turn();
    board[user_turn[0]][user_turn[1]] = 1;
    if calculate_field_points(board, user_turn, 1) >= 5 {
        println!("Computer thinking time: {}", computer_time);
        return 1;
    }
    let now = Instant::now();
    let computer_turn = get_single_turn(board);
    computer_time += now.elapsed().as_millis();
    board[computer_turn[0]][computer_turn[1]] = 2;
    if calculate_field_points(board, computer_turn, 2) >= 5 {
        println!("Computer thinking time: {}", computer_time);
        return 2;
    }
    return duel(board, computer_time);
}

pub fn start_new_game() {
    let mut board = [[0u8; 15]; 15];
    println!("Computer starts");
    let computer_code = 2;
    let starting_point_x = rand::thread_rng().gen_range(0, 15) as usize;
    let starting_point_y = rand::thread_rng().gen_range(0, 15) as usize;
    board[starting_point_x][starting_point_y] = computer_code;
    let winner = duel(&mut board, 0);
    print_board(&board);
    if winner == 1 {
        println!("YOU WON! CONGRATS!");
    } else {
        println!("COMPUTER WON, SORRY");
    }
}
