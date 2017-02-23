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
    //ExpectingComma,
    //ExpectingKey,
    //ExpectingColon,
    
    ReadingString,
    //ReadingNumber,
    //ReadingObject,
    //ReadingArray,

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
    //to confirm no trailing nonsense (ie `{"a": "b"}3`)
    //ExpectingNothingElse
}

impl JSON {
    pub fn new(input: String) {
        use ParserState::*;
        let mut state = ExpectingItem;
        let mut object_stack: Vec<JSON> = vec![];
        let mut key : Option<String> = None;
        let mut stringstart = 0;
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

        for (i,c) in input.chars().enumerate() {
            match state {
                ExpectingItem => {
                    match c {
                        ' ' | '\n' | '\t' | '\r' => continue,
                        '[' => object_stack.push(JSON::Array(vec![])),
                        '{' => object_stack.push(JSON::Object(std::collections::HashMap::new())),
                        't' => state = ExpectingR,
                        'f' => state = ExpectingA,
                        '"' => { stringstart = i + 1; state = ReadingString },
                        '}' => { object_stack.pop(); },
                        ']' => {},
                        _ => unreachable!(),
                    }
                },
                ExpectingR => if c == 'r' { state = ExpectingU } else { ParsingErr!("Unexpected character. Expected an 'r'."); },
                ExpectingU => if c == 'u' { state = ExpectingTrueE } else { ParsingErr!("Unexpected character. Expected a 'u'."); },
                ExpectingTrueE => if c == 'e' { TODO!(); } else { ParsingErr!("Unexpected character. Expected an 'e'."); },
                ExpectingA => if c == 'a' { state = ExpectingL } else { ParsingErr!("Unexpected character. Expected an 'a'."); },
                ExpectingL => if c == 'l' { state = ExpectingS } else { ParsingErr!("Unexpected character. Expected an 'l'."); },
                ExpectingS => if c == 's' { state = ExpectingFalseE } else { ParsingErr!("Unexpected character. Expected an 's'."); },
                ExpectingFalseE => if c == 'e' { TODO!(); } else { ParsingErr!("Unexpected character. Expected an 'e'."); },
                //ExpectingComma => TODO!(),
                //ReadingNumber => TODO!(),
                ReadingString => { 
                    match c {
                        '\\' => { state = IgnoringCharacter; },
                        '"' => { 
                            let s = input[stringstart..i].to_owned();
                            //ReturnIfNothingOnStack!(s);
                            let z = object_stack.len() - 1;
                            match object_stack[z] {
                                JSON::Array(ref mut a) => a.push(JSON::String(s)),
                                JSON::Object(ref mut o) => {
                                    match key {
                                        Some(k) => { o.insert(k,JSON::String(s)); key = None; }
                                        None => { key = Some(s); }
                                    }
                                },
                                _ => unreachable!()
                            } 
                        },
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
