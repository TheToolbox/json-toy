macro_rules! TODO { () => (unreachable!()) }

#[derive(Debug)]
pub enum JSON {
    Array(Vec<JSON>),
    Object(std::collections::HashMap<String, JSON>),
    String(String),
    Number(f64),
    Boolean(bool),
}

enum ParserState {
    ExpectingItem,
    ExpectingComma,
    ExpectingKey,
    
    ReadingString,
    ReadingNumber,
    ReadingObject,
    ReadingArray,

    ///for booleans
    ExpectingR,
    ExpectingU,
    ExpectingTrueE,
    ExpectingA,
    ExpectingL,
    ExpectingS,
    ExpectingFalseE,
    ///to ignore double quotes in escaped strings
    IgnoringCharacter,
    ///to confirm no trailing nonsense (ie `{"a": "b"}3`)
    ExpectingNothingElse
}

impl JSON {
    pub fn new(input: String) {
        use ParserState::*;
        let mut state = ExpectingItem;
        let mut objectStack: Vec<JSON> = vec![];
        let mut key = &"";
        let mut stringstart = 0;
        let mut stringend = 0;
        //1: remove starting whitespace
        //for each character
        //if character is [, add JSON array to stack and as value (err if expecting key)
        //if character is {, add JSON object to stack and as value (err if expecting key)
        //if character is ", begin ReadingString, parse until ", ignoring \"
            //add completed string as key if none and an object is on the stack
            //, or add as element if object/array is on the stack
            //, or assert no other data and return string
        //if character is 0-9 or -, begin ReadingNumber, parse until control character or whitespace,
            //add as value if array is on stack or object is and key exists,
            //, or error if object is on stack but no key
            //, or assert no other data and return number
        //✓if character is t, assert next 3 are 'rue'
        //✓, or if f, assert next 4 are 'alse'
            //add as value if array is on stack or object is and key exists,
            //, or error if object is on stack but no key
            //, or assert no other data and return bool
        //if character is }, assert object is top of stack and pop it
            //if stack is empty, assert no other data and return object
        //if character is ], assert array is top of stack and pop it
            //if stack is empty, assert no other data and return array
        //after any value add, go into ExpectingComma, and return to 
        //if data ends with anything on the stack, error

        macro_rules! ParsingErr { ($message:expr) => (TODO!();) }
        macro_rules! ReturnIfNothingOnStack { ($val:expr) => (TODO!();) }

        for i in input.chars() {
            match state {
                ExpectingItem => {
                    match i {
                        ' ' | '\n' | '\t' | '\r' => continue,
                        '[' => objectStack.push(JSON::Array(vec![])),
                        '{' => objectStack.push(JSON::Object(std::collections::HashMap::new())),
                        't' => state = ExpectingR,
                        'f' => state = ExpectingA,
                        '"' => state = ReadingString,
                        '}' => { objectStack.pop(); },
                        ']' => {},
                        _ => unreachable!(),
                    }
                },
                ExpectingR => if i == 'r' { state = ExpectingU } else { ParsingErr!("Unexpected character. Expected an 'r'."); },
                ExpectingU => if i == 'u' { state = ExpectingTrueE } else { ParsingErr!("Unexpected character. Expected a 'u'."); },
                ExpectingTrueE => if i == 'e' { TODO!(); } else { ParsingErr!("Unexpected character. Expected an 'e'."); },
                ExpectingA => if i == 'a' { state = ExpectingL } else { ParsingErr!("Unexpected character. Expected an 'a'."); },
                ExpectingL => if i == 'l' { state = ExpectingS } else { ParsingErr!("Unexpected character. Expected an 'l'."); },
                ExpectingS => if i == 's' { state = ExpectingFalseE } else { ParsingErr!("Unexpected character. Expected an 's'."); },
                ExpectingFalseE => if i == 'e' { TODO!(); } else { ParsingErr!("Unexpected character. Expected an 'e'."); },
                ReadingString => { 
                    match i {
                        '\\' => { state = IgnoringCharacter; },
                        '"' => { ReturnIfNothingOnStack!(9); TODO!(); },
                        _ => unreachable!()
                    }
                },

                _ => unreachable!(),

            }

        }
    }
}

fn main() {
    let z = JSON::Array(vec![JSON::Boolean(true)]);
    let example = "
    {
        \"testy\": {

        },
        \"poop\": {\"grob\": [3,4,\"33\"]}
        \"clastic\": 34.3
    }";
    for i in example.chars() {
        print!("{}", i);
    }
    //parse(example);
    //let a = JsonObject::new();
    println!("{:?}", z);


}
