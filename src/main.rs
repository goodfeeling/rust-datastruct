use Arrays::array_stack::{ArrayStack, Stack};

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
    let mut stack = ArrayStack::new(5);
    stack.push(1);
    stack.push(2);
    println!("{:?}", stack.peek());
    stack.push(6);
    println!("{:?}", stack.peek());
}
