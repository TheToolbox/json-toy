macro_rules! TODO { () => (unreachable!()) }
use std::str::FromStr;


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
    //ExpectingKey,
    ExpectingColon,
    
    ReadingString,
    ReadingNumber,
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

struct ParsingError{}

impl JSON {
    ///Creates a new JSON object, parsing it from a string
    pub fn new(input: String) -> Result<JSON, &'static str> {
        use ParserState::*;
        let mut state = ExpectingItem;
        let mut object_stack: Vec<JSON> = vec![];
        let mut key : Option<String> = None;
        let mut start = 0;
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
        macro_rules! Whitespace { ($somepat:pat) => { ' ' | '\n' | '\t' | '\r' } }
        macro_rules! ParsingErr { ($message:expr) => {return Err($message);} }
        macro_rules! CompleteItem {
            ($item:expr) => {
                match object_stack.last_mut() {
                    None => return Ok($item),
                    Some(&mut JSON::Array(ref mut a)) => a.push($item),
                    Some(&mut JSON::Object(ref mut o)) => {
                        match key {
                            Some(k) => { o.insert(k,$item); key = None; },
                            None => {
                                match $item {
                                    JSON::String(s) => key = Some(s),
                                    _ => ParsingErr!("Expected key.")
                                }
                            },
                        }
                    },
                    _ => unreachable!()
                }
            }
            
        }

        for (i,c) in input.chars().enumerate() {
            match state {
                ExpectingItem => {
                    match c {
                        Whitespace!(' ') => continue,
                        '[' => object_stack.push(JSON::Array(vec![])),
                        '{' => object_stack.push(JSON::Object(std::collections::HashMap::new())),
                        't' => state = ExpectingR,
                        'f' => state = ExpectingA,
                        '-' | '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {start = i; state = ReadingNumber},
                        '"' => { start = i + 1; state = ReadingString },
                        '}' | ']' => { 
                            match object_stack.pop() {
                                Some(x) => CompleteItem!(x),
                                None => ParsingErr!("Unexpected closing bracket (] or }).")
                            }},
                        _ => unreachable!(),
                    }
                },
                ExpectingR => if c == 'r' { state = ExpectingU } else { ParsingErr!("Unexpected character. Expected an 'r'."); },
                ExpectingU => if c == 'u' { state = ExpectingTrueE } else { ParsingErr!("Unexpected character. Expected a 'u'."); },
                ExpectingTrueE => if c == 'e' { CompleteItem!(JSON::Boolean(true)); } else { ParsingErr!("Unexpected character. Expected an 'e'."); },
                ExpectingA => if c == 'a' { state = ExpectingL } else { ParsingErr!("Unexpected character. Expected an 'a'."); },
                ExpectingL => if c == 'l' { state = ExpectingS } else { ParsingErr!("Unexpected character. Expected an 'l'."); },
                ExpectingS => if c == 's' { state = ExpectingFalseE } else { ParsingErr!("Unexpected character. Expected an 's'."); },
                ExpectingFalseE => if c == 'e' { TODO!(); } else { ParsingErr!("Unexpected character. Expected an 'e'."); },
                ExpectingComma => {
                    match c {
                        ',' => state = ExpectingItem,
                        Whitespace!() => continue,
                        _ => ParsingErr!("Expected a comma.")
                    }
                },
                ReadingNumber => {
                    match c {
                        '.' | '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => continue,
                        ///invalid numbers can get past here still, needs to be fixed
                        _ => CompleteItem!(JSON::Number(input[start..i].parse::<f64>().unwrap()))
                    }
                },
                ExpectingColon => match c { ':' => state = ExpectingItem, Whitespace!() => continue , _ => ParsingErr!("Expected Colon.") },
                IgnoringCharacter => continue,
                ReadingString => { 
                    match c {
                        '\\' => { state = IgnoringCharacter; },
                        '"' => { 
                            let s = input[start..i].to_owned();
                            //ReturnIfNothingOnStack!(s);
                            let z = object_stack.len() - 1;
                            match object_stack[z] {
                                JSON::Array(ref mut a) => a.push(JSON::String(s)),
                                JSON::Object(ref mut o) => {
                                    match key {
                                        Some(k) => { o.insert(k,JSON::String(s)); key = None; }
                                        None => { key = Some(s); state = ExpectingColon; }
                                    }
                                },
                                _ => unreachable!()
                            } 
                        },
                        _ => unreachable!()
                    }
                },
            }

        };
       Err("Failed")
    }
    
}

/// Testing function
fn test() {
    ///Confirm that we can construct objects as expected
    let z = JSON::Array(vec![JSON::Boolean(true)]);
    ///Load up a few examples to test parsing
    let examples = [
    ///Test parsing a single string
    "\"string\"",
    ///A more complex test with an object, array, empty object, numbers, strings, bools, and whitespace galore
    "
    {
        \"testy\": {

        },
        \"poop\": {\"grob\": [3,4,\"33\", false]}
        \"clastic\": 34.3
    }"];
    ///
    print!("{:?}", examples);
    
    //let a = JsonObject::new();
    println!("{:?}", z);
}

fn main() {
    ///Run tests
    test();
}
