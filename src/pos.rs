#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq)]
pub struct Pos<T> {
    pub x: T,
    pub y: T,
}

impl<T> Pos<T>
where
    T: PartialEq,
{
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    pub fn compare(&self, p: &Pos<T>) -> bool {
        if (self.x == p.x) && (self.y == p.y) {
            return true;
        }
        false
    }
}
