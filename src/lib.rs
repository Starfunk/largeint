//! A library that implements the `LargeInt` struct, used for working with arbitrarily large integers in Rust.
//! The purpose of this library is to provide an easy-to-use large integer implementation in Rust.
//! The ideal user is one that is looking to write small scale projects for personal use and does
//! not want to spend time a lot of time learning a complex crate such as num-bigint. For example,
//! this library would be ideal for one looking to solve [Project Euler Problem 13](https://projecteuler.net/problem=13).
//! However, the largeint library is not particularly efficient and therefore it is recommended to use
//! a crate like num-bigint for more serious projects.

 mod addition;
 mod difference;
 mod multiplication;
 mod division;

/// Implements the LargeInt struct. Supported methods include addition, subtraction,
/// multiplication, division (floor division), and remainder from floor division. Also includes getters and a compare
/// method.

pub mod largeint {
    use difference::*;
    use addition::*;
    use multiplication::*;
    use division::*;

    /// Arithmetic is carried out on instances of LargeInt. The scalar value is stored
    /// in a String and its sign in the enum Sign.

    #[derive(PartialEq, Debug, Clone)]
    pub struct LargeInt {
        /// Stores the scalar value of the LargeInt as a String.
        pub digits: String,
        /// Stores the sign of the value as a Sign.
        pub sign: Sign,
    }

    /// Represents the sign of a LargeInt.
    #[derive(PartialEq, Debug, Copy, Clone)]
    pub enum Sign {
        Positive,
        Negative,
        Unsigned,
    }

    /// Used in the output of the compare method to distinguish between comparisons of LargeInts.
    #[derive(PartialEq, Debug)]
    pub enum Compare {
        Larger,
        Equal,
        Smaller,
    }

    impl LargeInt {

        /// Returns a copy of the scalar LargeInt value as a String.
        pub fn get_int(&self) -> String {
            let mut int =  String::new();
            int.push_str(&self.digits);
            int
        }

        /// Returns the Sign of the LargeInt value as a String.
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

        /// Compares the scalar value of two LargeInts.
        ///
        /// # Examples
        ///
        /// ```
        /// extern crate largeint;
        /// use largeint::largeint::*;
        ///
        /// let largeint1 = new(String::from("150"), Sign::Positive);
        /// let largeint2 = new(String::from("100"), Sign::Positive);
        ///
        /// assert_eq!(largeint1.compare(&largeint2), Compare::Larger);
        /// ```

