use uintarray::UintArray;

fn main() {
    let ua = UintArray::new_size(1);
    println!("{}", ua.append(1).append(2).append(4).insert(6, 1).0);
}
