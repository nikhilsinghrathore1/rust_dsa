
// a compiler can only static analysis means is cannot detect the overflow issues present in the code or it cannot detect the logical error present in the code 


fn main() {
    let word :String = String::from("hello world");

    // i need to get the first word of this sentance 

    let first:String= get_first_word(word);
    println!("first word is {}" , first)
}

fn get_first_word(word:String)->String{
    let ret:String = String::from(word);
    return ret;
}

