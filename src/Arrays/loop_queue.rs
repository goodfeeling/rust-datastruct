use crate::Arrays::queue::Queue;
use std::fmt::Debug;

pub struct LoopQueue<T> {
    pub data: Box<[T]>,
    pub front: usize,
    pub tail: usize,
    pub size: usize,
}

impl<T> LoopQueue<T>
where
    T: Debug + Default + Copy + PartialEq + Clone + ToString,
{
    pub fn new(capacity: usize) -> LoopQueue<T> {
        LoopQueue {
            data: vec![T::default(); capacity].into_boxed_slice(),
            size: 0,
            front: 0,
            tail: 0,
        }
    }

    pub fn get_capacity(&self) -> usize {
        self.data.len() - 1
    }

    // 打印元素
    pub fn to_string(&self) -> String {
        let mut res = String::new();
        res.push_str(
            format!(
                "Queue: size = {},capacity = {}\n",
                self.size,
                self.get_capacity()
            )
            .as_str(),
        );
        res.push_str("front [");
        let mut i = self.front;
        loop {
            if i == self.tail {
                break;
            }

            res.push_str(self.data[i].to_string().as_str());
            if (i + 1) % self.data.len() != self.tail {
                res.push_str(",");
            }

            i = (i + 1) % self.data.len();
        }
        res.push_str("] tail");
        res
    }
}

impl<T> Queue<T> for LoopQueue<T>
where
    T: Debug + Default + Copy + PartialEq + Clone,
{
    fn get_size(&self) -> usize {
        self.size
    }

    fn is_empty(&self) -> bool {
        self.front == self.tail
    }

    fn enqueue(&mut self, e: T) {
        if (self.tail + 1) % self.data.len() == self.front {
            self.resize(true);
        }
        self.data[self.tail] = e;
        self.tail = (self.tail + 1) % self.data.len();
        self.size += 1;
    }

    fn dequeue(&mut self) -> Option<T>
    where
        T: Default,
    {
        if self.is_empty() {
            return None;
        }
        let ret = self.data[self.front];
        self.data[self.front] = T::default();
        self.front = (self.front + 1) % self.data.len();
        self.size -= 1;
        let capacity = self.get_capacity();
        if self.size == capacity / 4 && capacity / 2 != 0 {
            self.resize(false);
        }
        Some(ret)
    }

    fn get_front(&self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        Some(self.data[self.front])
    }

    fn resize(&mut self, is_inc: bool) {
        let new_capacity = if is_inc {
            self.get_capacity() * 2
        } else {
            self.get_capacity() / 2
        }; // Increase capacity
        let mut new_data = vec![T::default(); new_capacity].into_boxed_slice();

        for i in 0..self.size {
            new_data[i] = self.data[(i + self.front) % self.data.len()]
        }
        self.data = new_data;
        self.front = 0;
        self.tail = self.size;
    }

    fn get_capacity(&self) -> usize {
        self.data.len() - 1
    }
}
