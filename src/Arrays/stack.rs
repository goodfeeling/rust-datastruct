use std::fmt::Debug;

pub trait Stack<E>
where
    E: Clone + Default + Debug + PartialEq,
{
    fn get_size(&self) -> usize;
    fn is_empty(&self) -> bool;
    fn push(&mut self, e: E);
    fn pop(&mut self) -> Option<E>;
    fn peek(&self) -> Option<E>;
}
