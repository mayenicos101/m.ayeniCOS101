fn main() {
    let v = vec![1,2,3,4,5,6,7,8];
    let x = vec![5,6,7,8,9,10,11];

    for i in 0..7 {
        let sum = v[i] + x[i];
        println!("{:?}", sum);
    }
}