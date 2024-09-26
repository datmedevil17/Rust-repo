fn main() {
    let ref_to_nothing = create_string();
}
fn create_string()->&String{
    let s:String = String::from("hello");
    return &s;
}