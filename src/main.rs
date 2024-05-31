use macroquad::prelude::*;

const SIDE_LENGTH : u32 = 8;
const SPOS : &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";

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

fn i2xy(i: u32) -> (u32, u32) {
   let x = (i % SIDE_LENGTH) * XW!();
   let y = i / SIDE_LENGTH * YW!();
   (x, y)
}

fn xy2i(x: u32, y: u32) -> u32 {
    x + y*SIDE_LENGTH
}

fn conf() -> Conf {
    let size = 800;
    Conf {
        window_title: "Chess".to_string(), //this field is not optional!
        fullscreen:false,
        window_width:size,
        window_height:size,
        //you can add other options too, or just use the default ones:
        ..Default::default()
    }
}

// fn draw_pieces() {
//     i
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

    
    loop {
        clear_background(MAGENTA);
        draw_board();

        // Draw Pieces
        let mut boardi = 0;
        for c in SPOS.chars() {
            if c.is_digit(10) {
                // println!("Skipping {c} squares");
                boardi += c as u32 - ('0' as u32);
            }
            else if c == '/' {
                 // println!("Next line");
            }
            else {
                let texture: Texture2D = load_texture(&format!("../images/{c}.png")).await.unwrap();
                texture.set_filter(FilterMode::Nearest);
                
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
                boardi += 1;
            }
        }
        

        // FPS limiter
        let minimum_frame_time = 1. / 10.;
        let frame_time = get_frame_time();
        println!("FPS: {}", 1. / frame_time);
        if frame_time < minimum_frame_time {
            let time_to_sleep = (minimum_frame_time - frame_time) * 1000.;
            std::thread::sleep(std::time::Duration::from_millis(time_to_sleep as u64));
        }

        next_frame().await
    } 
}
