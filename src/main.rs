use macroquad::prelude::*;

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

#[macroquad::main(conf)]
async fn main() {
    loop {
        clear_background(MAGENTA);

        draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        next_frame().await
    } 
}
