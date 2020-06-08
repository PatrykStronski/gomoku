use crate::choice_tree::TreeSegment;
use crate::points_calculation::calculate_field_points;

fn are_adjacent_filled(board: &[[u8; 15]; 15], x: usize, y: usize) -> bool {
    if x > 0 {
        if board[x - 1][y] != 0 {
            return true;
        }
        if y > 0 {
            if board[x - 1][y - 1] != 0 {
                return true;
            }
        }
        if y < 14 {
            if board[x - 1][y + 1] != 0 {
                return true;
            }
        }
    }
    if x < 14 {
        if board[x + 1][y] != 0 {
            return true;
        }
        if y > 0 {
            if board[x + 1][y - 1] != 0 {
                return true;
            }
        }
        if y < 14 {
            if board[x + 1][y + 1] != 0 {
                return true;
            }
        }
    }
    if y > 0 {
        if board[x][y - 1] != 0 {
            return true;
        }
    }
    if y < 14 {
        if board[x][y + 1] != 0 {
            return true;
        }
    }
    return false;
}

fn get_considerate_fields(board: &[[u8; 15]; 15]) -> Vec<[usize; 2]> {
    let mut fields = Vec::<[usize; 2]>::with_capacity(8);
    for x in 0usize..15usize {
        for y in 0usize..15usize {
            if are_adjacent_filled(board, x, y) && board[x][y] == 0 {
                let coords = [x, y];
                fields.push(coords);
            }
        }
    }
    return fields;
}

fn copy_board(board: &[[u8; 15]; 15]) -> [[u8; 15]; 15] {
    let mut new_board = [[0u8; 15]; 15];
    for x in 0..15 {
        for y in 0..15 {
            new_board[x][y] = board[x][y];
        }
    }
    return new_board;
}

fn calculate_player_eventual_moves(
    board: &[[u8; 15]; 15],
    considerate_fields: &Vec<[usize; 2]>,
    depth: u8,
    alpha: i8,
    mut beta: i8,
) -> Vec<TreeSegment> {
    if depth == 0 {
        return Vec::<TreeSegment>::new();
    }
    let mut eventual_moves = Vec::<TreeSegment>::with_capacity(considerate_fields.len());
    for field_id in 0..considerate_fields.len() {
        let mut new_considerate_fields = considerate_fields.to_vec();
        let field = new_considerate_fields.remove(field_id);
        let mut new_board = copy_board(board);
        new_board[field[0]][field[1]] = 1;
        let gain = calculate_field_points(&new_board, considerate_fields[field_id], 1);
        if beta > gain {
            beta = gain;
        }
        if beta >= alpha {
            break;
        }
        let tr = TreeSegment {
            coordinates: considerate_fields[field_id],
            gain: -gain,
            leaves: calculate_computer_eventual_moves(
                &new_board,
                &new_considerate_fields,
                depth - 1,
                alpha,
                beta,
            ),
            minimize_leaves: false,
        };
        eventual_moves.push(tr);
    }
    return eventual_moves;
}

fn calculate_computer_eventual_moves(
    board: &[[u8; 15]; 15],
    considerate_fields: &Vec<[usize; 2]>,
    depth: u8,
    mut alpha: i8,
    beta: i8,
) -> Vec<TreeSegment> {
    if depth == 0 {
        return Vec::<TreeSegment>::new();
    }
    let mut eventual_moves = Vec::<TreeSegment>::with_capacity(considerate_fields.len());
    for field_id in 0..considerate_fields.len() {
        let mut new_considerate_fields = considerate_fields.to_vec();
        let field = new_considerate_fields.remove(field_id);
        let mut new_board = copy_board(board);
        let gain = calculate_field_points(&new_board, considerate_fields[field_id], 2);
        if alpha < gain {
            alpha = gain;
        }
        if beta <= alpha {
            break;
        }
        new_board[field[0]][field[1]] = 2;
        eventual_moves.push(TreeSegment {
            coordinates: considerate_fields[field_id],
            gain: gain,
            leaves: calculate_player_eventual_moves(
                &new_board,
                &new_considerate_fields,
                depth - 1,
                alpha,
                beta,
            ),
            minimize_leaves: true,
        });
    }
    return eventual_moves;
}

fn get_best_move(moves: &Vec<TreeSegment>) -> [usize; 2] {
    let mut best_points = -5;
    let mut coordinates = [0usize, 0usize];
    for single_move in moves {
        let current_points = single_move.get_highest_gain();
        if current_points > best_points {
            best_points = current_points;
            coordinates = single_move.coordinates;
        }
    }
    return coordinates;
}

pub fn get_single_turn(board: &[[u8; 15]; 15]) -> [usize; 2] {
    let considerate_fields = get_considerate_fields(board);
    let eventual_moves =
        calculate_computer_eventual_moves(board, &considerate_fields, 5, -100, 100);
    return get_best_move(&eventual_moves);
}
