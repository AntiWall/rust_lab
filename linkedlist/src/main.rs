struct Node {
    value : u32,
    next: Option<Box<Node>>,
}

struct LinkedList {
    head: Option<Box<Node>>,
    size: usize,
}

impl Node {
    pub fn new(value : u32, next: Option<Box<Node>>) -> Node {
        Node {value: value, next: next}
    }
}

impl LinkedList {
    pub fn new() -> LinkedList{
        LinkedList { head: None, size: 0 }
    }
    pub fn get_size(&self) -> usize {
        self.size
    }
    pub fn create3() -> usize {
        35
    }
    pub fn is_empty(&self) -> bool {
        self.get_size() == 0
    }
    pub fn push(&mut self, value: u32) {
        let new_node: Option<Box<Node>> = Some(Box::new(Node::new(value, self.head.take())));
        self.head = new_node;
        self.size += 1;
    }
    pub fn pop(&mut self) -> Option<u32> {
        let node = self.head.take()?;
        self.head = node.next;
        self.size -= 1;
        Some(node.value)
    }
    pub fn display(&self) {
        let mut current: &Option<Box<Node>> = &self.head;
        let mut result = String::new();
        loop {
            match current {
                Some(node) => {
                    result = format!("{} {}", result, node.value);
                    current = &node.next;
                },
                None => break,
            }
        }
        println!("{}", result)
    }
}
fn main() {
    // println!("Hello, world!");
    // let mut s = String::from("hello");
    // let s1 = &s;
    // // s1.push_str("asdasd");
    // println!("{}", s1);
    // s.push_str("asdasdsssssss");
    // println!("{}", s);

    let mut list : LinkedList = LinkedList::new();
    println!("{}", list.is_empty());
    println!("{}", LinkedList::create3());
    assert!(list.is_empty());
    assert_eq!(list.get_size(), 0);

    let mut x: Option<u32> = Some(5);
    let x_ref: &mut Option<u32> = &mut x;
    let y = x_ref.take();
    println!("result of take: {:#?}", y);
    println!("left at x: {:?}", x_ref);
    // let x: Box<u32> = Box::new(10);
    // println!("{}", *x);
    let u = y.unwrap_or(0);

    for i in 1..10 {
        list.push(i);
    }
    list.display();
    println!("list size: {}", list.get_size());
    println!("top element: {}", list.pop().unwrap());
    list.display();
}