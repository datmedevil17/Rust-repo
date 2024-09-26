fn main() {
    // let num:u8 = 5;
    // println!("This is stored in {}", num);

    // let literal:&str = "This is String literal";
    // println!("{}",literal);
    // let mut literal2 = String::from("This is String literal");
    // literal2.push_str("Whats Up");
    // println!("{}",literal2);

    //Tuple 
    // let emp_info:(&str,u8)=("Ramesh",50);

    // // let emp_name = emp_info.0;
    // // let emp_age = emp_info.1;

    // let (emp_name,emp_age) = emp_info;
    // println!("Employee Name: {}, Age: {}",emp_name,emp_age);

    // print_value();

    //Ownership
    // let s1:String = String::from("hello");
    // let (s2,length) = calculate_length(s1);
    // println!("The length of '{}' is {}.", s2, length);

    //Borrowing 
    let mut s1:String=String::from("Hello");
    append_string(&mut s1);
    println!("{}",s1);
}
fn append_string(s3:&mut String){
    s3.push_str(" World");
}

// fn calculate_length(s:String)->(String,usize){
//     let length = s.len();
//     return (s,length);
// }
// fn print_value(){
//     println!("Hello World!!");
// }
