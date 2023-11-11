use rand::Rng;
use std::io;

#[derive(Copy, Clone, PartialEq)]
enum Player {
    X,
    O,
}

impl Player {
    fn toggle(&self) -> Player {
        match self {
            Player::X => Player::O,
            Player::O => Player::X,
        }
    }

    fn as_char(&self) -> char {
        match self {
            Player::X => 'X',
            Player::O => 'O',
        }
    }
}

fn main() {
    let mut board = [' '; 9];
    let mut rng = rand::thread_rng();
    let human_player = if rng.gen() { Player::X } else { Player::O };
    let computer_player = human_player.toggle();

    println!("You are player {}", human_player.as_char());

    // Set the current player to X
    let mut current_player = Player::X;

    // If the human player is O, computer makes the first move
    if human_player == Player::O {
        computer_turn_minimax(&mut board, computer_player, human_player);
        current_player = current_player.toggle(); // Toggle the player so that human is next
    }

    print_board(&board);

    loop {
        if current_player == human_player {
            let choice = get_player_choice(&mut board);
            board[choice] = current_player.as_char();
        } else {
            computer_turn_minimax(&mut board, computer_player, human_player);
        }

        print_board(&board);

        if let Some(winner) = check_winner(&board) {
            println!("Player {} wins!", winner.as_char());
            break;
        } else if board.iter().all(|&x| x != ' ') {
            println!("It's a tie!");
            break;
        }

        current_player = current_player.toggle();
    }
}

fn print_board(board: &[char; 9]) {
    for (i, &cell) in board.iter().enumerate() {
        if i % 3 == 0 {
            println!();
        }
        if cell == ' ' {
            print!("{} ", i);
        } else {
            print!("{} ", cell);
        }
    }
    println!("\n");
}

fn get_player_choice(board: &mut [char; 9]) -> usize {
    loop {
        println!("Enter your move (0-8): ");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim().parse::<usize>() {
            Ok(index) if index < 9 && board[index] == ' ' => return index,
            _ => println!("Invalid input, please try again."),
        }
    }
}

fn check_winner(board: &[char; 9]) -> Option<Player> {
    let check_line = |player: char, a: usize, b: usize, c: usize| {
        board[a] == player && board[b] == player && board[c] == player
    };

    for &player in &[Player::X, Player::O] {
        let player_char = player.as_char();

        // Check horizontal lines
        for row in 0..3 {
            if check_line(player_char, row * 3, row * 3 + 1, row * 3 + 2) {
                return Some(player);
            }
        }

        // Check vertical lines
        for col in 0..3 {
            if check_line(player_char, col, col + 3, col + 6) {
                return Some(player);
            }
        }

        // Check diagonals
        if check_line(player_char, 0, 4, 8) || check_line(player_char, 2, 4, 6) {
            return Some(player);
        }
    }

    None
}

fn evaluate_board(board: &[char; 9], computer_player: Player, human_player: Player) -> i32 {
    match check_winner(board) {
        Some(player) if player == computer_player => 1, // Computer wins
        Some(player) if player == human_player => -1,   // Human wins
        Some(_) | None => 0,                            // Draw or undecided
    }
}

fn minimax(
    board: &mut [char; 9],
    depth: i32,
    is_maximizing: bool,
    computer_player: Player,
    human_player: Player,
) -> i32 {
    let score = evaluate_board(board, computer_player, human_player);

    if score != 0 || depth == 0 || is_board_full(board) {
        return score;
    }

    if is_maximizing {
        let mut best_score = i32::MIN;
        for i in 0..board.len() {
            if board[i] == ' ' {
                board[i] = computer_player.as_char();
                best_score = best_score.max(minimax(
                    board,
                    depth - 1,
                    false,
                    computer_player,
                    human_player,
                ));
                board[i] = ' ';
            }
        }
        best_score
    } else {
        let mut best_score = i32::MAX;
        for i in 0..board.len() {
            if board[i] == ' ' {
                board[i] = human_player.as_char();
                best_score = best_score.min(minimax(
                    board,
                    depth - 1,
                    true,
                    computer_player,
                    human_player,
                ));
                board[i] = ' ';
            }
        }
        best_score
    }
}

fn is_board_full(board: &[char; 9]) -> bool {
    board.iter().all(|&cell| cell != ' ')
}

fn computer_turn_minimax(board: &mut [char; 9], computer_player: Player, human_player: Player) {
    let mut best_score = i32::MIN;
    let mut best_move = None;

    for i in 0..board.len() {
        if board[i] == ' ' {
            board[i] = computer_player.as_char();
            let score = minimax(board, 9, false, computer_player, human_player);
            board[i] = ' ';
            if score > best_score {
                best_score = score;
                best_move = Some(i);
            }
        }
    }

    if let Some(move_index) = best_move {
        board[move_index] = computer_player.as_char();
    }
}
