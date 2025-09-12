#![allow(dead_code, unused_imports, unused_variables, unused_must_use)]

type Pointer = Option<Box<Node>>;

#[derive(Debug)]
struct Node {
    element: i32,
    next: Pointer,
}

#[derive(Debug)]
struct LinkList {
    head: Pointer,
}

impl LinkList {
    fn new() -> Self {
        LinkList { head: None }
    }

    fn append(&mut self, element: i32) {
        let new_node = Node {
            element,
            next: None,
        };

        if self.head.is_none() {
            self.head = Some(Box::new(new_node));
        } else {
            let mut current = self.head.as_mut();
            while let Some(node) = current {
                if node.next.is_none() {
                    node.next = Some(Box::new(new_node));
                    break;
                }
                current = node.next.as_mut();
            }
        }
    }

    fn peek(&self) -> Option<i32> {
        self.head.as_ref().map(|node| node.element)
    }
}

pub fn create_custom_linked_list() {
    let node = Node {
        element: 1,
        next: None,
    };

    let node2 = Node {
        element: 2,
        next: Some(Box::new(node)),
    };
    let node3 = Node {
        element: 3,
        next: Some(Box::new(Node {
            element: 2,
            next: Some(Box::new(Node {
                element: 1,
                next: None,
            })),
        })),
    };
}

pub fn create_custom_link_list() {
    let linked_list = LinkList {
        head: Some(Box::new(Node {
            element: 100,
            next: Some(Box::new(Node {
                element: 223,
                next: Some(Box::new(Node {
                    element: 456,
                    next: None,
                })),
            })),
        })),
    };

    let mut current = linked_list.head;

    while let Some(node) = current {
        println!("{}", node.element);
        current = node.next;
    }

    // println!("{:?}", &linked_list.head.unwrap().next.unwrap().next.unwrap().element);
}
