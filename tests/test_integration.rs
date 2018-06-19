
extern crate largeint;

use largeint::largeint::*;

#[test]
fn positive_positive_add_same_length() {
    let largeint1 = new(String::from("83901489213409093401849249010492000112"), Sign::Positive);
    let largeint2 = new(String::from("32948394029020109340294890318493100090"), Sign::Positive);
    let largeint3 = largeint1.add(&largeint2);
    let largeint4 = new(String::from("116849883242429202742144139328985100202"), Sign::Positive);
    assert_eq!(largeint3, largeint4)
}


#[test]
fn positive_positive_add_not_same_length() {
    let largeint1 = new(String::from("33901489213409093401849249010492000112"), Sign::Positive);
    let largeint2 = new(String::from("8294839402902010934029489031849310009324234230"), Sign::Positive);
    let largeint3 = largeint1.add(&largeint2);
    let largeint4 = new(String::from("8294839436803500147438582433698559019816234342"), Sign::Positive);
    assert_eq!(largeint3,largeint4)
}

#[test]
fn positive_negative_add_same_length() {
    let largeint1 = new(String::from("33901489213409093401849249010492000112"), Sign::Positive);
    let largeint2 = new(String::from("82948394029020109340294890318493100090"), Sign::Negative);
    let largeint3 = largeint1.add(&largeint2);
    let largeint4 = new(String::from("49046904815611015938445641308001099978"), Sign::Negative);
    assert_eq!(largeint3,largeint4)
}

#[test]
fn positive_negative_add_not_same_length() {
    let largeint1 = new(String::from("33901489213409093401849249010492000112"), Sign::Positive);
    let largeint2 = new(String::from("8294839402902010934029489031849310009324234230"), Sign::Negative);
    let largeint3 = largeint1.add(&largeint2);
    let largeint4 = new(String::from("8294839369000521720620395630000060998832234118"), Sign::Negative);
    assert_eq!(largeint3,largeint4)
}

#[test]
fn negative_negative_add_same_length() {
    let largeint1 = new(String::from("33901489213409093401849249010492000112"), Sign::Negative);
    let largeint2 = new(String::from("84098320948203948239489384098590385993"), Sign::Negative);
    let largeint3 = largeint1.add(&largeint2);
    let largeint4 = new(String::from("117999810161613041641338633109082386105"), Sign::Negative);
    assert_eq!(largeint3,largeint4)
}

#[test]
fn negative_negative_add_not_same_length() {
    let largeint1 = new(String::from("33901489213409093401849249010492000112"), Sign::Negative);
    let largeint2 = new(String::from("1008294839402902010934029489031849310009324234230"), Sign::Negative);
    let largeint3 = largeint1.add(&largeint2);
    let largeint4 = new(String::from("1008294839436803500147438582433698559019816234342"), Sign::Negative);
    assert_eq!(largeint3,largeint4)
}

#[test]
fn positive_unsigned_add() {
    let largeint1 = new(String::from("33901489213409093401849249010492000112"), Sign::Positive);
    let largeint2 = new(String::from("0"), Sign::Unsigned);
    let largeint3 = largeint1.add(&largeint2);
    let largeint4 = new(String::from("33901489213409093401849249010492000112"), Sign::Positive);
    assert_eq!(largeint3,largeint4)
}

#[test]
fn negative_unsigned_add() {
    let largeint1 = new(String::from("33901489213409093401849249010492000112"), Sign::Negative);
    let largeint2 = new(String::from("0"), Sign::Unsigned);
    let largeint3 = largeint1.add(&largeint2);
    let largeint4 = new(String::from("33901489213409093401849249010492000112"), Sign::Negative);
    assert_eq!(largeint3,largeint4)
}

#[test]
fn positive_negative_add_equal() {
    let largeint1 = new(String::from("33901489213409093401849249010492000112"), Sign::Positive);
    let largeint2 = new(String::from("33901489213409093401849249010492000112"), Sign::Negative);
    let largeint3 = largeint1.add(&largeint2);
    let largeint4 = new(String::from("0"), Sign::Unsigned);
    assert_eq!(largeint3,largeint4)
}

#[test]
fn unsigned_unsigned_add() {
    let largeint1 = new(String::from("0"), Sign::Unsigned);
    let largeint2 = new(String::from("0"), Sign::Unsigned);
    let largeint3 = largeint1.add(&largeint2);
    let largeint4 = new(String::from("0"), Sign::Unsigned);
    assert_eq!(largeint3,largeint4)
}


#[test]
fn positive_positive_subtract_same_length() {
    let largeint1 = new(String::from("33901489213409093401849249010492000112"), Sign::Positive);
    let largeint2 = new(String::from("94280329423048093284093240234809833999"), Sign::Positive);
    let largeint3 = largeint1.subtract(&largeint2);
    let largeint4 = new(String::from("60378840209638999882243991224317833887"), Sign::Negative);
    assert_eq!(largeint3,largeint4)
}

