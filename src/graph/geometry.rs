use rand::{Fill, Rng};

#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn distance_squared(&self, other: &Point) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        dx * dx + dy * dy
    }

    pub fn is_ccw(p0: &Point, p1: &Point, p2: &Point) -> bool {
        (p1.y - p0.y) * (p2.x - p1.x) - (p1.x - p0.x) * (p2.y - p1.y) > 0.0
    }

    fn circumdelta(p0: &Point, p1: &Point, p2: &Point) -> (f64, f64) {
        let dx = p1.x - p0.x;
        let dy = p1.y - p0.y;
        let ex = p2.x - p0.x;
        let ey = p2.y - p0.y;

        let bl = dx * dx + dy * dy;
        let cl = ex * ex + ey * ey;
        let d = 0.5 / (dx * ey - dy * ex);

        ((ey * bl - dy * cl) * d, (dx * cl - ex * bl) * d)
    }

    pub fn square_circumradius(p0: &Point, p1: &Point, p2: &Point) -> f64 {
        let (dx, dy) = Self::circumdelta(p0, p1, p2);
        dx * dx + dy * dy
    }

    pub fn circumcenter(p0: &Point, p1: &Point, p2: &Point) -> Point {
        let (dx, dy) = Self::circumdelta(p0, p1, p2);
        Point {
            x: p0.x + dx,
            y: p0.y + dy,
        }
    }

    pub fn in_circle(&self, p0: &Point, p1: &Point, p2: &Point) -> bool {
        let dx = p0.x - self.x;
        let dy = p0.y - self.y;
        let ex = p1.x - self.x;
        let ey = p1.y - self.y;
        let fx = p2.x - self.x;
        let fy = p2.y - self.y;

        let ap = dx * dx + dy * dy;
        let bp = ex * ex + ey * ey;
        let cp = fx * fx + fy * fy;

        dx * (ey * cp - bp * fy) - dy * (ex * cp - bp * fx) + ap * (ex * fy - ey * fx) < 0.0
    }
}

impl Fill for Point {
    fn fill<R: Rng + ?Sized>(&mut self, rng: &mut R) {
        self.x = rng.random();
        self.y = rng.random();
    }
}
