pub trait Queue<E> {
    fn get_size(&self) -> usize;
    fn get_capacity(&self) -> usize;
    fn is_empty(&self) -> bool;
    fn enqueue(&mut self, e: E);
    fn dequeue(&mut self) -> Option<E>;
    fn get_front(&self) -> Option<E>;
    fn resize(&mut self, is_inc: bool);
}
