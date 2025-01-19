use super::arr::Array;
use crate::Arrays::queue::Queue;
use std::fmt::Debug;

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
        for i in 0..self.array.get_size() {
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

    fn resize(&mut self, is_inc: bool) {
        todo!()
    }

    fn get_capacity(&self) -> usize {
        todo!()
    }
}
