use heapless::sorted_linked_list::{LinkedIndexU8, Max, SortedLinkedList};

fn main() {
    let _: SortedLinkedList<u8, LinkedIndexU8, Max, 256> = SortedLinkedList::new_u8();
}
