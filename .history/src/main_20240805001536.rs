
// a compiler can only static analysis means is cannot detect the overflow issues present in the code or it cannot detect the logical error present in the code 


fn main() {
    let word :String = String::from("hello world");

    // i need to get the first word of this sentance 

    let FirstWord:String= getFirstWord(word);
    println!("first word is {}" , FirstWord)
}

fn getFirstWord(word:String)->String{
    let ret:String = String::from(word);
    return ret;
}

