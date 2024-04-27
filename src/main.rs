use sdl2::{pixels::Color, event::Event};

fn main() -> Result<(), String>{
    let (screen_width, screen_height): (u32, u32) = (800, 600);
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem.window("Ant Simulator", screen_width, screen_height)
        .build()
        .unwrap();


    let mut canvas = window.into_canvas()
        .build()
        .unwrap();
    // let screen_area = Rect::new(0, 0, screen_width, screen_height);
    // let clear_color = Color::RGB(64, 192, 255);

    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas.clear();
    canvas.present();

    let mut running = true;
    let mut event_queue = sdl_context.event_pump().unwrap();
    while running {

        for event in event_queue.poll_iter() {
            match event {
                Event::Quit {..} => {running = false;},
                _ => {}
            }
        }

        canvas.present();
    }


    Ok(())
}
