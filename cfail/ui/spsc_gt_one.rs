use heapless::spsc::Queue;

fn main() {
    let _: Queue<u8, 1> = Queue::new();
}
