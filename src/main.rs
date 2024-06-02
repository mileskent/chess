use macroquad::prelude::*;
use std::collections::HashMap;
use std::process;

const SIDE_LENGTH : i32 = 8;
const S: i32 = 100;

macro_rules! SQUARE_WIDTH {
    () => {
        (screen_width() / SIDE_LENGTH as f32) as i32
    };
}

macro_rules! SQUARE_HEIGHT {
    () => {
        (screen_height() / SIDE_LENGTH as f32) as i32
    };
}

fn m2XY() -> (i32, i32) {
    let (mut x, mut y) = mouse_position();
    x /= S as f32;
    y /= S as f32;
    (x as i32, y as i32)
}

fn i2xy(i: u32) -> (i32, i32) {
    let i_t = i as i32;
    let x = i_t % SIDE_LENGTH * SQUARE_WIDTH!();
    let y = i_t / SIDE_LENGTH * SQUARE_HEIGHT!();
    (x, y)
}

fn i2XY(i: u32) -> (i32, i32) {
    let i_t = i as i32;
    let x = i_t % SIDE_LENGTH;
    let y = i_t / SIDE_LENGTH;
    (x, y)
}

fn XY2i(x: i32, y: i32) -> u32 {
    (x + y*SIDE_LENGTH).try_into().unwrap()
}

fn conf() -> Conf {
    let size = (S * SIDE_LENGTH) as i32;
    Conf {
        window_title: "Chess - Prototype".to_string(), //this field is not optional!
        fullscreen:false,
        window_width:size,
        window_height:size,
        //you can add other options too, or just use the default ones:
        ..Default::default()
    }
}

fn is_in_bounds(X: i32, Y: i32) -> bool {
    X >= 0 || X < SIDE_LENGTH as i32 && Y >= 0 || Y < SIDE_LENGTH as i32
}

fn rook_isnt_blocked(i_start: u32, i_try: u32, pieces: Vec<char>) -> bool {
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
            if pieces[XY2i(x_square, ys) as usize] != 'X' {
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
            if pieces[XY2i(xs, y_square) as usize] != 'X' {
                isnt_blocked = false;
                break;
            }
        }
    }
    isnt_blocked
}


fn bishop_isnt_blocked(i_start: u32, i_try: u32, pieces: Vec<char>) -> bool {
    let (mut xs, mut ys) = i2XY(i_start);
    let (mut xt, mut yt) = i2XY(i_try);
    let mut bishop_isnt_blocked = true;

    for x_square in if xs < xt {(xs+1)..xt} else {xt..xs}  {
        let y = (yt - ys)/(xt - xs) * (x_square - xs) + ys;
        let captured_square = pieces[XY2i(x_square, y) as usize];
        if captured_square != 'X' && pieces[i_start as usize].is_ascii_uppercase() == captured_square.is_ascii_uppercase() {
            bishop_isnt_blocked = false;
            break;
        }
    }
    bishop_isnt_blocked
}

fn is_appropriate_move(pieces: Vec<char>, mut piece_char: char, i_start: u32, i_try: u32) -> bool {
    let (mut xs, mut ys) = i2XY(i_start);
    let (mut xt, mut yt) = i2XY(i_try);

    if xs == xt && ys == yt {
        return false;
    }

    // All moves need -> king cannot be captured next move
    // Castling needs -> Piece cannot capture any of the squares between the king and rook
    match piece_char.to_lowercase().next().unwrap() {
        'r' => {
           // in + 
            let is_plus = xs == xt || ys == yt;
            is_plus && rook_isnt_blocked(i_start, i_try, pieces)
        }
        'n' => {
            true 
        }
        'b' => {
            let is_on_same_diagonal = yt - ys == xt - xs || yt - ys == xs - xt;
            is_on_same_diagonal && bishop_isnt_blocked(i_start, i_try, pieces)
        }
        'q' => {
            true
        }
        'k' => {
            true
        }
        'p' => {
            // TODO en passent
            // TODO promotion, note: autoqueen for simplicity
            let is_white = piece_char == 'P';
            let color_scalar = is_white as i32 * 2 - 1;
            let moved_forward = yt - ys == -1 * color_scalar;
            let moved_forward_2 = yt - ys == -2 * color_scalar;
            let empty_facing = pieces[XY2i(xs, ys - 1 * color_scalar) as usize] == 'X';
            let is_first_move = if is_white {
                ys == SIDE_LENGTH - 2
            }
            else {
                ys == 1
            };

            let is_double_push = moved_forward_2 && xs == xt && is_first_move && empty_facing; 
            let is_normal_push = moved_forward && xs == xt && empty_facing;
            let is_take = moved_forward && (xs - xt).abs() == 1 && pieces[XY2i(xt, yt) as usize] != 'X';

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
//             boardi += c as u32 - ('0' as u32);
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
    for x in 0..SIDE_LENGTH {
        for y in 0..SIDE_LENGTH {
            draw_rectangle(
                (x * SQUARE_WIDTH!()) as f32,
                (y * SQUARE_HEIGHT!()) as f32,
                SQUARE_WIDTH!() as f32,
                SQUARE_HEIGHT!() as f32,
                if (x + y) % 2 == 0 {WHITE} else {BROWN});
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
    let mut last_selected: Option<u32> = None; // an i
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
            last_selected = Some(XY2i(click_x, click_y) as u32);
            if let Some(value) = last_selected {
                last_piece = pieces[value as usize];
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
                let now_selected = XY2i(a,b) as u32; 
                let now_piece = pieces[now_selected as usize];
                if let Some(value) = last_selected {

                    let is_opposite_color: bool = !(is_white(last_piece) == is_white(now_piece) && now_piece != 'X');

                    if is_opposite_color && is_appropriate_move(pieces.clone(),last_piece, value, now_selected){
                        legal = true;
                    }

                    // if illegal
                    if !legal {
                        pieces[value as usize] = last_piece;
                    } 
                    // if legal (move the piece)
                    else {
                        pieces[now_selected as usize] = last_piece;
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
        let (x,y) = m2XY();
        draw_rectangle(
                (x * SQUARE_WIDTH!()) as f32,
                (y * SQUARE_HEIGHT!()) as f32,
                SQUARE_WIDTH!() as f32,
                SQUARE_HEIGHT!() as f32,
                c 
                );
        
        

        next_frame().await
    } 
}
