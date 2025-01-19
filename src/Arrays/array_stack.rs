use super::arr::Array;
use std::fmt::Debug;
use crate::Arrays::stack::Stack;

pub struct ArrayStack<E: Clone + Default + Debug + PartialEq> {
    pub array: Array<E>,
}

impl<T: Clone + Default + Debug + PartialEq> ArrayStack<T> {
    pub fn new(capacity: usize) -> ArrayStack<T> {
        ArrayStack {
            array: Array::new(capacity),
        }
    }
    pub fn get_capacity(&self) -> usize {
        self.array.get_capacity()
    }

    // 打印元素
    pub fn to_string(&self) -> String {
        let mut res = String::new();
        res += "Stack:";
        res += "[";
        for i in 0..self.array.get_size() {
            res += format!("{:?}", self.array.get(i)).as_str();
            if i != self.array.get_size() - 1 {
                res += ", ";
            }
        }
        res += "] top";
        res
    }
}

impl<E: Clone + Default + Debug + PartialEq> Stack<E> for ArrayStack<E> {
    fn get_size(&self) -> usize {
        self.array.get_size()
    }

    fn is_empty(&self) -> bool {
        self.array.is_empty()
    }

    fn push(&mut self, e: E) {
        self.array.add_last(e);
    }

    fn pop(&mut self) -> Option<E> {
        self.array.pop()
    }

    fn peek(&self) -> Option<E> {
        self.array.get_last()
    }
}
