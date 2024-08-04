
// a compiler can only static analysis means is cannot detect the overflow issues present in the code or it cannot detect the logical error present in the code 


fn main() {
    let word :String = String::from("hello world");
    for i in 1..3{
        println!("{}" , word);

    }
}

