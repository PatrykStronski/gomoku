mod gameplay;
mod points_calculation;
mod computer_turn;
mod user_turn;

fn main() {
    println!("Welcome to Gomoku game! \n The game is starting");
    gameplay::start_new_game();
}
