fn can_subtract(point: usize) -> bool {
    if point > 0 {
        return true;
    }
    return false;
}

fn can_add(point: usize) -> bool {
    if point < 14 {
        return true;
    }
    return false;
}

fn calculate_horizontal_points(board: &[[u8; 15]; 15], x: usize, y: usize, gamer_code: u8) -> i8 {
    let mut points = 0;
    let mut curr_x = 0;
    if can_add(x) {
        curr_x = x + 1;
        while board[curr_x][y] == gamer_code {
            points += 1;
            if !can_add(curr_x) {
                break;
            }
            curr_x += 1;
        }
    }
    if can_subtract(x) {
        curr_x = x - 1;
        while board[curr_x][y] == gamer_code {
            points += 1;
            if !can_subtract(curr_x) {
                break;
            }
            curr_x -= 1;
        }
    }
    return points;
}

fn calculate_vertical_points(board: &[[u8; 15]; 15], x: usize, y: usize, gamer_code: u8) -> i8 {
    let mut points = 0;
    let mut curr_y = 0;
    if can_add(y) {
        curr_y = y + 1;
        while board[x][curr_y] == gamer_code {
            points += 1;
            if !can_add(curr_y) {
                break;
            }
            curr_y += 1;
        }
    }
    if can_subtract(y) {
        curr_y = y - 1;
        while board[x][curr_y] == gamer_code {
            points += 1;
            if !can_subtract(curr_y) {
                break;
            }
            curr_y -= 1;
        }
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
) -> i8 {
    let mut points = 0;
    let mut curr_y = 0;
    let mut curr_x = 0;
    if can_add(y) && can_subtract(x) {
        curr_y = y + 1;
        curr_x = x - 1;
        while board[curr_x][curr_y] == gamer_code {
            points += 1;
            if !can_add(curr_y) || !can_subtract(curr_x) {
                break;
            }
            curr_x -= 1;
            curr_y += 1;
        }
    }
    if can_add(x) && can_subtract(y) {
        curr_y = y - 1;
        curr_x = x + 1;
        while board[curr_x][curr_y] == gamer_code {
            points += 1;
            if !can_add(curr_x) || !can_subtract(curr_y) {
                break;
            }
            curr_x += 1;
            curr_y -= 1;
        }
    }
    return points;
}

fn calculate_diagonal_points_right(
    board: &[[u8; 15]; 15],
    x: usize,
    y: usize,
    gamer_code: u8,
) -> i8 {
    let mut points = 0;
    let mut curr_y = 0;
    let mut curr_x = 0;
    if can_add(y) && can_add(x) {
        curr_y = y + 1;
        curr_x = x + 1;
        while board[curr_x][curr_y] == gamer_code {
            points += 1;
            if !can_add(curr_y) || !can_add(curr_x) {
                break;
            }
            curr_x += 1;
            curr_y += 1;
        }
    }
    if can_subtract(y) && can_subtract(x) {
        curr_y = y - 1;
        curr_x = x - 1;
        while points_valid(curr_x, curr_y) && board[curr_x][curr_y] == gamer_code {
            points += 1;
            if !can_subtract(curr_y) || !can_subtract(curr_x) {
                break;
            }
            curr_x -= 1;
            curr_y -= 1;
        }
    }
    return points;
}

fn calculate_diagonal_points(board: &[[u8; 15]; 15], x: usize, y: usize, gamer_code: u8) -> i8 {
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
    gamer_code: u8
) -> i8 {
    let mut points = 0i8;
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
    if board[coordinates[0]][coordinates[1]] == gamer_code {
        points += 1;
    }
    return points;
}
