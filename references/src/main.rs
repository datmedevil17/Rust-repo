fn main() {
    let mut x =5;
    x = x+1;
    let y = &mut x;
    *y=*y+1;
    println!("y is {} ",y); //auto dereferencing
}
