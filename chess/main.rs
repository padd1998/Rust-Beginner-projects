// Don't have the time to program the behaviour of each piece. 

use std::fmt::{ Display, Formatter, Result };
use std::io;
use crate::Action::Status;

fn main() {
    let mut store = Store::create_store(reducer);
    store.dispatch(Status(StatusAction::Update(GameStatus::Playing)));
    

    print_board(&store.state);
    println!("Make sure the piece can move in that way before you do it.");
    loop {
        if store.state.status == GameStatus::Playing {
            take_turn(&mut store);
            print_board(&store.state);
        } else {
            break;
        }
    }
    print_winner(&store.state);
}

fn take_turn(store: &mut Store) {
    let mut token = Pieces::Empty;
    println!("Pick a piece to move.");
    let mut piece1 = String::new();
    io::stdin().read_line(&mut piece1).expect("Failed to read piece to be moved.");
    match piece1.trim().len() {
        2 => {
            // split into chars (an iter), remove any empty pieces. position 
            let position: Vec<&str> = piece1.trim().split("").filter(|s| !s.is_empty()).collect();
            // Match the input with a location. 
            match position[1].parse::<usize>() {
                Ok(y) => { // Sets x corresponding to a, b, or c. Uses this along with y as co-ordinates in move_piece (or place_piece) fn.
                    let x: usize = match position[0] { 
                        "b" => 1,
                        "c" => 2,
                        "d" => 3,
                        "e" => 4,
                        "f" => 5,
                        "g" => 6,
                        "h" => 7,
                        _ => 0
                    };

                    if y > 0 && y < 9 {
                        token = *Board::find_piece(&store.state.board, x, y - 1);
                        Board::destroy_piece(&mut store.state.board, x, y - 1);
                    } else {
                        println!("Try choosing somewhere on the board");
                    }
                }
        _ => println!("I don't understand.")
            }
        }
        _ => println!("Input needs to be a letter then a number corresponding to a location on the board.")
    }
    println!("Where do you want the piece to be moved?");
    let mut target = String::new();
    io::stdin().read_line(&mut target).expect("Failed to read line.");
    match target.trim().len() {
        2 => {
            // split into chars (an iter), remove any empty pieces. position 
            let position: Vec<&str> = target.trim().split("").filter(|s| !s.is_empty()).collect();
            // Match the input with a location. 
            match position[1].parse::<usize>() {
                Ok(y) => { // Sets x corresponding to a, b, or c. Uses this along with y as co-ordinates in move_piece (or place_piece) fn.
                    let x: usize = match position[0] { 
                        "b" => 1,
                        "c" => 2,
                        "d" => 3,
                        "e" => 4,
                        "f" => 5,
                        "g" => 6,
                        "h" => 7,
                        _ => 0
                    };

                    if y > 0 && y < 9 {
                        Board::place_piece(&mut store.state.board, x, y - 1, token);
                    } else {
                        println!("Try choosing somewhere on the board");
                    }
                }
        _ => println!("I don't understand.")
            }
        }
        _ => println!("Input needs to be a letter then a number corresponding to a location on the board.")
    }
}

fn print_winner(state: &State) {
    println!("Game over! {:?}", state.status);
}

fn print_board(state: &State) {
    println!("{}", state.board);
}

#[derive(PartialEq, Clone, Debug, Copy)]
pub enum Pieces {
    Pawn,
    Rook,
    King,
    Queen,
    Bishop,
    Horse,
    Empty, 
    AIPawn,
    AIRook,
    AIKing,
    AIQueen,
    AIBishop,
    AIHorse
}

impl Display for Pieces {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            &Pieces::Pawn => write!(f, " P")?,
            &Pieces::King => write!(f, " K")?,
            &Pieces::Queen => write!(f, " Q")?,
            &Pieces::Rook => write!(f, " R")?,
            &Pieces::Bishop => write!(f, " B")?,
            &Pieces::Horse => write!(f, " H")?,
            &Pieces::AIPawn => write!(f, "AP")?,
            &Pieces::AIHorse => write!(f, "AH")?,
            &Pieces::AIRook => write!(f, "AR")?,
            &Pieces::AIKing => write!(f, "AK")?,
            &Pieces::AIQueen => write!(f, "AQ")?,
            &Pieces::AIBishop => write!(f, "AB")?,
            &Pieces::Empty => write!(f, " .")?
        };

        Ok(())
    }
}

#[derive(Clone)]
pub struct Board {
    pub board: Vec<Vec<Pieces>>
}


