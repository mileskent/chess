use macroquad::prelude::*;
use std::collections::HashMap;
use std::process;

const SIDE_LENGTH : u32 = 8;
const S: u32 = 100;

macro_rules! XW {
    () => {
        (screen_width() / SIDE_LENGTH as f32) as u32
    };
}

macro_rules! YW {
    () => {
        (screen_height() / SIDE_LENGTH as f32) as u32
    };
}

fn m2xy() -> (u32, u32) {
    let (mut x, mut y) = mouse_position();
    x /= S as f32;
    y /= S as f32;
    (x as u32, y as u32)
}

fn i2xy(i: u32) -> (u32, u32) {
   let x = (i % SIDE_LENGTH) * XW!();
   let y = i / SIDE_LENGTH * YW!();
   (x, y)
}

fn xy2i(x: u32, y: u32) -> u32 {
    x + y*SIDE_LENGTH
}

fn conf() -> Conf {
    let size = (S * SIDE_LENGTH) as i32;
    Conf {
        window_title: "Chess".to_string(), //this field is not optional!
        fullscreen:false,
        window_width:size,
        window_height:size,
        //you can add other options too, or just use the default ones:
        ..Default::default()
    }
}

fn draw_pieces(piece_textures: HashMap<char, Texture2D>, pieces: Vec<char>) {
    let mut boardi = 0;
    for c in pieces {
        if c != 'X' {
            if let Some(texture) = piece_textures.get(&c) {
                let (x, y) = i2xy(boardi);
                // println!("Putting {c} on i: {boardi}, x: {x}, y: {y}");
                draw_texture_ex(
                    &texture,
                    x as f32,
                    y as f32,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(Vec2::new(XW!() as f32, YW!() as f32)),
                        ..Default::default()
                    },
                );
            }
            else {
                break;
            }
        }
        boardi += 1;
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
//                         dest_size: Some(Vec2::new(XW!() as f32, YW!() as f32)),
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
                (x * XW!()) as f32,
                (y * YW!()) as f32,
                XW!() as f32,
                YW!() as f32,
                if (x + y) % 2 == 0 {WHITE} else {BROWN});
        }
    }
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
    let mut last_selected: Option<u32> = None;
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
            let (click_x,click_y) = m2xy();
            // What piece did you click on?
            last_selected = Some(xy2i(click_x, click_y) as u32);
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

        if dragging == true && is_mouse_button_released(MouseButton::Left) {
            dragging = false;
            if let Some(value) = last_selected {
                pieces[value as usize] = last_piece;
            }
            else {
                eprintln!("last_selected is not yet initialized but is being accessed");
                process::exit(1);
            }
        }

        let (x,y) = m2xy();
        draw_rectangle(
                (x * XW!()) as f32,
                (y * YW!()) as f32,
                XW!() as f32,
                YW!() as f32,
                c 
                );
        
        

        next_frame().await
    } 
}
