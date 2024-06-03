use macroquad::prelude::*;
use std::collections::HashMap;
use std::process;

const SIDE_LENGTH : i8 = 8;
const S: i16 = 100;

macro_rules! SQUARE_WIDTH {
    () => {
        (screen_width() / SIDE_LENGTH as f32) as i16
    };
}

macro_rules! SQUARE_HEIGHT {
    () => {
        (screen_height() / SIDE_LENGTH as f32) as i16
    };
}

fn m2XY() -> (i8, i8) {
    let (mut x, mut y) = mouse_position();
    x /= S as f32;
    y /= S as f32;
    (x as i8, y as i8)
}

fn i2xy(i: usize) -> (f32, f32) { // xy are screen coordinates
    let i_t: i8 = i as i8;
    let x = (i_t % SIDE_LENGTH) as i16 * SQUARE_WIDTH!();
    let y = (i_t / SIDE_LENGTH) as i16 * SQUARE_HEIGHT!();
    (x as f32, y as f32)
}

fn i2XY(i: usize) -> (i8, i8) { // XY are board coordinates
    let i_t = i as i8;
    let x = i_t % SIDE_LENGTH;
    let y = i_t / SIDE_LENGTH;
    (x, y)
}

fn XY2i(x: i8, y: i8) -> usize {
    (x + y*SIDE_LENGTH).try_into().unwrap()
}

fn conf() -> Conf {
    let size = (S * SIDE_LENGTH as i16) as i32;
    Conf {
        window_title: "Chess - Prototype".to_string(), //this field is not optional!
        fullscreen:false,
        window_width:size,
        window_height:size,
        //you can add other options too, or just use the default ones:
        ..Default::default()
    }
}

fn is_in_bounds(X: i8, Y: i8) -> bool {
    X >= 0 || X < SIDE_LENGTH as i8 && Y >= 0 || Y < SIDE_LENGTH as i8
}

fn rook_isnt_blocked(i_start: usize, i_try: usize, pieces: Vec<char>) -> bool {
    // Assumes that is_plus is true
    let (mut xs, mut ys) = i2XY(i_start);
    let (mut xt, mut yt) = i2XY(i_try);
    let mut isnt_blocked: bool = true;
    if xs != xt { // horizontal
        if xs > xt {
            let mut t = xs;
            xs = xt;
            xt = t;
        }
        for x_square in (xs+1)..xt { // exclusive range because should be able to capture
            if pieces[XY2i(x_square, ys)] != 'X' {
                isnt_blocked = false;
                break;
            }
        }
    } 
    else { // vertical
        if ys > yt {
            let t = ys;
            ys = yt;
            yt = t;
        }
        for y_square in (ys+1)..yt { // exclusive range because should be able to capture
            if pieces[XY2i(xs, y_square)] != 'X' {
                isnt_blocked = false;
                break;
            }
        }
    }
    isnt_blocked
}


fn bishop_isnt_blocked(i_start: usize, i_try: usize, pieces: Vec<char>) -> bool {
    let (mut xs, mut ys) = i2XY(i_start);
    let (mut xt, mut yt) = i2XY(i_try);
    let mut bishop_isnt_blocked = true;

    for x_square in if xs < xt {(xs+1)..xt} else {xt..xs}  {
        let y = (yt - ys)/(xt - xs) * (x_square - xs) + ys;
        let captured_square = pieces[XY2i(x_square, y)];
        if captured_square != 'X' && pieces[i_start].is_ascii_uppercase() == captured_square.is_ascii_uppercase() {
            bishop_isnt_blocked = false;
            break;
        }
    }
    bishop_isnt_blocked
}

