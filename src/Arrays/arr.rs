use std::fmt::Debug;

#[derive(Clone, Debug)]
pub struct Array<T> {
    data: Box<[T]>,
    size: usize,
}

impl<T: Clone + Debug + PartialEq + Default> Array<T> {
    pub fn new(capacity: usize) -> Array<T> {
        Array {
            data: vec![T::default(); capacity].into_boxed_slice(),
            size: 0,
        }
    }

    // 获取数组容量
    pub fn get_capacity(&self) -> usize {
        self.data.len()
    }

    // 获取已存入数据
    pub fn get_size(&self) -> usize {
        self.size
    }

    // 判断是否为空
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    // 向数组中插入数据
    pub fn push(&mut self, value: T)
    where
        T: Clone,
    {
        if self.size == self.data.len() {
            // 扩展容量
            let new_capacity = self.size * 2;
            let mut new_data = vec![T::default(); new_capacity].into_boxed_slice();
            new_data[..self.size].clone_from_slice(&self.data[..self.size]);
            self.data = new_data;
        }
        self.data[self.size] = value;
        self.size += 1;
    }

    // 删除并返回数组中的最后一个元素
    pub fn pop(&mut self) -> Option<T> {
        if self.size == 0 {
            return None;
        }
        self.size -= 1;
        Some(self.data[self.size].clone())
    }

    // 获取数组中指定位置的元素（索引）
    pub fn get(&self, index: usize) -> Option<T> {
        Some(self.data[index].clone())
    }

    pub fn get_last(&self) -> Option<T> {
        self.get(self.size - 1)
    }

    pub fn get_first(&self) -> Option<T> {
        self.get(0)
    }

    // 在所有元素前添加一个新元素
    pub fn add_first(&mut self, e: T) {
        self.add(0, e);
    }

    // 在所有元素前添加一个新元素
    pub fn add_last(&mut self, e: T) {
        self.add(self.size, e);
    }

    // 在index索引的位置插入一个新元素e
    pub fn add(&mut self, index: usize, e: T) {
        if index > self.size {
            panic!("Add failed. Require index >= 0 and index <= size.")
        }
        if self.size == self.data.len() {
            self.resize(2 * self.data.len());
        }
        for i in (index..self.size).rev() {
            self.data[i + 1] = self.data[i].clone();
        }
        self.data[index] = e;
        self.size += 1;
    }

    // 修改index索引位置的元素
    pub fn set(&mut self, index: usize, e: T) {
        if index >= self.size {
            panic!("Get failed. Index is illegal.")
        }
        self.data[index] = e;
    }

    // 打印元素
    pub fn to_string(&self) -> String {
        let mut res = String::new();
        res += format!(
            "Array: size = {} , capacity = {}\n",
            self.size,
            self.data.len()
        )
        .as_str();

        res += "[";

        for i in 0..self.size {
            res += format!("{:?}", self.data[i]).as_str();
            if i != self.size - 1 {
                res += ", ";
            }
        }
        res += "]";
        res
    }

    // 查找数组中的是否有元素e
    pub fn contains(&self, e: T) -> bool {
        for v in self.data.iter() {
            if e == *v {
                return true;
            }
        }
        return false;
    }

    // 查找数组中元素e所在的索引，如果不存在元素e,则返回-1
    pub fn find(&self, e: T) -> Option<usize> {
        for i in 0..self.size {
            if self.data[i] == e {
                return Some(i);
            }
        }
        return None;
    }

    // 从数组中删除index位置的元素，返回删除的元素
    pub fn remove(&mut self, index: usize) -> Option<T> {
        if index >= self.size {
            panic!("Remove failed. Index is illegal.");
        }
        let ret = self.data[index].clone();
        for i in 0..self.size {
            self.data[i - 1] = self.data[i].clone();
        }
        self.size -= 1;
        self.data[self.size] = T::default();
        if self.size == self.data.len() / 4 && self.data.len() / 2 != 0 {
            self.resize(self.data.len() / 2);
        }
        Some(ret)
    }

    // 从数组中删除最后一个元素，返回删除的元素
    pub fn remove_last(&mut self) -> Option<T> {
        return self.remove(self.size - 1);
    }

    // 从数组中删除第一个元素，返回
    pub fn remove_first(&mut self) -> Option<T> {
        return self.remove(0);
    }

    // 从数组中删除元素e
    pub fn remove_element(&mut self, e: T) {
        if let Some(index) = self.find(e) {
            self.remove(index);
        };
    }

    // 将数组空间的容量变成new_capacity大小
    pub fn resize(&mut self, new_capacity: usize) {
        let mut new_data = vec![T::default(); new_capacity].into_boxed_slice();
        new_data[..self.size].clone_from_slice(&self.data[..self.size]);
        self.data = new_data;
    }
}
