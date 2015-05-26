

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
enum FigureKind { Pawn, Knight, Rook, Bishop, Queen, King }
use FigureKind::*;

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
    board_index(board, p) == Field::Empty
}

fn is_enemy(p:Pos, color:Color, board:Board) -> bool {
    match board_index(board, p) {
        Field::Empty     => false,
        Field::Figure(f) => f.color != color
    }
}



fn board_index(board:Board, p:Pos) -> Field {
    board.fields[p.y as usize][p.x as usize]
}


fn get_direction(color:Color) -> i32 {
    return match color {
        Color::W => -1,
        Color::B => 1
    }
}



fn figure_kind_black_to_char(kind:FigureKind) -> char {
    match kind {
        Pawn    => '♙',
        Knight  => '♖',
        Rook    => '♘',
        Bishop  => '♗',
        Queen   => '♕',
        King    => '♔',
    }
}

fn figure_kind_white_to_char(kind:FigureKind) -> char {
    match kind {
        Pawn    => '♟',
        Knight  => '♜',
        Rook    => '♞',
        Bishop  => '♝',
        Queen   => '♛',
        King    => '♚',
    }
}

fn figure_to_char(figure:Figure) -> char {
    match figure {
        Figure { kind, color: Color::W } =>
            figure_kind_white_to_char(kind),
        Figure { kind, color: Color::B } =>
            figure_kind_black_to_char(kind),
    }
}

fn field_to_char(field:Field, odd:bool) -> char {
    let square = '\u{25A0}';
    match field {
        Field::Empty     => if odd { square } else { ' ' },
        Field::Figure(f) => figure_to_char(f),
    }
}



fn board_to_string(board:Board) -> String {
    let mut s = String::with_capacity(11*9);
    s.push_str("   A B C D E F G H \n");
    for y in 0..8 {
        s.push_str(&format!(" {}", y+1));
        for x in 0..8 {
            let odd_field   = (x + y * 9) % 2 == 1;
            let field       = board_index(board, Pos { x:x, y:y });
            s.push(' ');
            s.push(field_to_char(field, odd_field));
        }
        s.push('\n');
    }
    s
}



fn board_from_string(s:&str) -> Board {
    let mut fields = [[Field::Empty; 8]; 8];
    for (i, c) in s.char_indices() {
        fields[i / 8][i % 8] = field_from_char(c);
    }
    Board { fields: fields }
}



fn field_from_char(c:char) -> Field {
    if let Some(kind) = figure_kind_from_char(c) {
        let color = color_from_char(c);
        Field::Figure(Figure { kind:kind, color:color })
    } else {
        Field::Empty
    }
}

fn color_from_char(c:char) -> Color {
    if c.is_lowercase() {
        Color::W
    } else {
        Color::B
    }
}

fn figure_kind_from_char(c:char) -> Option<FigureKind> {
    match c.to_uppercase().next().unwrap() {
        'P' => Some(Pawn),
        'N' => Some(Knight),
        'R' => Some(Rook),
        'B' => Some(Bishop),
        'Q' => Some(Queen),
        'K' => Some(King),
        _   => None,
    }
}



fn main() {
    let standard_board = "NRBKQBRN\
    PPPPPPPP                                \
    pppppppp\
    nrbqkbrn";
    let board   = board_from_string(standard_board);
    println!("{}", board_to_string(board));
}


