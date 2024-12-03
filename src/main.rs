use types::Matrix;

mod types;

fn main() {
    let m = Matrix::<3, usize>::from_vec(vec![1, 2, 3]).unwrap();
    let res = m * 100;
    println!("base matrix {:?}", res);
}