fn is_appropriate_move(pieces: Vec<char>, mut piece_char: char, i_start: usize, i_try: usize) -> bool {
    let (mut xs, mut ys) = i2XY(i_start);
    let (mut xt, mut yt) = i2XY(i_try);

    if xs == xt && ys == yt {
        return false;
    }

    // All moves need -> king cannot be captured next move
    // Castling needs -> Piece cannot capture any of the squares between the king and rook
    // Need to create a list of all legal moves at some point which can replace these individual
    // functions and also be inspired by them
    match piece_char.to_lowercase().next().unwrap() {
        'r' => {
           // in + 
            let is_plus = xs == xt || ys == yt;
            is_plus && rook_isnt_blocked(i_start, i_try, pieces)
        }
        'n' => {
            let knight_moves: [(i8, i8); 8] = [(2,1),(2,-1),(1,2),(-1,2),(-2,1),(-2,-1),(-1,-2),(1,-2)];
            let delta_xy = (yt - ys, xt - xs);
            knight_moves.contains(&delta_xy)
        }
        'b' => {
            let is_on_same_diagonal = yt - ys == xt - xs || yt - ys == xs - xt;
            is_on_same_diagonal && bishop_isnt_blocked(i_start, i_try, pieces)
        }
        'q' => {
            let is_on_same_diagonal = yt - ys == xt - xs || yt - ys == xs - xt;
            let is_plus = xs == xt || ys == yt;
            if is_on_same_diagonal {
                bishop_isnt_blocked(i_start, i_try, pieces)
            }
            else if is_plus {
                rook_isnt_blocked(i_start, i_try, pieces)
            }
            else {
                false
            }
        }
        'k' => {
            let wont_kiss_king = {
                // where is the other king?
                let other_king: char = if piece_char.is_ascii_uppercase() {'k'} else {'K'};
                let other_king_i: usize;
                if let Some(index) = pieces.iter().position(|x| *x == other_king) {
                    other_king_i = index as usize; 
                }
                else {
                    eprintln!("ERROR! Missing a king!");
                    process::exit(1);
                }
                let (otherx, othery) = i2XY(other_king_i);
                (yt - othery).pow(2) + (xt - otherx).pow(2) > 2
            };
            (xt - xs).abs() <= 1 && (yt - ys).abs() <= 1 && wont_kiss_king
        }
        'p' => {
            // TODO en passent
            // TODO checks out of bounds at some point when at top of board for ex
            // TODO promotion, note: autoqueen for simplicity
            let is_white = piece_char == 'P';
            let color_scalar = is_white as i8 * 2 - 1;
            let moved_forward = yt - ys == -1 * color_scalar;
            let moved_forward_2 = yt - ys == -2 * color_scalar;
            let empty_facing = pieces[XY2i(xs, ys - 1 * color_scalar)] == 'X';
            let is_first_move = if is_white {
                ys == SIDE_LENGTH - 2
            }
            else {
                ys == 1
            };

            let is_double_push = moved_forward_2 && xs == xt && is_first_move && empty_facing; 
            let is_normal_push = moved_forward && xs == xt && empty_facing;
            let is_take = moved_forward && (xs - xt).abs() == 1 && pieces[XY2i(xt, yt)] != 'X';

            is_double_push || is_normal_push || is_take
        }
        _ => {
            false
        }
    }
}

fn draw_pieces(piece_textures: HashMap<char, Texture2D>, pieces: Vec<char>) {
    let mut boardi = 0;
    for c in &pieces {
        if *c != 'X' {
            let (x, y) = i2xy(boardi);
            draw_from_char(piece_textures.clone(), pieces.clone(), *c, x as f32, y as f32);
        }
        boardi += 1;
    }
}

fn draw_from_char (piece_textures: HashMap<char, Texture2D>, pieces: Vec<char>, c: char, x: f32, y: f32) {
    if c != 'X' {
        if let Some(texture) = piece_textures.get(&c) {
            // println!("Putting {c} on i: {boardi}, x: {x}, y: {y}");
            draw_texture_ex(
                &texture,
                x,
                y,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(Vec2::new(SQUARE_WIDTH!() as f32, SQUARE_HEIGHT!() as f32)),
                    ..Default::default()
                },
            );
        }
        else {
            eprintln!("ERROR! Texture not found. c: {:?}", c);
            process::exit(1);
        }
    }
}

// fn draw_pieces_fen(piece_textures: HashMap<char, Texture2D>, fen: &str) {
//     let mut boardi = 0;
//     for c in fen.chars() {
//         if c.is_digit(10) {
//             // println!("Skipping {c} squares");
//             boardi += c as u8 - ('0' as u32);
//         }
//         else if c == '/' {
//                 // println!("Next line");
//         }
//         else {
//             if let Some(texture) = piece_textures.get(&c) {
//                 let (x, y) = i2xy(boardi);
//                 // println!("Putting {c} on i: {boardi}, x: {x}, y: {y}");
//                 draw_texture_ex(
//                     &texture,
//                     x as f32,
//                     y as f32,
//                     WHITE,
//                     DrawTextureParams {
//                         dest_size: Some(Vec2::new(SQUARE_WIDTH!() as f32, SQUARE_HEIGHT!() as f32)),
//                         ..Default::default()
//                     },
//                 );
//             }
//             else {
//                 break;
//             }
//             boardi += 1;
//         }
//     }
// }

