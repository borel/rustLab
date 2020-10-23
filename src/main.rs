fn main() {
    let s = String::from("weber");
    
    let m = s.clone();
    // let s_bis = takes_ownership(s);    
    println!("The brandt is: {}", m);
}

// fn takes_ownership(some_string: String) { // some_string comes into scope
//     println!("{}", some_string);
// } 