use crate::draw::Draw;
struct Turtle<D: Draw> {
    posx: i32,
    posy: i32,
    angle: f32, // Radians?

    draw: D,
}

impl<D: Draw> Turtle<D> {
    fn forward(&mut self, dist: i32) {
        let xnext = self.posx + (self.angle.cos() * dist as f32) as i32;
        let ynext = self.posy + (self.angle.sin() * dist as f32) as i32;

        self.draw.drawLine(self.posx, self.posy, xnext, ynext);

        self.posx = xnext;
        self.posy = ynext;
    }

    fn rotate(&mut self, angle: f32) {
        self.angle += angle;
    }

    fn relocate(&mut self, newx: i32, newy: i32) {
        self.posx = newx;
        self.posy = newy;
    }
}
