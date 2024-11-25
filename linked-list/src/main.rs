use list::{LinkedList, SinglyLinkList};

mod list;

fn main() {
    println!("Hello, world!");

    let mut list: SinglyLinkList<u32> = SinglyLinkList::new();

    list.add(0);
    list.add(2);
    list.add(4);
    list.add(6);
    list.add(8);
}
