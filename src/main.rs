


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

#[derive(PartialEq,Eq,Clone,Copy,Debug)]
enum Move {
    BasicMove   (Pos, Pos),
    EnPassant   (Pos, Pos),
    Castling    (Pos, Pos),
    Promotion   (Pos, Pos),
}



fn is_on_board(p:Pos) -> bool {
    p.0 >= 0 && p.0 < 8 && p.1 >= 0 && p.1 < 8
}

fn is_empty(p:Pos, board:&Board) -> bool {
    board_index(board, p) == Field::Empty
}

fn is_enemy(p:Pos, color:Color, board:&Board) -> bool {
    match board_index(board, p) {
        Field::Empty     => false,
        Field::Figure(f) => f.color != color
    }
}

fn is_empty_or_enemy(p:Pos, color:Color, board:&Board) -> bool {
    match board_index(board, p) {
        Field::Empty     => true,
        Field::Figure(f) => f.color != color
    }
}




fn board_index(board:&Board, p:Pos) -> Field {
    board_index_xy(board, p.0, p.1)
}

fn board_index_xy(board:&Board, x:i32, y:i32) -> Field {
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
        Rook    => '♖',
        Knight  => '♘',
        Bishop  => '♗',
        Queen   => '♕',
        King    => '♔',
    }
}

