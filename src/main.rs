use sdl2::{pixels::Color, event::Event, rect::Rect};
use rand::seq::SliceRandom;

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

    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas.clear();
    canvas.present();

    // Dummy ant
    let mut ant: Rect = Rect::new((screen_width / 2) as i32 , (screen_height / 2) as i32, 10, 10);
    canvas.set_draw_color(Color::RGB(72, 71, 70));
    canvas.fill_rect(ant);

    let mut event_queue = sdl_context.event_pump().unwrap();
    'running: loop {
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.clear();

        for event in event_queue.poll_iter() {
            match event {
                Event::Quit {..} => {break 'running;},
                _ => {}
            }
        }

        let opts = vec![-1, 0, 1];
        let x = opts.choose(&mut rand::thread_rng()).unwrap();
        let y = opts.choose(&mut rand::thread_rng()).unwrap();
        
        ant.offset(*x, *y);
        canvas.set_draw_color(Color::RGB(72, 71, 70));
        canvas.fill_rect(ant);

        canvas.present();
    }


    Ok(())
}
