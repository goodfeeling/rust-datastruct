use std::fmt::Debug;

use super::arr::Array;

pub trait Queue<E> {
    fn get_size(&self) -> usize;
    fn is_empty(&self) -> bool;
    fn enqueue(&mut self, e: E);
    fn dequeue(&mut self) -> Option<E>;
    fn get_front(&self) -> Option<E>;
}

pub struct ArrayQueue<T> {
    pub array: Array<T>,
}

impl<T: Default + Clone + Copy + Debug + PartialEq> ArrayQueue<T> {
    pub fn new(capacity: usize) -> ArrayQueue<T> {
        ArrayQueue {
            array: Array::new(capacity),
        }
    }

    // 打印元素
    pub fn to_string(&self) -> String {
        let mut res = String::new();
        res += "Queue:";
        res += "front [";
        for i in 0..self.get_size() {
            res += format!("{:?}", self.array.get(i)).as_str();
            if i != self.array.get_size() - 1 {
                res += ", ";
            }
        }
        res += "] tail";
        res
    }
}

impl<T: Default + Clone + Copy + Debug + PartialEq> Queue<T> for ArrayQueue<T> {
    fn get_size(&self) -> usize {
        self.array.get_size()
    }
    fn is_empty(&self) -> bool {
        self.array.is_empty()
    }
    fn get_front(&self) -> Option<T> {
        self.array.get_last()
    }
    fn dequeue(&mut self) -> Option<T> {
        self.array.remove_first()
    }
    fn enqueue(&mut self, e: T) {
        self.array.add_first(e);
    }
}
