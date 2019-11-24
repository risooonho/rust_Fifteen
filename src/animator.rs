use graphics::math::Matrix2d;

///Todo change Direction and [f64;2] to Matrix2d
pub trait Animator {
    fn animate(&mut self, direction: Direction) -> [f64; 2];
    fn is_over(&self) -> bool;
    fn start(&mut self);
}

pub struct PlainAnimator {
    pub max: f64,
    pub step: f64,
    pub count: f64,
}

impl PlainAnimator {
    pub fn new(max: f64, step: f64, count: f64) -> Self {

        Self { max, step, count:max }
    }

    fn reset(&mut self) {
        self.count = 0.0;
    }

    fn resume_animate(&mut self, direction: Direction) -> [f64; 2] {
        if self.count < self.max {
            self.count += 2.0;
            let (x, y) = match direction {
                Direction::Top => (0.0, -(self.count)),
                Direction::Right => ( self.count, 0.0),
                Direction::Bottom => (0.0,  self.count),
                Direction::Left => (- self.count, 0.0)
            };
            [x, y]
        } else {[0.0, 0.0]}
    }
}

impl Animator for PlainAnimator {
    fn animate(&mut self, direction: Direction) -> [f64; 2] {
        if self.is_over() {
            [0.0, 0.0]
        } else {
            self.resume_animate(direction)
        }
    }


    fn is_over(&self) -> bool {
        if self.count >= self.max {
            true
        } else {
            false
        }
    }

    fn start(&mut self) {
        self.reset();
    }
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Copy, Clone)]
pub enum Direction {
    Top,
    Right,
    Bottom,
    Left,
}
