# largeint
A library that implements the `LargeInt` type, used for working with arbitrarily large integers in Rust!

## Getting Started
First, add largeint to your dependencies:
```toml
[dependencies]
largeint = "0.1.0"
```
Next, add this to the root of your crate to bring the contents of largeint into the scope of your project:
```rust
extern crate largeint;

use largeint::largeint::*;
```


You can then easily create many instances of `LargeInt`:
```rust
let largeint1 = new(String::from("999999999999999999999"), Sign::Positive);
let largeint2 = new(String::from("999999999999999999999"), Sign::Negative);
let largeint3 = new(String::from("0"), Sign::Unsigned);
```
An instance of `LargeInt` contains two fields, the scalar value of the integer stored as a `String` and the sign of the integer stored as the enum, `Sign`, which can be `Positive`, `Negative`, or `Unsigned` (note that `0` is the only integer that should be assigned `Unsigned`). 

Using `new()` to create an instance of `LargeInt` is highly recommmended as there are checks in place to ensure that the `LargeInt` will be created properly.

Refer to the documentation for more details.

## An Example

```rust
extern crate largeint;

use largeint::largeint::*;

fn main() {

    // Adding two LargeInts.
    let largeint1 = new(String::from("33901489213409093401849249010492000112"), Sign::Positive);
    let largeint2 = new(String::from("8294839402902010934029489031849310009324234230"), Sign::Negative);
    let largeint3 = largeint1.add(&largeint2);
    let largeint4 = new(String::from("8294839369000521720620395630000060998832234118"), Sign::Negative);
    assert_eq!(largeint3,largeint4);

    // Subtracting two LargeInts.
    let largeint1 = new(String::from("33901489213409093401849249010492000112"), Sign::Negative);
    let largeint2 = new(String::from("100320394280329423048093284093240234809833999"), Sign::Negative);
    let largeint3 = largeint1.subtract(&largeint2);
    let largeint4 = new(String::from("100320360378840209638999882243991224317833887"), Sign::Positive);
    assert_eq!(largeint3,largeint4);
    
    //The get_int() method returns the scalar value of the LargeInt as a String.
    println!("The value of largeint1 is: {}", largeint1.get_int());
    
    //The get_sign() method returns the Sign of the LargeInt as a String.
     println!("The Sign of largeint1 is: {}", largeint1.get_sign());
}

```

## License
This project is licensed under the MIT License - see [LICENSE.md](https://github.com/Starfunk/largeint/blob/master/LICENSE) for more details.

## Future Updates
Integer multiplication and division for `LargeInt` is currently being developed and should be released sometime in the next few weeks in the `"0.2.0"` update.
