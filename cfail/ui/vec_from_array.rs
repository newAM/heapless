use heapless::Vec;

fn main() {
    Vec::<u8, 4>::from_array([0; 5]);
}
