
// a compiler can only static analysis means is cannot detect the overflow issues present in the code or it cannot detect the logical error present in the code 

// the time have come to put back the semi-colon 
fn main() {
    let word :String = String::from("my world");

    // i need to get the first word of this sentance 

    let first:String= get_first_word(word);
    println!("first word is {}" , first)
}

fn get_first_word(word:String)->String{

    let mut ans:String = String::from("");
    for char in word.chars(){
        ans.push_str(char.to_string().as_str());
        if char == ' '{
            break;
        }
    }
    return  ans;
}

