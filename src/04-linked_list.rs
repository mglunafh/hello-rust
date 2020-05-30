use crate::List::*;

enum List {
    // Box<T> is a smart pointer for an object that is to be allocated on the heap.
    Node(u32, Box<List>),
    Nil,
}

// this is how code is attached to an enum
impl List {
    fn new() -> List {
        Nil
    }

    fn prepend(self, elem: u32) -> List {
        Node(elem, Box::new(self))
    }

    fn len(&self) -> u32 {
        // It is preferred to match a concrete type T and not a reference &T
        match self {
            Node(_elem, ref tail) => 1 + tail.len(),
            Nil => 0
        }
    }

    fn is_empty(&self) -> bool {
        match self {
            Node(_elem, ref _tail) => false,
            Nil => true
        }
    }

    fn stringify(&self) -> String {
        match *self {
            Node(elem, ref tail) => {
                // 'format!' returns heap-allocated string
                format!("{}, {}", elem, tail.stringify())
            }
            Nil => format!("Nil!")
        }
    }
}


fn main() {
    let mut list = List::new();
    list = list.prepend(15);
    list = list.prepend(9);
    list = list.prepend(4);
    list = list.prepend(3);
    list = list.prepend(1);

    let string_repr = list.stringify();
    println!("{}", string_repr);
    println!("Length is {}", list.len());
    println!("Is empty: {}", list.is_empty());
}
