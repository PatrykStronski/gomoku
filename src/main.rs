mod choice_tree;
mod computer_turn;
mod gameplay;
mod points_calculation;
mod user_turn;

fn main() {
    println!("Welcome to Gomoku game! \n The game is starting");
    gameplay::start_new_game();
}
