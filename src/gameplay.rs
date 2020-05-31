use rand::Rng;
use crate::points_calculation::calculate_field_points;

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
    for row in board {
        let mut row_str = String::new();
        for field in row {
            row_str.push_str(&field_to_char(*field));
        }
        println!("{}", row_str);
    }
}

pub fn start_new_game() {
    let mut board = [[0u8; 15]; 15];
    println!("Computer starts");
    let computer_code = 2;
    let starting_point_x = rand::thread_rng().gen_range(0,15) as usize;
    let starting_point_y = rand::thread_rng().gen_range(0,15) as usize;
    board[starting_point_x][starting_point_y] = computer_code;
    print_board(&board);
    
}