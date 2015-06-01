#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct Vector2d<V> {
    x: V,
    y: V
}
impl<V: Add + Sub + Mul + Div> Vector2d<V> {
    pub fn norm2(&self) -> V { self.x*self.x + self.y*self.y }
    pub fn norm(&self) -> V { self.norm2().sqrt() }
}