#[test]
fn positive_positive_subtract_same_not_length() {
    let largeint1 = new(String::from("33901489213409093401849249010492000112"), Sign::Positive);
    let largeint2 = new(String::from("100320394280329423048093284093240234809833999"), Sign::Positive);
    let largeint3 = largeint1.subtract(&largeint2);
    let largeint4 = new(String::from("100320360378840209638999882243991224317833887"), Sign::Negative);
    assert_eq!(largeint3,largeint4)
}

#[test]
fn negative_positive_subtract_same_length() {
    let largeint1 = new(String::from("33901489213409093401849249010492000112"), Sign::Negative);
    let largeint2 = new(String::from("94280329423048093284093240234809833999"), Sign::Positive);
    let largeint3 = largeint1.subtract(&largeint2);
    let largeint4 = new(String::from("128181818636457186685942489245301834111"), Sign::Negative);
    assert_eq!(largeint3,largeint4)
}

#[test]
fn negative_positive_subtract_not_same_length() {
    let largeint1 = new(String::from("33901489213409093401849249010492000112"), Sign::Negative);
    let largeint2 = new(String::from("6432847772394280329423048093284093240234809833999"), Sign::Positive);
    let largeint3 = largeint1.subtract(&largeint2);
    let largeint4 = new(String::from("6432847772428181818636457186685942489245301834111"), Sign::Negative);
    assert_eq!(largeint3,largeint4)
}

#[test]
fn positive_negative_subtract_same_length() {
    let largeint1 = new(String::from("33901489213409093401849249010492000112"), Sign::Positive);
    let largeint2 = new(String::from("94280329423048093284093240234809833999"), Sign::Negative);
    let largeint3 = largeint1.subtract(&largeint2);
    let largeint4 = new(String::from("128181818636457186685942489245301834111"), Sign::Positive);
    assert_eq!(largeint3,largeint4)
}

#[test]
fn positive_negative_subtract_not_same_length() {
    let largeint1 = new(String::from("33901489213409093401849249010492000112"), Sign::Positive);
    let largeint2 = new(String::from("6432847772394280329423048093284093240234809833999"), Sign::Negative);
    let largeint3 = largeint1.subtract(&largeint2);
    let largeint4 = new(String::from("6432847772428181818636457186685942489245301834111"), Sign::Positive);
    assert_eq!(largeint3,largeint4)
}

#[test]
fn negative_negative_subtract_same_length() {
    let largeint1 = new(String::from("33901489213409093401849249010492000112"), Sign::Negative);
    let largeint2 = new(String::from("94280329423048093284093240234809833999"), Sign::Negative);
    let largeint3 = largeint1.subtract(&largeint2);
    let largeint4 = new(String::from("60378840209638999882243991224317833887"), Sign::Positive);
    assert_eq!(largeint3,largeint4)
}

#[test]
fn negative_negative_subtract_same_not_length() {
    let largeint1 = new(String::from("33901489213409093401849249010492000112"), Sign::Negative);
    let largeint2 = new(String::from("100320394280329423048093284093240234809833999"), Sign::Negative);
    let largeint3 = largeint1.subtract(&largeint2);
    let largeint4 = new(String::from("100320360378840209638999882243991224317833887"), Sign::Positive);
    assert_eq!(largeint3,largeint4)
}

#[test]
fn positive_unsigned_subtract() {
    let largeint1 = new(String::from("33901"), Sign::Positive);
    let largeint2 = new(String::from("0"), Sign::Unsigned);
    let largeint3 = largeint1.subtract(&largeint2);
    let largeint4 = new(String::from("33901"), Sign::Positive);
    assert_eq!(largeint3,largeint4)
}

#[test]
fn negative_unsigned_subtract() {
    let largeint1 = new(String::from("33901"), Sign::Negative);
    let largeint2 = new(String::from("0"), Sign::Unsigned);
    let largeint3 = largeint1.subtract(&largeint2);
    let largeint4 = new(String::from("33901"), Sign::Negative);
    assert_eq!(largeint3,largeint4)
}

#[test]
fn unsigned_positive_subtract() {
    let largeint1 = new(String::from("0"), Sign::Unsigned);
    let largeint2 = new(String::from("33901"), Sign::Positive);
    let largeint3 = largeint1.subtract(&largeint2);
    let largeint4 = new(String::from("33901"), Sign::Negative);
    assert_eq!(largeint3,largeint4)
}

#[test]
fn unsigned_negative_subtract() {
    let largeint1 = new(String::from("0"), Sign::Unsigned);
    let largeint2 = new(String::from("33901"), Sign::Negative);
    let largeint3 = largeint1.subtract(&largeint2);
    let largeint4 = new(String::from("33901"), Sign::Positive);
    assert_eq!(largeint3,largeint4)
}

#[test]
fn unsigned_unsigned_subtract() {
    let largeint1 = new(String::from("0"), Sign::Unsigned);
    let largeint2 = new(String::from("0"), Sign::Unsigned);
    let largeint3 = largeint1.subtract(&largeint2);
    let largeint4 = new(String::from("0"), Sign::Unsigned);
    assert_eq!(largeint3,largeint4)
}
