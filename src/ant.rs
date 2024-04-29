use sdl2::{render::Canvas, video::Window, rect::Rect, pixels::Color};

pub struct Ant {
    x: f32,
    y: f32,
    width: u32,
    height: u32,
    color: Color,
    speed: u32
}

impl Ant {

    pub fn new(x: i32, y: i32, width: u32, height: u32, color: Color, speed: u32) -> Ant {
        Ant {x: x as f32, y: y as f32, width: width, height: height, color: color, speed: speed}
    }

    pub fn set_pos(&mut self, pos: (Option<&i32>, Option<&i32>)) {
        if pos.0.is_some() {
            self.x = *pos.0.unwrap() as f32;
        }

        if pos.1.is_some() {
            self.y = *pos.1.unwrap() as f32;
        }
        
    }

    pub fn set_size(&mut self, size: (Option<&u32>, Option<&u32>)) {
        if size.0.is_some() {
            self.width = *size.0.unwrap();
        }

        if size.1.is_some() {
            self.height = *size.1.unwrap();
        }
        
    }

    pub fn set_color(&mut self, color: &Color) {
        self.color = *color;
    }

    pub fn set_speed(&mut self, speed: &u32) {
        self.speed = *speed;
    }

    pub fn move_ant(&mut self, dir: (&i32, &i32)) {
        let normal: f32 = f32::sqrt((dir.0.pow(2) + dir.1.pow(2)) as f32);
        if normal == 0.0 {
            return
        }
        
        self.x += ((*dir.0 as f32) / normal) * (self.speed as f32);
        self.y += ((*dir.1 as f32) / normal) * (self.speed as f32);

    }

    pub fn draw_ant(&self, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(self.color);
        let ant: Rect = Rect::new(self.x as i32, self.y as i32, self.width, self.height);
        canvas.fill_rect(ant);
    }
}