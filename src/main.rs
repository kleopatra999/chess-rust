

#[derive(Clone,Copy)]
struct Pos { x:i32, y:i32 }

#[derive(Clone,Copy)]
struct Board {
    fields: [[Field; 8]; 8]
}

type Moves = Vec<Pos>;

#[derive(PartialEq,Eq,Clone,Copy)]
enum Color { W, B }

#[derive(PartialEq,Eq,Clone,Copy)]
enum FigureKind { Pawn, Bishop }

#[derive(PartialEq,Eq,Clone,Copy)]
enum Field { Empty, Figure(Figure) }

#[derive(PartialEq,Eq,Clone,Copy)]
struct Figure {
    kind     : FigureKind,
    color    : Color
}

fn is_valid_move_pawn(p1:Pos, p2:Pos, color:Color, board:Board) -> bool {
    let dir = get_direction(color);
    is_on_board(p2) && ((
        p1.x == p2.x && p2.y == p1.y + dir
        && is_empty(p2, board))
    ||  ((p1.x - p2.x).abs() == 1 && p2.y == p1.y + dir
        && is_enemy(p2, color, board))
    )
}

fn is_on_board(p:Pos) -> bool {
    p.x >= 0 && p.x < 8 && p.y >= 0 && p.y < 8
}

fn is_empty(p:Pos, board:Board) -> bool {
    board_get_field(board, p) == Field::Empty
}

fn is_enemy(p:Pos, color:Color, board:Board) -> bool {
    match board_get_field(board, p) {
        Field::Empty     => false,
        Field::Figure(f) => f.color != color
    }
}



fn board_get_field(board:Board, p:Pos) -> Field {
    board.fields[p.y as usize][p.x as usize]
}



fn get_direction(color:Color) -> i32 {
    return match color {
        Color::W => -1,
        Color::B => 1
    }
}



fn main() {    
    let p1      = Pos { x: 0, y: 0 };
    let p2      = Pos { x: 0, y: 1 };
    let color   = Color::B;
    let board   = Board { fields: [[Field::Empty; 8]; 8] };
    let valid   = is_valid_move_pawn(p1, p2, color, board);
    println!("{:?}", valid);
}