        pub fn compare(&self, large2: &LargeInt) -> Compare {
            let mut flag = Compare::Larger;
            let large1 = to_vec(&self.digits);
            let large2 = to_vec(&large2.digits);

            if large1.len() > large2.len() {
                flag = Compare::Larger;
            } else if large1.len() < large2.len() {
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

        /// Adds two LargeInts together.
        ///
        /// # Examples
        ///
        /// ```
        /// extern crate largeint;
        /// use largeint::largeint::*;
        ///
        /// let largeint1 = new(String::from("80000000000000000000000000000"), Sign::Positive);
        /// let largeint2 = new(String::from("10000000000000000000000000000"), Sign::Positive);
        /// let sum = largeint1.add(&largeint2);
        /// let check = new(String::from("90000000000000000000000000000"), Sign::Positive);
        /// assert_eq!(sum, check);
        /// ```

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

        /// Subtracts a LargeInt from another.
        ///
        /// # Examples
        ///
        /// ```
        /// extern crate largeint;
        /// use largeint::largeint::*;
        ///
        /// let largeint1 = new(String::from("80000000000000000000000000000"), Sign::Positive);
        /// let largeint2 = new(String::from("10000000000000000000000000000"), Sign::Positive);
        /// let sum = largeint1.sub(&largeint2);
        /// let check = new(String::from("70000000000000000000000000000"), Sign::Positive);
        /// assert_eq!(sum, check);
        /// ```
        pub fn sub(&self, large2: &LargeInt) -> LargeInt{
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

        /// Multiplies two LargeInts together.
        ///
        /// # Examples
        ///
        /// ```
        /// extern crate largeint;
        /// use largeint::largeint::*;
        ///
        /// let largeint1 = new(String::from("1110987654321"), Sign::Positive);
        /// let largeint2 = new(String::from("1234567891011"), Sign::Negative);
        /// let sum = largeint1.mul(&largeint2);
        /// let check = new(String::from("1371589685334334871208531"), Sign::Negative);
        /// assert_eq!(sum, check);
        /// ```

        pub fn mul(&self, large2: &LargeInt) -> LargeInt{
            let digits: String;
            let sign: Sign;
            if &self.sign == &Sign::Positive && large2.sign == Sign::Positive {
                digits = multiplication_2(&self.digits, &large2.digits);
                sign = Sign::Positive;
            } else if &self.sign == &Sign::Negative && large2.sign == Sign::Negative {
                digits = multiplication_2(&self.digits, &large2.digits);
                sign = Sign::Positive;
            } else if &self.sign == &Sign::Unsigned || large2.sign == Sign::Unsigned {
                digits = String::from("0");
                sign = Sign::Unsigned;
            } else if &self.sign == &Sign::Negative || large2.sign == Sign::Negative {
                digits = multiplication_2(&self.digits, &large2.digits);
                sign = Sign::Negative;
            } else {
                panic!("Something went wrong while multiplying!")
            }
            LargeInt {
                digits,
                sign,
            }
        }

        /// Divides one LargeInt by another LargeInt.
        ///
        /// # Examples
        ///
        /// ```
        /// extern crate largeint;
        /// use largeint::largeint::*;
        ///
        /// let largeint1 = new(String::from("3987382479238749823798759823745983748927"), Sign::Negative);
        /// let largeint2 = new(String::from("98394783947982377583783759928"), Sign::Negative);
        /// let sum = largeint1.div(&largeint2);
        /// let check = new(String::from("40524327807"), Sign::Positive);
        /// assert_eq!(sum, check);
        /// ```

        pub fn div(&self, large2: &LargeInt) -> LargeInt {
            let digits_vec: Vec<String>;
            let sign: Sign;
            if large2.sign == Sign::Unsigned {
                panic!("Cannot divide by 0!")
            }
            else if &self.sign == &Sign::Positive && large2.sign == Sign::Positive {
                sign = Sign::Positive;
            } else if &self.sign == &Sign::Unsigned {

                sign = Sign::Unsigned;
            } else if &self.sign == &Sign::Negative && large2.sign == Sign::Negative {
                sign = Sign::Positive;
            } else if  &self.sign == &Sign::Negative || large2.sign == Sign::Negative {
                sign = Sign::Negative;
            }  else {
                panic!("This should never run! Please report this issue at:
                 https://github.com/Starfunk/largeint/issues");
            }
            if &self.compare(large2) == &Compare::Larger {
                digits_vec = division(&self.digits, &large2.digits);
            } else if &self.compare(large2) == &Compare::Smaller {
                digits_vec = vec![String::from("0")];
            } else if &self.sign == &Sign::Unsigned {
                println!("THIS IS RUNNING MAX");
                digits_vec = vec![String::from("0")];
            } else {
                digits_vec = vec![String::from("1")];
            }
            let digits_vec0 = &digits_vec[0];
            let digits = digits_vec0.to_string();
            LargeInt {
                digits,
                sign,
            }
        }

        /// Returns the remainder of a floor division operation.
        ///
        /// # Examples
        ///
        /// ```
        /// extern crate largeint;
        /// use largeint::largeint::*;
        ///
        /// let largeint1 = new(String::from("3987382479238749823798759823745983748927"), Sign::Negative);
        /// let largeint2 = new(String::from("98394783947982377583783759928"), Sign::Negative);
        /// let sum = largeint1.rem(&largeint2);
        /// let check = new(String::from("31770318334258129020721031031"), Sign::Positive);
        /// assert_eq!(sum, check);
        /// ```

        pub fn rem(&self, large2: &LargeInt) -> LargeInt {
            let digits_vec: Vec<String>;
            let digits_vec1: &str;
            let mut sign: Sign;
            if large2.sign == Sign::Unsigned {
                panic!("Cannot divide by 0!")
            } else if &self.sign == &Sign::Positive && large2.sign == Sign::Positive {
                sign = Sign::Positive;
            } else if &self.sign == &Sign::Unsigned {
                sign = Sign::Unsigned;
            } else if &self.sign == &Sign::Negative && large2.sign == Sign::Negative {
                sign = Sign::Positive;
            } else if  &self.sign == &Sign::Negative || large2.sign == Sign::Negative {
                sign = Sign::Negative;
            }  else {
                panic!("This should never run! Please report this issue at:
                 https://github.com/Starfunk/largeint/issues");
            }
            if &self.digits == "0" {
                digits_vec1 = "0";
            }
            else if &self.compare(large2) == &Compare::Larger {
                digits_vec = division(&self.digits, &large2.digits);
                digits_vec1 = &digits_vec[1];
                if digits_vec1 == "0" {
                    sign = Sign::Unsigned;
                }
            } else if &self.compare(large2) == &Compare::Smaller {
                digits_vec1 = &large2.digits;
            } else {
                digits_vec1 = "0";
            }
            let digits = digits_vec1.to_string();
            LargeInt {
                digits,
                sign,
            }
        }
    }

    /// Creates a new instance of LargeInt. It is strongly recommended that
    /// LargeInt creation is done through the new() function which will instantiate an instance of
    /// LargeInt safely, making the necessary corrections to the fields if necessary, or panicking
    /// if an invalid input was submitted. See above for examples of LargeInt creation.

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

    #[doc(hidden)]
    pub fn to_vec(input_str: &str) -> Vec<u8> {
        let mut vec: Vec<u8> = Vec::new();
        for i in input_str.chars() {
            let vec_input = i.to_digit(10).unwrap() as u8;
            vec.push(vec_input);
        }
        vec
    }

    #[doc(hidden)]
    pub fn vec_to_str(vec: &Vec<u8>) -> String {
        let mut output = String::from("");
        for i in vec {
            output.push_str(&i.to_string());
        }
        output
    }
}
