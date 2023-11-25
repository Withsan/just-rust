fn main() {
    let v = vec![1, 2, 3, 4, 5, 6];
    let r = &v;
    println!("{}", r[0]);
    let aside = v;
}
