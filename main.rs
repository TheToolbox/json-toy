macro_rules! TODO { () => (unreachable!()) }

#[derive(Debug)]
pub enum JSON {
    Array(Vec<JSON>),
    Object(std::collections::HashMap<String, JSON>),
    String(String),
    Number(f64),
    Boolean(bool),
}

#[derive(Debug,PartialEq)]
enum ParserState {
    ExpectingItem,
    ExpectingComma,
    //ExpectingKey,
    ExpectingColon,
    
    ReadingString,
    ReadingNumber,

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
    ExpectingNothingElse
}

//struct ParsingError{}

impl JSON {
    ///Creates a new JSON object, parsing it from a string
    pub fn new(input: String) -> Result<JSON, &'static str> {
        use ParserState::*;
        let mut state = ExpectingItem;
        let mut object_stack: Vec<JSON> = vec![];
        let mut key_stack : Vec<Option<String>> = vec![];
        let mut start = 0;
        let mut retval : Result<JSON, &'static str> = Err("Unknown error occurred");
        
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
        //giving up on this for now
        //macro_rules! Whitespace { ($somepat:pat) => { ' ' | '\n' | '\t' | '\r' => } }
        macro_rules! ParsingErr { ($message:expr) => { return Err($message); } }
        macro_rules! PopObject {
            () => ({
                key_stack.pop();
                match object_stack.pop() {
                    Some(x) => CompleteItem!(x),
                    None => ParsingErr!("Unexpected ']' or '}'.")
                };
            })
        }
        macro_rules! CompleteItem {
            ($item:expr) => {
                match object_stack.last_mut() {
                    None => { retval = Ok($item); state = ExpectingNothingElse; },
                    Some(&mut JSON::Array(ref mut a)) => {a.push($item); state = ExpectingComma; },
                    Some(&mut JSON::Object(ref mut o)) => {
                        ///Possible that this will unwrap to None, needs evaluation
                        match key_stack.pop().unwrap() {
                            Some(k) => {o.insert(k,$item); key_stack.push(None); state = ExpectingComma; },
                            None => {
                                match $item {
                                    JSON::String(s) => { key_stack.push(Some(s)); state = ExpectingColon; },
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
                ExpectingItem => match c {
                        ' ' | '\n' | '\t' | '\r' => continue,
                        '[' => { object_stack.push(JSON::Array(vec![])); key_stack.push(None);},
                        '{' => { object_stack.push(JSON::Object(std::collections::HashMap::new())); key_stack.push(None);},
                        't' => state = ExpectingR,
                        'f' => state = ExpectingA,
                        '-' | '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {start = i; state = ReadingNumber},
                        '"' => { start = i + 1; state = ReadingString },
                        '}' | ']' => PopObject!(),
                        _ => ParsingErr!("Unexpected character encountered"),
                },
                ExpectingR => if c == 'r' { state = ExpectingU } else { ParsingErr!("Unexpected character. Expected an 'r'."); },
                ExpectingU => if c == 'u' { state = ExpectingTrueE } else { ParsingErr!("Unexpected character. Expected a 'u'."); },
                ExpectingTrueE => if c == 'e' { CompleteItem!(JSON::Boolean(true)); } else { ParsingErr!("Unexpected character. Expected an 'e'."); },
                ExpectingA => if c == 'a' { state = ExpectingL } else { ParsingErr!("Unexpected character. Expected an 'a'."); },
                ExpectingL => if c == 'l' { state = ExpectingS } else { ParsingErr!("Unexpected character. Expected an 'l'."); },
                ExpectingS => if c == 's' { state = ExpectingFalseE } else { ParsingErr!("Unexpected character. Expected an 's'."); },
                ExpectingFalseE => if c == 'e' { CompleteItem!(JSON::Boolean(false)); } else { ParsingErr!("Unexpected character. Expected an 'e'."); },
                ExpectingColon => match c { ':' => state = ExpectingItem, ' ' | '\n' | '\t' | '\r' => continue , _ => ParsingErr!("Expected Colon.") },
                ExpectingComma => match c { 
                        ',' => state = ExpectingItem,
                        '}' | ']' => PopObject!(),
                        ' ' | '\n' | '\t' | '\r' => continue,
                        _ => ParsingErr!("Expected a comma.")
                },
                ReadingNumber => match c {
                        '.' | '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => continue,
                        ///invalid numbers can get past here still, needs to be fixed TODO
                        '}' | ']' => { CompleteItem!(JSON::Number(input[start..i].parse::<f64>().unwrap())); PopObject!(); },
                        ',' => { CompleteItem!(JSON::Number(input[start..i].parse::<f64>().unwrap())); state = ExpectingItem; }
                        _ => { CompleteItem!(JSON::Number(input[start..i].parse::<f64>().unwrap()));}

                },
                IgnoringCharacter => continue,
                ReadingString => match c {
                        '\\' => { state = IgnoringCharacter; },
                        '"' => { CompleteItem!(JSON::String(input[start..i].to_owned())) },
                        _ => continue
                },
                ExpectingNothingElse => match c {
                    ' ' | '\n' | '\r' | '\t' => continue,
                    _ => ParsingErr!("Encountered unexpected character after JSON content")
                }
            }

        };
        if object_stack.len() > 0 {
            ParsingErr!("Unexpected end of data")
        } else {
            match state {
                ExpectingNothingElse => retval,
                //TODO improve Number implementation
                ReadingNumber => Ok(JSON::Number(input.parse::<f64>().unwrap())),
                _ => ParsingErr!("Unexpected end of data.")
            }
        }
    }
    
}

/// Testing function
fn test() {
    ///Confirm that we can construct objects as expected
    let _z = JSON::Array(vec![JSON::Boolean(true)]);
    ///Load up a few examples to test parsing
    let examples = vec![
    ///Test parsing a single string
    "\"strung\"",
    ///Empty array
    "[]",
    ///array of 1 string
    "[\"strang\"]",
    ///Array w/ two strings
    "[\"strang\",\"strunk\"]",
    "[\"strang\",\"strunk\",2]",
    "{}",
    "{\"grok\": \"jok\"}",
    "{\"grok\": \"jok\", \"gluckgluckgluckyeah\": {\"ooooh\": \"caribou!\"}}",
    "0",
    "10",
    "-23.3",
    ///A more complex test with an object, array, empty object, numbers, strings, bools, and whitespace galore
 "
    {
        \"testy\": {

        },
        \"clop\": {\"grob\": [3,4,\"33\", false]},
        \"clastic\": 34.3
}"];
    ///
    //println!("{:?}", examples);
    for e in &examples {
        let _a = JSON::new(e.to_string());
        //println!("{} \n\t\t\t\t\t=> {:?}", e, a);
    }
}

fn main() {
    ///Run tests
    test();
}
