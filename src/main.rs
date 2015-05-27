


#[derive(Clone,Copy)]
struct Board {
    fields: [[Field; 8]; 8]
}

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

type Pos  = (i32, i32);
type Move = (Pos, Pos);



fn is_on_board(p:Pos) -> bool {
    p.0 >= 0 && p.0 < 8 && p.1 >= 0 && p.1 < 8
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
    board_index_xy(board, p.0, p.1)
}

fn board_index_xy(board:Board, x:i32, y:i32) -> Field {
    board.fields[y as usize][x as usize]
}

fn board_set(board:&mut Board, p:Pos, field:Field) {
    board_set_xy(board, p.0, p.1, field);
}

fn board_set_xy(board:&mut Board, x:i32, y:i32, field:Field) {
    board.fields[y as usize][x as usize] = field;
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
            let field       = board_index(board, (x, y));
            s.push(' ');
            s.push(field_to_char(field, odd_field));
        }
        s.push('\n');
    }
    s
}



fn board_from_str(s:&str) -> Board {
    let mut fields = [[Field::Empty; 8]; 8];
    for (i, c) in s.char_indices().take(8*8) {
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



fn board_apply_move(board:&mut Board, move_:Move) {
    let (s, d) = move_;
    if !is_on_board(s) || !is_on_board(d) {
        return;
    }
    let field = board_index(*board, s);
    if let Field::Figure(figure) = field {
        if figure_move_is_valid(figure, move_, *board) {
            board_set(board, d, field);
            board_set(board, s, Field::Empty);
        }
    }
}

fn figure_move_is_valid(figure:Figure, move_:Move, board:Board) -> bool {
    let f : fn(Pos, Pos, Board, Color) -> bool =
    match figure.kind {
        Pawn    => move_is_valid_for_pawn,
        Knight  => move_is_valid_for_knight,
        Rook    => move_is_valid_for_rook,
        Bishop  => move_is_valid_for_bishop,
        Queen   => move_is_valid_for_queen,
        King    => move_is_valid_for_king,
    };
    f(move_.0, move_.1, board, figure.color)
}

fn move_is_valid_for_pawn(s:Pos, d:Pos, b:Board, c:Color) -> bool {
    let dir = get_direction(c);
        (s.0 == d.0 && d.1 == s.1 + dir
        && is_empty(d, b))
    ||  ((s.0 - d.0).abs() == 1 && d.1 == s.1 + dir
        && is_enemy(d, c, b))
}

#[allow(unused_variables)]
fn move_is_valid_for_knight(s:Pos, d:Pos, b:Board, c:Color) -> bool {
    true
}

#[allow(unused_variables)]
fn move_is_valid_for_rook(s:Pos, d:Pos, b:Board, c:Color) -> bool {
    true
}

#[allow(unused_variables)]
fn move_is_valid_for_bishop(s:Pos, d:Pos, b:Board, c:Color) -> bool {
    true
}

#[allow(unused_variables)]
fn move_is_valid_for_queen(s:Pos, d:Pos, b:Board, c:Color) -> bool {
    true
}

#[allow(unused_variables)]
fn move_is_valid_for_king(s:Pos, d:Pos, b:Board, c:Color) -> bool {
    true
}





fn main() {
    let standard_board = "NRBKQBRNPPPPPPPP                                \
    ppppppppnrbqkbrn";
    let board = &mut board_from_str(standard_board);
    println!("{}", board_to_string(*board));
    board_apply_move(board, ((0, 1), (0, 2)));
    println!("{}", board_to_string(*board));
    board_apply_move(board, ((0, 0), (0, 3)));
    println!("{}", board_to_string(*board));
}