impl Board {
    // Creates a new unplayed on board.
    pub fn new() -> Self {
        Board {
            board: vec![
                vec![Pieces::AIRook, Pieces::AIHorse, Pieces::AIBishop, Pieces::AIKing, Pieces::AIQueen, Pieces::AIBishop, Pieces::AIHorse, Pieces::AIRook],
                vec![Pieces::AIPawn, Pieces::AIPawn, Pieces::AIPawn, Pieces::AIPawn, Pieces::AIPawn, Pieces::AIPawn, Pieces::AIPawn, Pieces::AIPawn],
                vec![Pieces::Empty, Pieces::Empty, Pieces::Empty, Pieces::Empty, Pieces::Empty, Pieces::Empty, Pieces::Empty, Pieces::Empty],
                vec![Pieces::Empty, Pieces::Empty, Pieces::Empty, Pieces::Empty, Pieces::Empty, Pieces::Empty, Pieces::Empty, Pieces::Empty],
                vec![Pieces::Empty, Pieces::Empty, Pieces::Empty, Pieces::Empty, Pieces::Empty, Pieces::Empty, Pieces::Empty, Pieces::Empty],
                vec![Pieces::Empty, Pieces::Empty, Pieces::Empty, Pieces::Empty, Pieces::Empty, Pieces::Empty, Pieces::Empty, Pieces::Empty],
                vec![Pieces::Pawn, Pieces::Pawn, Pieces::Pawn, Pieces::Pawn, Pieces::Pawn, Pieces::Pawn, Pieces::Pawn, Pieces::Pawn],
                vec![Pieces::Rook, Pieces::Horse, Pieces::Bishop, Pieces::Queen, Pieces::King, Pieces::Bishop, Pieces::Horse, Pieces::Rook]
            ]
        }
    }
    pub fn update(&mut self, x: usize, y: usize, token: &Pieces) {
        if self.can_place(x, y) {
            self.board[x as usize][y as usize] = token.clone();
        }
    }
    pub fn can_place(&self, x: usize, y: usize) -> bool {
        self.board[x as usize][y as usize] == Pieces::Empty
    }
    /// Checks that there are still empty spaces on the board
    pub fn has_space(&self) -> bool {
        let mut space = false;
        for row in &self.board {
            if row.contains(&Pieces::Empty) {
                space = true;
                break;
            }
        };

        space
    }
    pub fn find_piece(self: &Self, x: usize, y: usize) -> &Pieces {

        if self.can_place(x, y) {
            println!("Square doesn't contain a piece. Pick again.")
            
        }
        let token: &Pieces = &self.board[x][y];
        token
    }

    pub fn destroy_piece(&mut self, x: usize, y: usize) {
        self.board[x][y] = Pieces::Empty;
    }
    
    pub fn place_piece(self: &mut Self, x: usize, y: usize, token: Pieces) { // Call can_place() in this to ensure square is empty.
    self.board[x][y] = token;
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let row_names = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];

        write!(f, "  ")?;
        for col in 0..self.board[0].len() {
            write!(f, "  {} ", col + 1)?;
        }

        write!(f, "\n")?;

        for row in 0..self.board.len() {
            write!(f, "{} ", row_names[row])?;

            for col in 0..self.board[row].len() {
                write!(f, " {} ", self.board[row as usize][col as usize])?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

#[derive(Clone)]
pub struct State {
    pub board: Board,
    pub status: GameStatus,
    pub winner: Pieces
}

impl State {
    pub fn default() -> Self {
        State {
            board: Board::new(),
            status: GameStatus::Playing,
            winner: Pieces::Empty
        }
    }
}

pub struct Store {
    pub state: State,
    reducer: fn(&State, Action) -> State
}

impl Store {
    pub fn create_store(reducer: fn(&State, Action) -> State) -> Store {
        Store {
            state: State::default(),
            reducer: reducer
        }
    }

    /// Trigger an update on the store
    pub fn dispatch(&mut self, action: Action) {
        self.state = (self.reducer)(&self.state, action);
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum GameStatus {
    Playing,
    Won,
    Lost,
    Draw
}

#[derive(Clone)]
pub enum Action {
    BoardUpdate(BoardAction),
    Status(StatusAction),
    Winner(WinnerAction)
}

pub fn reducer(state: &State, action: Action) -> State {
    State {
        board: board_reducer(&state.board, &action),
        status: status_reducer(&state.status, &action),
        winner: winner_reducer(&state.winner, &action)
    }
}

#[derive(Clone)]
pub enum BoardAction {
    Update(usize, usize, Pieces)
}

fn board_reducer(state: &Board, action: &Action) -> Board {
    let mut new_board: Board = state.clone();

    match *action {
        Action::BoardUpdate(ref board_action) => match *board_action {
            BoardAction::Update(x, y, ref token) => {
                new_board.update(x, y, token);
            },
        },
        _ => ()
    }

    new_board
}

#[derive(Clone)]
pub enum StatusAction {
    Update(GameStatus)
}

fn status_reducer(state: &GameStatus, action: &Action) -> GameStatus {
    let mut new_status: GameStatus = state.clone();

    match *action {
        Action::Status(ref status_action) => match *status_action {
            StatusAction::Update(ref status) => {
                new_status = status.clone();
            }
        },
        _ => ()
    }

    new_status
}

#[derive(Clone)]
pub enum WinnerAction {
    Update(Pieces)
}

fn winner_reducer(winner: &Pieces, action: &Action) -> Pieces {
    let mut new_winner: Pieces = winner.clone();

    match *action {
        Action::Winner(ref winner_action) => match *winner_action {
            WinnerAction::Update(ref token) => {
                new_winner = token.clone();
            }
        },
        _ => ()
    }

    new_winner
}
