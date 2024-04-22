// from_str.rs
//
// This is similar to from_into.rs, but this time we'll implement `FromStr` and
// return errors instead of falling back to a default value. Additionally, upon
// implementing FromStr, you can use the `parse` method on strings to generate
// an object of the implementor type. You can read more about it at
// https://doc.rust-lang.org/std/str/trait.FromStr.html
//
// Execute `rustlings hint from_str` or use the `hint` watch subcommand for a
// hint.

use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Person {
    name: String,
    age: usize,
}

// We will use this error type for the `FromStr` implementation.
#[derive(Debug, PartialEq)]
enum ParsePersonError {
    // Empty input string
    Empty,
    // Incorrect number of fields
    BadLen,
    // Empty name field
    NoName,
    // Wrapped error from parse::<usize>()
    ParseInt(ParseIntError),
}



// Steps:
// 1. If the length of the provided string is 0, an error should be returned
// 2. Split the given string on the commas present in it
// 3. Only 2 elements should be returned from the split, otherwise return an
//    error
// 4. Extract the first element from the split operation and use it as the name
// 5. Extract the other element from the split operation and parse it into a
//    `usize` as the age with something like `"4".parse::<usize>()`
// 6. If while extracting the name and the age something goes wrong, an error
//    should be returned
// If everything goes well, then return a Result of a Person object
//
// As an aside: `Box<dyn Error>` implements `From<&'_ str>`. This means that if
// you want to return a string error message, you can do so via just using
// return `Err("my error message".into())`.

impl FromStr for Person {
    type Err = ParsePersonError;
    fn from_str(s: &str) -> Result<Person, Self::Err> {
        // 1. If the length of the provided string is 0
        if s.is_empty() {
            return Err(ParsePersonError::Empty);
        }
        // 2. Split the given string on the commas present in it.
        let mut iter = s.split(',');
        // 3. Extract the first element from the split operation and use it as the name.
        // 4. If the name is empty, then return the default of Person.
        let mut name: String;
        let mut age: usize;
        if let Some(name_str) = iter.next() {
            if name_str.is_empty(){
                return Err(ParsePersonError::NoName)
            } else {
                name = name_str.to_string();
            }
        } else {
            return Err(ParsePersonError::BadLen);
        }

        // 5. Extract the other element from the split operation and parse it into a
        //    `usize` as the age.
        if let Some(age_str) = iter.next() {
            age = match age_str.parse::<usize>() {
                Ok(age_) => age_,
                Err(e) => return Err(ParsePersonError::ParseInt(e)),
            };
        } else {
            return Err(ParsePersonError::BadLen);
        }
        

        // 6. more than 2 elements after split
        if let Some(x) = iter.next() {
            return Err(ParsePersonError::BadLen);
        }
        Ok( Person { name, age })

    }
}

/*
* Other methods to convert Err()
* method1: use map_err # convert std::num::ParseIntError -> ParsePersonError::ParseInt <显式>
let age = age_str.parse::<usize>().map_err(ParsePersonError::ParseInt)?;

* method2: impl `From<ParseIntError>`  for `ParsePersonError` <隐式>
impl From<ParseIntError>  for ParsePersonError{
    fn from(err:ParseIntError) -> ParsePersonError{
        ParsePersonError::ParseInt(err)
    }
}
*
*/

fn main() {
    let p = "Mark,20".parse::<Person>().unwrap();
    println!("{:?}", p);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_input() {
        assert_eq!("".parse::<Person>(), Err(ParsePersonError::Empty));
    }
    #[test]
    fn good_input() {
        let p = "John,32".parse::<Person>();
        assert!(p.is_ok());
        let p = p.unwrap();
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 32);
    }
    #[test]
    fn missing_age() {
        assert!(matches!(
            "John,".parse::<Person>(),
            Err(ParsePersonError::ParseInt(_))
        ));
    }

    #[test]
    fn invalid_age() {
        assert!(matches!(
            "John,twenty".parse::<Person>(),
            Err(ParsePersonError::ParseInt(_))
        ));
    }

    #[test]
    fn missing_comma_and_age() {
        assert_eq!("John".parse::<Person>(), Err(ParsePersonError::BadLen));
    }

    #[test]
    fn missing_name() {
        assert_eq!(",1".parse::<Person>(), Err(ParsePersonError::NoName));
    }

    #[test]
    fn missing_name_and_age() {
        assert!(matches!(
            ",".parse::<Person>(),
            Err(ParsePersonError::NoName | ParsePersonError::ParseInt(_))
        ));
    }

    #[test]
    fn missing_name_and_invalid_age() {
        assert!(matches!(
            ",one".parse::<Person>(),
            Err(ParsePersonError::NoName | ParsePersonError::ParseInt(_))
        ));
    }

    #[test]
    fn trailing_comma() {
        assert_eq!("John,32,".parse::<Person>(), Err(ParsePersonError::BadLen));
    }

    #[test]
    fn trailing_comma_and_some_string() {
        assert_eq!(
            "John,32,man".parse::<Person>(),
            Err(ParsePersonError::BadLen)
        );
    }
}
