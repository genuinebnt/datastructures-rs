mod lists;

use lists::singly_linked_list::List;

fn main() {
    let mut list = List::new();

    list.push_front(5);

    println!("{:?}", list);
}
