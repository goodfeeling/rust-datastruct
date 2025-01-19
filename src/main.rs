use crate::Arrays::loop_queue::LoopQueue;
use crate::Arrays::queue::Queue;
mod Arrays;

#[derive(Debug, Clone, PartialEq, Default)]
struct Student {
    name: String,
}
impl Student {
    pub fn new(name: String) -> Student {
        Student { name: name }
    }
}
fn main() {
    // let mut arr = Arrays::arr::Array::new(2);
    // println!("容量1={}", arr.get_capacity());
    // let p1 = Student::new("张三".to_string());
    // arr.push(p1);
    // arr.add_first(Student::new("李四".to_string()));
    // println!("容量2={}", arr.get_capacity());
    // arr.add_first(Student::new("李四".to_string()));
    // println!("容量3={}", arr.get_capacity());
    // println!("{}", arr.to_string())

    // let mut stack = ArrayStack::new(5);
    // stack.push(1);
    // stack.push(2);
    // stack.push(6);
    // println!("{}", stack.to_string());

    // let mut queue = ArrayQueue::new(5);
    // queue.enqueue(2);
    // queue.enqueue(3);
    // queue.enqueue(5);
    // println!("{}", queue.to_string());
    let mut lq = LoopQueue::new(5);
    lq.enqueue(1);
    lq.enqueue(2);
    lq.enqueue(3);
    lq.dequeue();
    println!("{}", lq.to_string());
}
