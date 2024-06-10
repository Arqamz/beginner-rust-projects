// Define a struct for the node of the linked list
struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

// Define methods for the linked list
impl<T> Node<T> {
    // Create a new node
    fn new(data: T) -> Self {
        Node { data, next: None }
    }

    // Append a new node to the end of the list
    fn append(&mut self, data: T) {
        let mut current = self;
        while let Some(ref mut next) = current.next {
            current = next;
        }
        current.next = Some(Box::new(Node::new(data)));
    }

    // Print the elements of the list
    fn print(&self) {
        let mut current = self;
        while let Some(ref node) = current.next {
            print!("{} -> ", current.data);
            current = node;
        }
        println!("{}", current.data);
    }
}

fn main() {
    // Create a new linked list
    let mut list = Node::new(1);
    // Append some elements
    list.append(2);
    list.append(3);
    list.append(4);
    list.append(5);

    // Print the list
    list.print();
}