fn draw_board() {
    for X in 0..SIDE_LENGTH {
        for Y in 0..SIDE_LENGTH {
            draw_rectangle(
                (X as i16 * SQUARE_WIDTH!()) as f32,
                (Y as i16 * SQUARE_HEIGHT!()) as f32,
                SQUARE_WIDTH!() as f32,
                SQUARE_HEIGHT!() as f32,
                if (X + Y) % 2 == 0 {WHITE} else {BROWN});
        }
    }
}

fn is_white(c: char) -> bool {
    c.is_ascii_uppercase()
}

#[macroquad::main(conf)]
async fn main() {  


    // let fen: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";
    let mut pieces: Vec<char> ="rnbqkbnrppppppppXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXPPPPPPPPRBNQKBNR".chars().collect();
    let piece_textures = HashMap::from([
        ('r', load_texture("../images/r.png").await.unwrap()),
        ('n', load_texture("../images/n.png").await.unwrap()),
        ('b', load_texture("../images/b.png").await.unwrap()),
        ('q', load_texture("../images/q.png").await.unwrap()),
        ('k', load_texture("../images/k.png").await.unwrap()),
        ('p', load_texture("../images/p.png").await.unwrap()),
        ('R', load_texture("../images/R.png").await.unwrap()),
        ('N', load_texture("../images/N.png").await.unwrap()),
        ('B', load_texture("../images/B.png").await.unwrap()),
        ('Q', load_texture("../images/Q.png").await.unwrap()),
        ('K', load_texture("../images/K.png").await.unwrap()),
        ('P', load_texture("../images/P.png").await.unwrap()),
    ]);
    let mut dragging: bool = false;
    let mut last_selected: Option<usize> = None; // an i
    let mut last_piece: char = 'X';
    
    loop {
        clear_background(MAGENTA);
        draw_board();
        draw_pieces(piece_textures.clone(), pieces.clone());
          
        // Dragging

        let mut c: Color;
        if is_mouse_button_down(MouseButton::Left) && !dragging {
            c = Color::new(0.0, 0., 1., 0.2);

            dragging = true;
            let (click_x,click_y) = m2XY();
            // What piece did you click on?
            last_selected = Some(XY2i(click_x, click_y));
            if let Some(value) = last_selected {
                last_piece = pieces[value];
                pieces[value as usize] = 'X';
            } else {
                eprintln!("Beyond me if there's an error here");
                process::exit(1);
            }
        }
        else {
            c = Color::new(0.,0.,0.,0.1)
        }

        if dragging == true {

            if is_mouse_button_released(MouseButton::Left) {
                dragging = false;
                // Check if legal
                let mut legal: bool = false;
                let (a,b) = m2XY(); 
                let now_selected = XY2i(a,b); 
                let now_piece = pieces[now_selected];
                if let Some(value) = last_selected {

                    let is_opposite_color: bool = !(is_white(last_piece) == is_white(now_piece) && now_piece != 'X');

                    if is_opposite_color && is_appropriate_move(pieces.clone(),last_piece, value, now_selected){
                        legal = true;
                    }

                    // if illegal
                    if !legal {
                        pieces[value] = last_piece;
                    } 
                    // if legal (move the piece)
                    else {
                        pieces[now_selected] = last_piece;
                    }
                }
                else {
                    eprintln!("last_selected is not yet initialized but is being accessed");
                    process::exit(1);
                }
            }
            else {
                // Draw the piece on the mouse
                let (x, y) = mouse_position();
                draw_from_char(piece_textures.clone(), pieces.clone(), last_piece, x - (SQUARE_WIDTH!() / 2) as f32, y - (SQUARE_HEIGHT!() / 2) as f32);
            }
        }
        
        // Hilighting
        let (X,Y) = m2XY();
        draw_rectangle(
                (X as i16 * SQUARE_WIDTH!()) as f32,
                (Y as i16 * SQUARE_HEIGHT!()) as f32,
                SQUARE_WIDTH!() as f32,
                SQUARE_HEIGHT!() as f32,
                c 
                );
        
        

        next_frame().await
    } 
}
