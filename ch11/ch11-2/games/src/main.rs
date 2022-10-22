pub struct Canvas {
    c: Vec<Vec<char>>
}
impl Canvas {
    fn new(w: usize, h: usize) -> Canvas {
        let v = vec![vec!['-';w];h];
        Canvas {c: v,}
    }

    fn write_at(&mut self, x: i32, y: i32, v: char) -> std::io::Result<()> {
        self.c[y as usize][x as usize] = v;
        Ok(())
    }
}

// A trait for characters, items, and scenery -
// Anything in the game world that's visible on screen.
trait Visible {
    // Render this object on the given canvas.
    fn draw(&self, canvas: &mut Canvas);
    // Return tru if clicking at (x, y) should
    // select this object.
    fn hit_test(&self, x: i32, y: i32) -> bool;
}

pub struct Broom {
    x: i32,
    y: i32,
    height: i32,
}
impl Visible for Broom {
    fn draw(&self, canvas: &mut Canvas) {
        for y in self.y - self.height - 1 .. self.y {
            canvas.write_at(self.x, y, '|').unwrap();
        }
        canvas.write_at(self.x, self.y, 'M').unwrap();
    }

    fn hit_test(&self, x: i32, y: i32) -> bool {
        self.x == x && self.y - self.height - 1 <= y && y <= self.y
    }
}

fn main() {
    let mut c = Canvas::new(1000, 1000);
    let b = Broom {x:10, y:10, height:1};
    println!("{:?}", c.c);

    b.draw(&mut c);
    println!("{:?}", c.c);
    b.hit_test(10, 5);
}
