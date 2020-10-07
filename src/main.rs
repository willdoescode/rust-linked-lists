use std::ops::Deref;

mod singly_linked_list;

fn main() {
	let mut i = singlyLinkedList::Node::new(1);
	i.push_front(4);
	println!("{}", i.next.unwrap().deref().elem)
}
