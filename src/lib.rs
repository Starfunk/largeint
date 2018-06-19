/// The module,addition, is where scalar addition takes place.
/// The module, difference, is where the difference between two scalar values is calculated.

pub mod addition;
pub mod difference;


/// The module, largeint, is where the largeint struct is implemented along with a variety
/// of useful methods and functions. Update 0.2.0 will introduce the capability to multiply and
/// divide LargeInts. Currently only addition and subtraction are supported for the type LargeInt.

pub mod largeint {

    use difference::*;
    use addition::*;

/// Arithmetic is carried out on an instance of LargeInt. The scalar value is stored in a
/// in a String and its sign in the enum Sign.

    #[derive(PartialEq, Debug)]
    pub struct LargeInt {
         pub digits: String,
         pub sign: Sign,
    }

    #[derive(PartialEq, Debug)]
    pub enum Sign {
        Positive,
        Negative,
        Unsigned,
    }

    // An enum with 3 states to be used in the compare function.
    #[derive(PartialEq, Debug)]
    pub enum Compare {
        Larger,
        Equal,
        Smaller,
    }

    impl LargeInt {

        /// The get_int() and get_sign() methods are in place to make it simple to retrieve
        /// information about an instance of LargeInt in a simple way.

        // get_int() returns a copy of the scalar LargeInt value as a String.
        pub fn get_int(&self) -> String {
            let mut int =  String::new();
            int.push_str(&self.digits);
            int
        }

        // get_sign() returns the Sign of the LargeInt value as a String.
        pub fn get_sign(&self) -> String {
            let sign: String;
            if &self.sign == &Sign::Positive {
                sign = String::from("Positive");
            } else if &self.sign == &Sign::Negative {
                sign = String::from("Negative");
            } else {
                sign = String::from("Unsigned");
            }
            sign
        }

        // Compares two LargeInts scalar value. Output is Compare::Larger if large2 contains the larger
        // scalar value, Compare::Smaller if large2 is smaller, and Compare::Equal if both are equal.
        pub fn compare(&self, large2: &LargeInt) -> Compare {
            let mut flag = Compare::Larger;
            let large1 = to_vec(&self.digits);
            let large2 = to_vec(&large2.digits);

            if large1.len() < large2.len() {
                flag = Compare::Smaller;
            } else if large1 == large2 {
                flag = Compare::Equal;
            } else if large1.len() == large2.len() {
                for i in 0..large2.len() {
                    if &large2[i] > &large1[i] {
                        flag = Compare::Smaller;
                        break
                    }
                    else if &large2[i] < &large1[i] {
                        break
                    }
                    else {
                        continue
                    }
                }
            }
            flag
        }

        /// Depending on the Sign of the parameters, add() and subtract() will either call addition()
        ///or difference() and then assign the appropiate Sign to the result.

        pub fn add(&self, large2: &LargeInt) -> LargeInt {
            let digits: String;
            let sign: Sign;
            if &self.sign == &Sign::Positive && large2.sign == Sign::Positive {
                digits = addition(&self.digits, &large2.digits);
                sign = Sign::Positive;
            } else if &self.sign == &Sign::Positive && large2.sign == Sign::Negative {
                if &self.digits.len() > &large2.digits.len() {
                    digits = difference(&self.digits, &large2.digits);
                    sign = Sign::Positive;
                } else if &self.digits.len() < &large2.digits.len(){
                    digits = difference(&large2.digits, &self.digits);
                    sign = Sign::Negative;
                } else if &self.compare(&large2) == &Compare::Larger {
                    digits = difference(&self.digits, &large2.digits);
                    sign = Sign::Positive;
                } else if &self.compare(&large2) == &Compare::Smaller {
                    digits = difference(&large2.digits, &self.digits);
                    sign = Sign::Negative;
                } else {
                    digits = String::from("0");
                    sign = Sign::Unsigned;
                    }
            } else if &self.sign == &Sign::Negative && large2.sign == Sign::Positive {
                if &self.digits.len() > &large2.digits.len() {
                    digits = difference(&self.digits, &large2.digits);
                    sign = Sign::Negative;
                } else if &self.digits.len() < &large2.digits.len(){
                    digits = difference(&large2.digits, &self.digits);
                    sign = Sign::Positive;
                } else if &self.compare(&large2) == &Compare::Larger {
                    digits = difference(&self.digits, &large2.digits);
                    sign = Sign::Negative;
                } else if &self.compare(&large2) == &Compare::Smaller {
                    digits = difference(&large2.digits, &self.digits);
                    sign = Sign::Positive;
                } else {
                    digits = String::from("0");
                    sign = Sign::Unsigned;
                }
            } else if &self.sign == &Sign::Unsigned || large2.sign == Sign::Unsigned {
                match &self.sign {
                    &Sign::Unsigned => {
                        if large2.sign == Sign::Positive {
                            digits = large2.get_int();
                            sign = Sign::Positive;
                        } else if large2.sign == Sign::Negative {
                            digits = large2.get_int();
                            sign = Sign::Negative;
                        } else {
                            digits = String::from("0");
                            sign = Sign::Unsigned;
                        }
                    }
                    &Sign::Positive => {
                        digits = self.get_int();
                        sign = Sign::Positive;
                    }
                    &Sign::Negative => {
                        digits = self.get_int();
                        sign = Sign::Negative;
                    }
                }
            } else {
                digits = addition(&self.digits, &large2.digits);
                sign = Sign::Negative;
            }

            LargeInt {
                digits,
                sign,
            }
        }

