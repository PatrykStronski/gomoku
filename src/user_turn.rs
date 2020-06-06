use std::io::{stdin, stdout, Write};

fn into_u8(input: Vec<&str>) -> [usize; 2] {
    let x: usize = input[0].parse().unwrap();
    let y: usize = input[1].parse().unwrap();
    return [x, y];
}

pub fn get_user_turn() -> [usize; 2] {
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
        return get_user_turn();
    }
    let out_u8 = into_u8(out);
    if out_u8[0] == 15 {
        println!("Wrong input, try once again");
        return get_user_turn();
    }
    return out_u8;
}
