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
            if are_adjacent_filled(board, x, y) {
                let mut coords = [x, y];
                fields.push(coords);
            }
        }
    }
    return fields;
}

fn calculate_player_eventual_moves(board: &[[u8; 15]; 15], considerate_fields: &Vec<[usize; 2]>) -> Vec<TreeSegment> {
    let mut eventual_moves = Vec::<TreeSegment>::with_capacity(considerate_fields.len());
    for field in considerate_fields {
        let mut tr = TreeSegment {
            coordinates: *field,
            gain: 5 - calculate_field_points(board, *field, 1, true),
            leaves: Vec::<TreeSegment>::new(),
        };
        eventual_moves.push(tr);
    }
    return eventual_moves;
}

fn calculate_computer_eventual_moves(board: &[[u8; 15]; 15], considerate_fields: &Vec<[usize; 2]>) -> Vec<TreeSegment> {
    let mut eventual_moves = Vec::<TreeSegment>::with_capacity(considerate_fields.len());
    for field_id in 0..considerate_fields.len() {
        let mut new_considerate_fields = considerate_fields.to_vec();
        new_considerate_fields.remove(field_id);
        eventual_moves.push(TreeSegment {
            coordinates: considerate_fields[field_id],
            gain: calculate_field_points(board, considerate_fields[field_id], 2, true),
            leaves: calculate_player_eventual_moves(board, &new_considerate_fields),
        });
    }
    return eventual_moves;
}

fn get_best_move(moves: &Vec<TreeSegment>) -> [usize; 2] {
    let mut best_points = 0;
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
    let eventual_moves = calculate_computer_eventual_moves(board, &considerate_fields);
    return get_best_move(&eventual_moves);
}