fn figure_kind_white_to_char(kind:FigureKind) -> char {
    match kind {
        Pawn    => '♟',
        Rook    => '♜',
        Knight  => '♞',
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



fn board_to_string(board:&Board) -> String {
    let mut s = String::with_capacity(11*9);
    s.push_str("   A B C D E F G H \n");
    for y in 0..8 {
        s.push_str(&format!(" {}", 8-y));
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



fn board_apply_valid_move(board:&mut Board, mymove:Move) {
    use Move::*;
    match mymove {
        BasicMove(s, d) => board_apply_basic_move(board, s, d),
        _               => panic!("I'm sorry Dave, I'm afraid I can't do that."),
    }
}

fn board_apply_basic_move(board:&mut Board, s:Pos, d:Pos) {
    let field = board_index(board, s);
    board_set(board, d, field);
    board_set(board, s, Field::Empty);
}

fn board_get_valid_moves (b : &Board, c : Color) -> Vec<Move> {
    let mut moves = Vec::with_capacity(
        4*8     + // pawns
        14*4    + // rooks and bishops
        8*2     + // knights
        7*4     + // queen
        8       + // king
        2);       // castling
    for y in 0..8 {
        for x in 0..8 {
            if let Field::Figure(f) = board_index_xy(b, x, y) {
                if f.color == c {
                    figure_get_valid_moves(f, b, (x, y), &mut moves);
                }
            }
        }
    }
    moves
}

fn figure_get_valid_moves(
        figure  : Figure,
        board   : &Board,
        pos     : Pos,
        moves   : &mut Vec<Move>) {
    let f : fn(Pos, &Board, Color, &mut Vec<Move>) =
    match figure.kind {
        Pawn    => get_valid_moves_pawn,
        Knight  => get_valid_moves_knight,
        Rook    => get_valid_moves_rook,
        Bishop  => get_valid_moves_bishop,
        Queen   => get_valid_moves_queen,
        King    => get_valid_moves_king,
    };
    f(pos, board, figure.color, moves)
}

fn get_valid_moves_pawn(s:Pos, b:&Board, c:Color, moves:&mut Vec<Move>) {
    use Move::*;
    use Color::*;
    let (x, y)  = s;
    let d       = get_direction(c);    
    for p in [(x, y+d)].iter() {
        if is_on_board(*p) && is_empty(*p, b) {
            moves.push( BasicMove(s, *p) );
            if ((c == W && y == 6) || (c == B && y == 1))
                && is_empty((x, y+d+d), b) {
                moves.push( BasicMove(s, (x, y+d+d)) )
            }
        }
    }
    for p in [(x-1, y+d), (x+1, y+d)].iter() {
        if is_on_board(*p) && is_enemy(*p, c, b) {
            moves.push( BasicMove(s, *p) )
        }
    }
}

fn get_valid_moves_knight(s:Pos, b:&Board, c:Color, moves:&mut Vec<Move>) {
    let (x, y)  = s;
    let poses = [
        (x+1, y+2),
        (x+1, y-2),
        (x-1, y+2),
        (x-1, y-2),
        (x+2, y+1),
        (x+2, y-1),
        (x-2, y+1),
        (x-2, y-1),
    ];
    for p in poses.iter() {
        if is_on_board(*p) && is_empty_or_enemy(*p, c, b) {
            moves.push( Move::BasicMove(s, *p) )
        }
    }
}

fn get_valid_moves_check_line(
        s : Pos, (dx, dy) : Pos,
        b : &Board, c : Color,
        moves : &mut Vec<Move>) {
    use Move::*;
    let (mut x, mut y) = s;
    loop {
        x += dx;
        y += dy;
        if !is_on_board((x, y)) {
            break;
        }
        if is_empty((x, y), b) {
            moves.push( BasicMove(s, (x, y)) );
            continue;
        }
        if is_enemy((x, y), c, b) {
            moves.push( BasicMove(s, (x, y)) );
        }
        break;
    }
}

fn get_valid_moves_rook(s:Pos, b:&Board, c:Color, moves:&mut Vec<Move>) {
    for p in [
        ( 0,  1),
        ( 0, -1),
        ( 1,  0),
        (-1,  0)].iter() {
        get_valid_moves_check_line(s, *p, b, c, moves);
    }
}

fn get_valid_moves_bishop(s:Pos, b:&Board, c:Color, moves:&mut Vec<Move>) {
    for p in [
        ( 1,  1),
        ( 1, -1),
        (-1,  1),
        (-1, -1)].iter() {
        get_valid_moves_check_line(s, *p, b, c, moves);
    }
}

fn get_valid_moves_queen(s:Pos, b:&Board, c:Color, moves:&mut Vec<Move>) {
    for p in [
        ( 0,  1),
        ( 0, -1),
        ( 1,  0),
        (-1,  0),
        ( 1,  1),
        ( 1, -1),
        (-1,  1),
        (-1, -1)].iter() {
        get_valid_moves_check_line(s, *p, b, c, moves);
    }
}

fn get_valid_moves_king(s:Pos, b:&Board, c:Color, moves:&mut Vec<Move>) {
    let (x, y) = s;
    for p in [
        (x-1, y-1),
        (x-1, y+1),
        (x-1, y  ),
        (x+1, y-1),
        (x+1, y+1),
        (x+1, y  ),
        (x  , y-1),
        (x  , y+1)].iter() {
        if is_on_board(*p) && is_empty_or_enemy(*p, c, b) {
            moves.push( Move::BasicMove(s, *p) )
        }
    }
}



extern crate rand;

fn main() {
    let standard_board = "RNBKQBNRPPPPPPPP                                \
    pppppppprnbqkbnr";
    let board = &mut board_from_str(standard_board);
    println!("{}", board_to_string(board));
    let mut current_color = Color::B;
    for _ in 0..3 {
        let moves   = board_get_valid_moves(board, current_color);
        let len     = moves.len();
        if len > 0 {
            let index   = rand::random::<usize>() % len;
            let mymove  = moves[index];
            println!("{:?}", mymove);
            board_apply_valid_move(board, mymove);
        }
        println!("{}", board_to_string(board));
        {   // TODO :D, current_color.next()
            use Color::*;
            current_color = if current_color == W { B } else { W };
        }
    }
}

/*
fn main_interactive_loop (board : &mut Board) {
    loop {
        let input = &mut String::with_capacity(8);
        std::io::stdin().read_line(input).unwrap();
        if input == "q\n" {
            break;
        }
        let next_move = board_random_move(board);
        board_apply_move(board, next_move);
        println!("{}", board_to_string(*board));
    }
}
*/
