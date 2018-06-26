# largeint
A library that supports large integer arithmetic.

## Getting Started
First, add largeint to your dependencies:
```toml
[dependencies]
largeint = "0.2.0"
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

Using `new()` to create an instance of `LargeInt` is highly recommended as there are checks in place to ensure that the the instance of `LargeInt` will be created properly. For example, creating an instance of a `LargeInt` with a scalar value of `0` using `new` will automatically assign `Sign::Unsigned` to the sign of the LargeInt even if you enter another `Sign` variant.  

The purpose of this library is to provide an easy-to-use large integer implementation in Rust.
The ideal user is one that is looking to write small scale projects for personal use and does
not want to spend time a lot of time learning a complex crate such as num-bigint. For example,
this library would be ideal for one looking to solve [Project Euler Problem 13](https://projecteuler.net/problem=13).
The largeint library is not particularly efficient and therefore it is recommended to use
a crate like num-bigint for more serious projects.

Let's see just how easy it is to start performing large integer arithmetic!

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
    let largeint3 = largeint1.sub(&largeint2);
    let largeint4 = new(String::from("100320360378840209638999882243991224317833887"), Sign::Positive);
    assert_eq!(largeint3,largeint4);

    // Multiplying two LargeInts.
    let largeint1 = new(String::from("239014892134090934018492404920112"), Sign::Negative);
    let largeint2 = new(String::from("820948948039443908494308943885"), Sign::Negative);
    let largeint3 = largeint1.mul(&largeint2);
    let largeint4 = new(String::from("196219024263243108752932957733805138559777844813650340515915120"), Sign::Positive);
    assert_eq!(largeint3,largeint4);

    // Dividing two LargeInts.
    let largeint1 = new(String::from("33901489213409093401849249010492088384894374938712"), Sign::Positive);
    let largeint2 = new(String::from("1003203942803294230480932840934343489333999"), Sign::Negative);
    let largeint3 = largeint1.div(&largeint2);
    let largeint4 = new(String::from("33793217"), Sign::Negative);
    assert_eq!(largeint3,largeint4);

    //The get_int() method returns the scalar value of the LargeInt as a String.
    println!("The value of largeint1 is: {}", largeint1.get_int());

    //The get_sign() method returns the Sign of the LargeInt as a String.
    println!("The Sign of largeint1 is: {}", largeint1.get_sign());
}

```
## Updates
Code Breaking Changes:
The subtraction method has been renamed `sub` from  `subtraction`.

New Library Additions:
Multiplication, floor division - and remainder - have been added.  

## License
This project is licensed under the MIT License - see [LICENSE.md](https://github.com/Starfunk/largeint/blob/master/LICENSE) for more details.

