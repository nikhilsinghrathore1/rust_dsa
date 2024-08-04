// rust is multi threaded 
// a compiler can only static analysis means is cannot detect the overflow issues present in the code or it cannot detect the logical error present in the code 

// the time have come to put back the semi-colon 
fn main() {
    let _word :String = String::from("my world");

    // i need to get the first word of this sentance 
    // loops 
    // okay lets try to iterate over an string  
   // memory managemet in rust without worrying the dangling pointer error as there is no null value 
   // not having a garbage collector is one of the reason why rust is so fast 

//    mutability values can be changed but all the variable are immutable by default means value can't be changed  immutability assures that there is no race condition 

    let _num:u8 = 10;

    for  char in _word.chars(){ // .chars() return each character from the string also return the empty spaces 
            print!("{} ",char)
    }

        
}


// to define the function do fn _name_ (_arguments_:return_type) -> _function_return_type{
             // the return type are not infered    
            // logic
        
//}

fn _get_first_word(word:String)->String{

    let mut ans:String = String::from("");
    for char in word.chars(){
        ans.push_str(char.to_string().as_str());
        if char == ' '{
            break;
        }
    }
    return  ans;
}

