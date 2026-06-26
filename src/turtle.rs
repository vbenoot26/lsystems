use crate::draw::Draw;

#[derive(Default)]
pub struct Turtle {
    posx: i32,
    posy: i32,
    angle: f32, // Radians
}

impl Turtle {
    pub fn forward<D: Draw>(&mut self, dist: i32, draw: &mut D) {
        let xnext = self.posx + (self.angle.cos() * dist as f32) as i32;
        let ynext = self.posy + (self.angle.sin() * dist as f32) as i32;

        draw.draw_line(self.posx, self.posy, xnext, ynext);

        self.posx = xnext;
        self.posy = ynext;
    }

    pub fn rotate(&mut self, angle: f32) {
        self.angle += angle;
    }

    pub fn relocate(&mut self, newx: i32, newy: i32, angle: f32) {
        self.posx = newx;
        self.posy = newy;
        self.angle = angle
    }
}
