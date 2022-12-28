use std::fs;
use std::collections::{HashSet, HashMap};

const NUM_PIECES: i32 = 5;
const X_OFF: i32 = 2;
const Y_OFF: i32 = 4;
const P2_ITER: i64 = 1000000000000;

struct Tetris{
    board: HashSet<(i32, i32)>,
    past_states: HashMap<(Vec<(i32, i32)>, i32, i32), (i32, i32)>,
    jets: Vec<i32>,
    jet_iter: i32,
    piece_iter: i32,
    n: i32,
    height: i32,
    l: i32,
    r: i32
}

impl Tetris {
    fn new(width: i32, _jet: String) -> Tetris{
        let mut _board = (0..width).map(|x| (x,0_i32)).collect::<HashSet<(_, _)>>();
        Tetris{
            board: _board,
            past_states: HashMap::new(),
            jets: _jet.chars().map(|x| if x == '>' {1} else {-1}).collect(),
            jet_iter: 0,
            piece_iter: 0,
            n: _jet.len() as i32,
            height: 0,
            l: -1,
            r: width
        }
    }

    // Mixes the jet/downward movement by supplying the dir arg. Returns the board and if the piece stopped moving
    fn move_piece(&self, piece: HashSet<(i32, i32)>, dir: (i32, i32)) -> (HashSet<(i32, i32)>, bool){
        let mut new_loc: HashSet<(i32, i32)> = HashSet::new();
        for loc in &piece{
            let new_coord = (loc.0+dir.0, loc.1+dir.1);
            if self.board.contains(&new_coord) || new_coord.0 == self.l || new_coord.0 == self.r{
                return (piece, true);
            }
            new_loc.insert(new_coord);
        }
        return(new_loc, false)
    }

    // Generates the plus ignoring the middle
    fn get_plus(&self) -> HashSet<(i32, i32)> {
        let mut piece = HashSet::new();
        for x in 0..3{
            piece.insert((x + X_OFF, self.height + Y_OFF + 1 + (x % X_OFF)));
            piece.insert((x + X_OFF, self.height + Y_OFF + 1 - (x % X_OFF)));
        }
        return piece;
    }

    // Generates the L piece
    fn get_L(&self) -> HashSet<(i32, i32)>{
        let mut piece = HashSet::new();
        for x in 0..3{
            piece.insert((x + X_OFF, self.height + Y_OFF));
            piece.insert((4, self.height + Y_OFF + x));
        }
        return piece;
    }

    // Generates the square piece
    fn get_square(&self) -> HashSet<(i32, i32)> {
        let mut piece = HashSet::new();
        for x in 0..2{
            for y in 0..2 {
                piece.insert((x + X_OFF, self.height + Y_OFF + y));
            }
        }
        return piece;
    }
    
    // Finds and generates the current piece that needs to be spawned
    fn get_piece(&self) -> HashSet<(i32, i32)> {
        match self.piece_iter % NUM_PIECES{
            0 => {return (0..4).map(|x| (x + X_OFF, self.height + Y_OFF)).collect::<HashSet<(_, _)>>();},
            1 => {return self.get_plus()},
            2 => {return self.get_L()},
            3 => {return (0..4).map(|x| (X_OFF, x + self.height + Y_OFF)).collect::<HashSet<(_, _)>>();},
            _ => {return self.get_square();},
        }
    }

    // Gets the top 8 pieces of the board and returns them as a vector along with the current jet/piece state
    fn get_top(&mut self) -> (Vec<(i32,i32)>, i32, i32){
        const STACK: i32 = 8;
        let mut state: Vec<(i32,i32)> = vec![];
        for x in 0..self.r{
            for y in self.height-STACK..self.height+1{
                if self.board.contains(&(x, y)){
                    state.push((x,y-self.height));
                }
            }
        }
        return (state, self.jet_iter%self.n, self.piece_iter%NUM_PIECES);
    }

    // Spawns a new piece and iterates it's movement until it stops moving.
    // Returns if there is a cycle and (cycle height, iteration cycle occurs)
    fn spawn_piece(&mut self, i: i32) -> (bool, (i32, i32)){
        let mut piece = self.get_piece();
        let mut finished =  false;

        // Conduct the movement until the piece stops
        while !finished{
            let jet_dir = self.jets[(self.jet_iter%self.n) as usize];
            (piece, finished) = self.move_piece(piece, (jet_dir, 0));
            (piece, finished) = self.move_piece(piece, (0, -1));
            self.jet_iter += 1;
        }

        // Update next piece
        self.piece_iter += 1;

        // Add the piece coordinates to the board
        for coord in piece{
            self.board.insert(coord);
            if coord.1 > self.height {self.height = coord.1;}
        }

        // Check the top 8 for a cycle
        let state = self.get_top();

        // If the board has been seen before it is a cycle, return the info
        if self.past_states.contains_key(&state){
            return (true, *self.past_states.get(&state).unwrap());
        }

        // IF a cycle was not detected add the state as seen. Return val here is a placeholder
        self.past_states.insert(state, (self.height, i));
        return (false, (self.height, i))
        
    }

    // Prints the board with some excess space above the pieces
    fn print_board(&self){
        println!("{:?}", self.board);
        for y in (0..self.height+6).rev(){
            let mut line: String = "|".to_string();
            for x in 0..(self.r){
                if self.board.contains(&(x,y)){line.push('#');}
                else{line.push('.');}
            }
            line.push('|');
            println!("{}", line);
        }
    }
}



fn main() {
    let jet = fs::read_to_string("./src/input.txt").expect("Error reading file");
    let mut game = Tetris::new(7, jet);

    // Loop needs to be large enough to detect a cycle
    while(game.piece_iter < 100000){
        let i = game.piece_iter;
        let (cycle, cycle_info) = game.spawn_piece(i);

        // If a cycle is found, finish p1 and calculate p2
        if cycle {

            // Calculate the cycle information
            let cycle_height = (game.height - cycle_info.0) as i64;
            let cycle_length = (i - cycle_info.1) as i64;
            let n_left = P2_ITER - (i+1) as i64;
            let remaining_iter = (n_left) % cycle_length as i64;
            let n_cycles = (n_left)/cycle_length as i64;
            let cycle_added_height = n_cycles*cycle_height;

            // Do the excess iterations not covered by a full cycle and print p1 solution as it is contained
            // This is a rough solution as p1 does not have to be contained but just minor cleanup needed to modify this
            for j in 0..remaining_iter{
                game.spawn_piece(i+j as i32);
                if i as i64+j == 2021{
                    println!("P1 Height: {}", game.height);
                }
            }

            // P2 answer is just the current height + height of the full cycles
            println!("P2 Height: {}", game.height as i64+cycle_added_height);
            break;
        }    
    }

}