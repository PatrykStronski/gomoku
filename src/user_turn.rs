use std::io::{stdin, stdout, Write};

fn into_usize(input: Vec<&str>) -> [usize; 2] {
    let x: usize = input[0].parse().unwrap();
    let y: usize = input[1].parse().unwrap();
    return [x, y];
}

fn invalid_coordinates(out_usize: [usize; 2]) -> bool {
    let condition =
        out_usize[0] >= 0 && out_usize[0] < 15 && out_usize[1] >= 0 && out_usize[1] < 15;
    if condition {
        return false;
    }
    return true;
}

pub fn get_user_turn(board: &[[u8; 15]; 15]) -> [usize; 2] {
    let mut s = String::new();
    println!("Your turn. Enter the coordinates, for example 2,11");
    let _ = stdout().flush();
    stdin()
        .read_line(&mut s)
        .expect("Did not enter a correct string");
    s = s.trim().to_string();
    let out = s.split(',').collect::<Vec<&str>>();
    if out.len() < 2 {
        println!("Wrong input, try once again");
        return get_user_turn(board);
    }
    let out_usize = into_usize(out);
    if invalid_coordinates(out_usize) {
        println!("Wrong input, try once again");
        return get_user_turn(board);
    }
    if board[out_usize[0]][out_usize[1]] != 0 {
        println!("Wrong input, field taken, try once again");
        return get_user_turn(board);
    }
    return out_usize;
}
