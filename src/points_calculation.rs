fn calculate_horizontal_points(board: &[[u8; 15]; 15], x: usize, y: usize, gamer_code: u8) -> u8 {
    let mut points = 0;
    let mut curr_x = x + 1;
    while curr_x < 15 && board[curr_x][y] == gamer_code {
        points += 1;
        curr_x += 1;
    }
    return points;
}

fn calculate_vertical_points(board: &[[u8; 15]; 15], x: usize, y: usize, gamer_code: u8) -> u8 {
    let mut points = 0;
    let mut curr_y = y + 1;
    while curr_y < 15 && board[x][curr_y] == gamer_code {
        points += 1;
        curr_y += 1;
    }
    return points;
}

fn points_valid(x: usize, y: usize) -> bool {
    if x > 14 {
        return false;
    }
    if y > 14 {
        return false;
    }
    return true;
}

fn calculate_diagonal_points_left(
    board: &[[u8; 15]; 15],
    x: usize,
    y: usize,
    gamer_code: u8,
) -> u8 {
    let mut points = 0;
    let mut curr_y = y + 1;
    let mut curr_x = x - 1;
    while points_valid(curr_x, curr_y) && board[curr_x][curr_y] == gamer_code {
        points += 1;
        curr_x -= 1;
        curr_y += 1;
    }
    curr_y = y - 1;
    curr_x = x + 1;
    while points_valid(curr_x, curr_y) && board[curr_x][curr_y] == gamer_code {
        points += 1;
        curr_x += 1;
        curr_y -= 1;
    }
    return points;
}

fn calculate_diagonal_points_right(
    board: &[[u8; 15]; 15],
    x: usize,
    y: usize,
    gamer_code: u8,
) -> u8 {
    let mut points = 0;
    let mut curr_y = y + 1;
    let mut curr_x = x + 1;
    while points_valid(curr_x, curr_y) && board[curr_x][curr_y] == gamer_code {
        points += 1;
        curr_x += 1;
        curr_y += 1;
    }
    curr_y = y - 1;
    curr_x = x - 1;
    while points_valid(curr_x, curr_y) && board[curr_x][curr_y] == gamer_code {
        points += 1;
        curr_x -= 1;
        curr_y -= 1;
    }
    return points;
}

fn calculate_diagonal_points(board: &[[u8; 15]; 15], x: usize, y: usize, gamer_code: u8) -> u8 {
    let left = calculate_diagonal_points_left(board, x, y, gamer_code);
    let right = calculate_diagonal_points_right(board, x, y, gamer_code);
    if left > right {
        return left;
    }
    return right;
}

pub fn calculate_field_points(
    board: &[[u8; 15]; 15],
    coordinates: [usize; 2],
    gamer_code: u8,
    assume_filled: bool,
) -> u8 {
    let mut points = 0u8;
    if board[coordinates[0]][coordinates[1]] != gamer_code {
        if !assume_filled {
            return points;
        }
        points += 1;
    }
    let horizontal = calculate_horizontal_points(board, coordinates[0], coordinates[1], gamer_code);
    let vertical = calculate_vertical_points(board, coordinates[0], coordinates[1], gamer_code);
    let diagonal = calculate_diagonal_points(board, coordinates[0], coordinates[1], gamer_code);
    if horizontal > vertical {
        points = horizontal;
    } else {
        points = vertical;
    }
    if points < diagonal {
        points = diagonal;
    }
    return points;
}
