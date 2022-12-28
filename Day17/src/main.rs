use std::fs;
use std::collections::HashSet;

const NUM_PIECES: i32 = 5;
const X_OFF: i32 = 2;
const Y_OFF: i32 = 4;

struct Piece{
    edges: HashSet<(i32, i32)>
}
struct Tetris{
    board: HashSet<(i32, i32)>,
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
            jets: _jet.chars().map(|x| if x == '>' {1} else {-1}).collect(),
            jet_iter: 0,
            piece_iter: 0,
            n: _jet.len() as i32,
            height: 0,
            l: -1,
            r: width
        }
    }

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

    fn get_plus(&self) -> HashSet<(i32, i32)> {
        let mut piece = HashSet::new();
        for x in 0..3{
            piece.insert((x + X_OFF, self.height + Y_OFF + 1 + (x % X_OFF)));
            piece.insert((x + X_OFF, self.height + Y_OFF + 1 - (x % X_OFF)));
        }
        return piece;
    }

    fn get_L(&self) -> HashSet<(i32, i32)>{
        let mut piece = HashSet::new();
        for x in 0..3{
            piece.insert((x + X_OFF, self.height + Y_OFF));
            piece.insert((4, self.height + Y_OFF + x));
        }
        return piece;
    }

    fn get_square(&self) -> HashSet<(i32, i32)> {
        let mut piece = HashSet::new();
        for x in 0..2{
            for y in 0..2 {
                piece.insert((x + X_OFF, self.height + Y_OFF + y));
            }
        }
        return piece;
    }
    
    fn get_piece(&self) -> HashSet<(i32, i32)> {
        match self.piece_iter % NUM_PIECES{
            0 => {return (0..4).map(|x| (x + X_OFF, self.height + Y_OFF)).collect::<HashSet<(_, _)>>();},
            1 => {return self.get_plus()},
            2 => {return self.get_L()},
            3 => {return (0..4).map(|x| (X_OFF, x + self.height + Y_OFF)).collect::<HashSet<(_, _)>>();},
            _ => {return self.get_square();},
        }
    }
    
    fn spawn_piece(&mut self){
        let mut piece = self.get_piece();
        let mut finished =  false;

        while !finished{
            let jet_dir = self.jets[(self.jet_iter%self.n) as usize];
            (piece, finished) = self.move_piece(piece, (jet_dir, 0));
            (piece, finished) = self.move_piece(piece, (0, -1));
            self.jet_iter += 1;
        }

        self.piece_iter += 1;

        for coord in piece{
            self.board.insert(coord);
            if coord.1 > self.height {self.height = coord.1;}
        }
        
    }

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
    for _ in 0..2022{
        game.spawn_piece();
        
    }
    //game.print_board();
    println!("{}", game.height);

}