        pub fn subtract(&self, large2: &LargeInt) -> LargeInt{
            let digits: String;
            let sign: Sign;
            if &self.sign == &Sign::Positive && large2.sign == Sign::Positive {
                if &self.digits.len() > &large2.digits.len() {
                    digits = difference(&self.digits, &large2.digits);
                    sign = Sign::Positive;
                } else if &self.digits.len() < &large2.digits.len(){
                    digits = difference(&large2.digits, &self.digits);
                    sign = Sign::Negative;
                } else if &self.compare(&large2) == &Compare::Larger {
                    digits = difference(&self.digits, &large2.digits);
                    sign = Sign::Positive;
                } else if &self.compare(&large2) == &Compare::Smaller {
                    digits = difference(&large2.digits, &self.digits);
                    sign = Sign::Negative;
                } else {
                    digits = String::from("0");
                    sign = Sign::Unsigned;
                }
            } else if &self.sign == &Sign::Positive && large2.sign == Sign::Negative {
                digits = addition(&self.digits, &large2.digits);
                sign = Sign::Positive;
            } else if &self.sign == &Sign::Negative && large2.sign == Sign::Positive {
                digits = addition(&self.digits, &large2.digits);
                sign = Sign::Negative;
            } else if &self.sign == &Sign::Negative && large2.sign == Sign::Negative {
                if &self.digits.len() > &large2.digits.len() {
                    digits = difference(&self.digits, &large2.digits);
                    sign = Sign::Negative;
                } else if &self.digits.len() < &large2.digits.len() {
                    digits = difference(&large2.digits, &self.digits);
                    sign = Sign::Positive;
                } else if &self.compare(&large2) == &Compare::Larger {
                    digits = difference(&self.digits, &large2.digits);
                    sign = Sign::Negative;
                } else if &self.compare(&large2) == &Compare::Smaller {
                    digits = difference(&large2.digits, &self.digits);
                    sign = Sign::Positive;
                } else {
                    digits = String::from("0");
                    sign = Sign::Unsigned;
                    }
            } else {
                match &self.sign {
                    &Sign::Unsigned => {
                        if large2.sign == Sign::Positive {
                            digits = large2.get_int();
                            sign = Sign::Negative;
                        } else if large2.sign == Sign::Negative {
                            digits = large2.get_int();
                            sign = Sign::Positive;
                        } else {
                            digits = String::from("0");
                            sign = Sign::Unsigned;
                        }
                    }
                    &Sign::Positive => {
                        digits = self.get_int();
                        sign = Sign::Positive;
                    }
                    &Sign::Negative => {
                        digits = self.get_int();
                        sign = Sign::Negative;
                    }
                }
            }
            LargeInt {
                digits,
                sign,
            }
        }
    }

    /// While Largeint and its fields have been made public, it is strongly recommended that
    /// LargeInt creation is done through the new() function which will instantiate an instance of
    /// LargeInt safely, making the necessary corrections to the fields if necessary, or panicking
    /// if an invalid input was submitted.

    pub fn new(digits: String, mut sign: Sign) -> LargeInt {
        let mut digits_clone = digits.clone();
        let digit_0 = &digits[0..1];
        if digits == String::from("") {
            panic!("The large integer value is undefined!")
        } else if digit_0 == "0" && digits.len() > 1 {
            panic!("Invalid LargeInt creation. Please remove the leading 0")
        } else if digit_0 == "0" && digits.len() == 1 {
            digits_clone = String::from("0");
            sign = Sign::Unsigned;
        } else if digit_0 != "0" && sign == Sign::Unsigned {
            panic!("Invalid LargeInt creation. Nonzero integers must be assigned
            either Sign::Positive or Sign::Negative")
        }
        let new_int = LargeInt {
            digits: digits_clone,
            sign: sign,
        };
        new_int
    }

    pub fn to_vec(input_str: &str) -> Vec<u8> {
        let mut vec: Vec<u8> = Vec::new();
        for i in input_str.chars() {
            let vec_input = i.to_digit(10).unwrap() as u8;
            vec.push(vec_input);
        }
        vec
    }

    pub fn vec_to_str(vec: &Vec<u8>) -> String {
        let mut output = String::from("");
        for i in vec {
            output.push_str(&i.to_string());
        }
        output
    }
}
