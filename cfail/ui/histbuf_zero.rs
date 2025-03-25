use heapless::histbuf::HistoryBuffer;

fn main() {
    let _: HistoryBuffer<u8, 0> = HistoryBuffer::new();
}
