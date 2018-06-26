
extern crate largeint;

use largeint::largeint::*;

#[cfg(test)]
mod tests {
    use super::*;

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
    fn positive_positive_sub_same_length() {
        let largeint1 = new(String::from("33901489213409093401849249010492000112"), Sign::Positive);
        let largeint2 = new(String::from("94280329423048093284093240234809833999"), Sign::Positive);
        let largeint3 = largeint1.sub(&largeint2);
        let largeint4 = new(String::from("60378840209638999882243991224317833887"), Sign::Negative);
        assert_eq!(largeint3,largeint4)
    }

    #[test]
    fn positive_positive_sub_same_not_length() {
        let largeint1 = new(String::from("33901489213409093401849249010492000112"), Sign::Positive);
        let largeint2 = new(String::from("100320394280329423048093284093240234809833999"), Sign::Positive);
        let largeint3 = largeint1.sub(&largeint2);
        let largeint4 = new(String::from("100320360378840209638999882243991224317833887"), Sign::Negative);
        assert_eq!(largeint3,largeint4)
    }

    #[test]
    fn negative_positive_sub_same_length() {
        let largeint1 = new(String::from("33901489213409093401849249010492000112"), Sign::Negative);
        let largeint2 = new(String::from("94280329423048093284093240234809833999"), Sign::Positive);
        let largeint3 = largeint1.sub(&largeint2);
        let largeint4 = new(String::from("128181818636457186685942489245301834111"), Sign::Negative);
        assert_eq!(largeint3,largeint4)
    }

    #[test]
    fn negative_positive_sub_not_same_length() {
        let largeint1 = new(String::from("33901489213409093401849249010492000112"), Sign::Negative);
        let largeint2 = new(String::from("6432847772394280329423048093284093240234809833999"), Sign::Positive);
        let largeint3 = largeint1.sub(&largeint2);
        let largeint4 = new(String::from("6432847772428181818636457186685942489245301834111"), Sign::Negative);
        assert_eq!(largeint3,largeint4)
    }

    #[test]
    fn positive_negative_sub_same_length() {
        let largeint1 = new(String::from("33901489213409093401849249010492000112"), Sign::Positive);
        let largeint2 = new(String::from("94280329423048093284093240234809833999"), Sign::Negative);
        let largeint3 = largeint1.sub(&largeint2);
        let largeint4 = new(String::from("128181818636457186685942489245301834111"), Sign::Positive);
        assert_eq!(largeint3,largeint4)
    }

    #[test]
    fn positive_negative_sub_not_same_length() {
        let largeint1 = new(String::from("33901489213409093401849249010492000112"), Sign::Positive);
        let largeint2 = new(String::from("6432847772394280329423048093284093240234809833999"), Sign::Negative);
        let largeint3 = largeint1.sub(&largeint2);
        let largeint4 = new(String::from("6432847772428181818636457186685942489245301834111"), Sign::Positive);
        assert_eq!(largeint3,largeint4)
    }

    #[test]
    fn negative_negative_sub_same_length() {
        let largeint1 = new(String::from("33901489213409093401849249010492000112"), Sign::Negative);
        let largeint2 = new(String::from("94280329423048093284093240234809833999"), Sign::Negative);
        let largeint3 = largeint1.sub(&largeint2);
        let largeint4 = new(String::from("60378840209638999882243991224317833887"), Sign::Positive);
        assert_eq!(largeint3,largeint4)
    }

    #[test]
    fn negative_negative_sub_same_not_length() {
        let largeint1 = new(String::from("33901489213409093401849249010492000112"), Sign::Negative);
        let largeint2 = new(String::from("100320394280329423048093284093240234809833999"), Sign::Negative);
        let largeint3 = largeint1.sub(&largeint2);
        let largeint4 = new(String::from("100320360378840209638999882243991224317833887"), Sign::Positive);
        assert_eq!(largeint3,largeint4)
    }

    #[test]
    fn positive_unsigned_sub() {
        let largeint1 = new(String::from("33901"), Sign::Positive);
        let largeint2 = new(String::from("0"), Sign::Unsigned);
        let largeint3 = largeint1.sub(&largeint2);
        let largeint4 = new(String::from("33901"), Sign::Positive);
        assert_eq!(largeint3,largeint4)
    }

    #[test]
    fn negative_unsigned_sub() {
        let largeint1 = new(String::from("33901"), Sign::Negative);
        let largeint2 = new(String::from("0"), Sign::Unsigned);
        let largeint3 = largeint1.sub(&largeint2);
        let largeint4 = new(String::from("33901"), Sign::Negative);
        assert_eq!(largeint3,largeint4)
    }

    #[test]
    fn unsigned_positive_sub() {
        let largeint1 = new(String::from("0"), Sign::Unsigned);
        let largeint2 = new(String::from("33901"), Sign::Positive);
        let largeint3 = largeint1.sub(&largeint2);
        let largeint4 = new(String::from("33901"), Sign::Negative);
        assert_eq!(largeint3,largeint4)
    }

    #[test]
    fn unsigned_negative_sub() {
        let largeint1 = new(String::from("0"), Sign::Unsigned);
        let largeint2 = new(String::from("33901"), Sign::Negative);
        let largeint3 = largeint1.sub(&largeint2);
        let largeint4 = new(String::from("33901"), Sign::Positive);
        assert_eq!(largeint3,largeint4)
    }

    #[test]
    fn unsigned_unsigned_sub() {
        let largeint1 = new(String::from("0"), Sign::Unsigned);
        let largeint2 = new(String::from("0"), Sign::Unsigned);
        let largeint3 = largeint1.sub(&largeint2);
        let largeint4 = new(String::from("0"), Sign::Unsigned);
        assert_eq!(largeint3,largeint4)
    }

    #[test]
    fn positive_positive_mul_length_1() {
        let largeint1 = new(String::from("32984732984702"), Sign::Positive);
        let largeint2 = new(String::from("9"), Sign::Positive);
        let largeint3 = largeint1.mul(&largeint2);
        let largeint4 = new(String::from("296862596862318"), Sign::Positive);
        assert_eq!(largeint3,largeint4)
    }

    #[test]
    fn positive_positive_mul() {
        let largeint1 = new(String::from("9384932"), Sign::Positive);
        let largeint2 = new(String::from("49332493"), Sign::Positive);
        let largeint3 = largeint1.mul(&largeint2);
        let largeint4 = new(String::from("462982092195476"), Sign::Positive);
        assert_eq!(largeint3,largeint4)
    }

    #[test]
    fn positive_negative_mul() {
        let largeint1 = new(String::from("9384932"), Sign::Positive);
        let largeint2 = new(String::from("49332493"), Sign::Negative);
        let largeint3 = largeint1.mul(&largeint2);
        let largeint4 = new(String::from("462982092195476"), Sign::Negative);
        assert_eq!(largeint3,largeint4)
    }

    #[test]
    fn unsigned_positive_mul() {
        let largeint1 = new(String::from("0"), Sign::Unsigned);
        let largeint2 = new(String::from("349032480934"), Sign::Positive);
        let largeint3 = largeint1.mul(&largeint2);
        let largeint4 = new(String::from("0"), Sign::Unsigned);
        assert_eq!(largeint3,largeint4)
    }

    #[test]
    fn unsigned_unsigned_mul() {
        let largeint1 = new(String::from("0"), Sign::Positive);
        let largeint2 = new(String::from("0"), Sign::Negative);
        let largeint3 = largeint1.mul(&largeint2);
        let largeint4 = new(String::from("0"), Sign::Unsigned);
        assert_eq!(largeint3,largeint4)
    }

    #[test]
    fn negative_negative_mul() {
        let largeint1 = new(String::from("43249"), Sign::Negative);
        let largeint2 = new(String::from("9999900988"), Sign::Negative);
        let largeint3 = largeint1.mul(&largeint2);
        let largeint4 = new(String::from("432485717830012"), Sign::Positive);
        assert_eq!(largeint3,largeint4)
    }

    #[test]
    fn positive_positive_div_1() {
        let largeint1 = new(String::from("911111111111111111111"), Sign::Positive);
        let largeint2 = new(String::from("3123456789"), Sign::Positive);
        let largeint3 = largeint1.div(&largeint2);
        let largeint4 = new(String::from("291699604848"), Sign::Positive);
        assert_eq!(largeint3,largeint4)
    }

    #[test]
    fn positive_positive_rem_1() {
        let largeint1 = new(String::from("911111111111111111111"), Sign::Positive);
        let largeint2 = new(String::from("3123456789"), Sign::Positive);
        let largeint3 = largeint1.rem(&largeint2);
        let largeint4 = new(String::from("8198039"), Sign::Positive);
        assert_eq!(largeint3,largeint4)
    }

    #[test]
    fn positive_positive_div_2() {
        let largeint1 = new(String::from("59871"), Sign::Positive);
        let largeint2 = new(String::from("26"), Sign::Positive);
        let largeint3 = largeint1.div(&largeint2);
        let largeint4 = new(String::from("2302"), Sign::Positive);
        assert_eq!(largeint3,largeint4)
    }

    #[test]
    fn positive_positive_rem_2() {
        let largeint1 = new(String::from("59871"), Sign::Positive);
        let largeint2 = new(String::from("26"), Sign::Positive);
        let largeint3 = largeint1.rem(&largeint2);
        let largeint4 = new(String::from("19"), Sign::Positive);
        assert_eq!(largeint3,largeint4)
    }

    #[test]
    fn positive_positive_div_3() {
        let largeint1 = new(String::from("10010"), Sign::Positive);
        let largeint2 = new(String::from("10"), Sign::Positive);
        let largeint3 = largeint1.div(&largeint2);
        let largeint4 = new(String::from("1001"), Sign::Positive);
        assert_eq!(largeint3,largeint4)
    }

    #[test]
    fn positive_positive_rem_3() {
        let largeint1 = new(String::from("10010"), Sign::Positive);
        let largeint2 = new(String::from("10"), Sign::Positive);
        let largeint3 = largeint1.rem(&largeint2);
        let largeint4 = new(String::from("0"), Sign::Unsigned);
        assert_eq!(largeint3,largeint4)
    }

    #[test]
    fn positive_negative_div() {
        let largeint1 = new(String::from("13925893850923850983290850932850932859"), Sign::Positive);
        let largeint2 = new(String::from("999999999991140979439141320984"), Sign::Negative);
        let largeint3 = largeint1.div(&largeint2);
        let largeint4 = new(String::from("13925893"), Sign::Negative);
        assert_eq!(largeint3,largeint4)
    }

    #[test]
    fn positive_negative_rem() {
        let largeint1 = new(String::from("13925893850923850983290850932850932859"), Sign::Positive);
        let largeint2 = new(String::from("999999999991140979439141320984"), Sign::Negative);
        let largeint3 = largeint1.rem(&largeint2);
        let largeint4 = new(String::from("851047220755706168884949094147"), Sign::Negative);
        assert_eq!(largeint3,largeint4)
    }

    #[test]
    fn negative_positive_div() {
        let largeint1 = new(String::from("3849934893230948093840938439840989348238394893842093480"), Sign::Negative);
        let largeint2 = new(String::from("689993948093849340329048903284839483928403284"), Sign::Positive);
        let largeint3 = largeint1.div(&largeint2);
        let largeint4 = new(String::from("5579664725"), Sign::Negative);
        assert_eq!(largeint3,largeint4)
    }

    #[test]
    fn negative_positive_rem() {
        let largeint1 = new(String::from("2227"), Sign::Negative);
        let largeint2 = new(String::from("10"), Sign::Positive);
        let largeint3 = largeint1.rem(&largeint2);
        let largeint4 = new(String::from("7"), Sign::Negative);
        assert_eq!(largeint3,largeint4)
    }

    #[test]
    fn negative_negative_div() {
        let largeint1 = new(String::from("2227"), Sign::Negative);
        let largeint2 = new(String::from("10"), Sign::Positive);
        let largeint3 = largeint1.div(&largeint2);
        let largeint4 = new(String::from("222"), Sign::Negative);
        assert_eq!(largeint3,largeint4)
    }

    #[test]
    fn negative_negative_rem() {
        let largeint1 = new(String::from("2227"), Sign::Negative);
        let largeint2 = new(String::from("10"), Sign::Negative);
        let largeint3 = largeint1.rem(&largeint2);
        let largeint4 = new(String::from("7"), Sign::Positive);
        assert_eq!(largeint3,largeint4)
    }

    #[test]
    fn unsigned_negative_div() {
        let largeint1 = new(String::from("0"), Sign::Negative);
        let largeint2 = new(String::from("103092830933333"), Sign::Negative);
        let largeint3 = largeint1.div(&largeint2);
        let largeint4 = new(String::from("0"), Sign::Unsigned);
        assert_eq!(largeint3,largeint4)
    }

    #[test]
    fn unsigned_negative_rem() {
        let largeint1 = new(String::from("0"), Sign::Negative);
        let largeint2 = new(String::from("103092830933333"), Sign::Negative);
        let largeint3 = largeint1.rem(&largeint2);
        let largeint4 = new(String::from("0"), Sign::Unsigned);
        assert_eq!(largeint3,largeint4)
    }

    #[test]
    #[should_panic(expected = "Cannot divide by 0!")]
    fn positive_unsigned_div() {
        let largeint1 = new(String::from("2227"), Sign::Positive);
        let largeint2 = new(String::from("0"), Sign::Positive);
        let _largeint3 = largeint1.div(&largeint2);
        // let largeint4 = new(String::from("7"), Sign::Negative);
        // assert_eq!(largeint3,largeint4)
    }

    #[test]
    #[should_panic(expected = "Cannot divide by 0!")]
    fn positive_unsigned_rem() {
        let largeint1 = new(String::from("2227"), Sign::Positive);
        let largeint2 = new(String::from("0"), Sign::Positive);
        let largeint3 = largeint1.rem(&largeint2);
        let largeint4 = new(String::from("7"), Sign::Negative);
        assert_eq!(largeint3,largeint4)
    }

    #[test]
    #[should_panic(expected = "Cannot divide by 0!")]
    fn unsigned_unsigned_div() {
        let largeint1 = new(String::from("0"), Sign::Negative);
        let largeint2 = new(String::from("0"), Sign::Positive);
        let largeint3 = largeint1.div(&largeint2);
        let largeint4 = new(String::from("7"), Sign::Negative);
        assert_eq!(largeint3,largeint4)
    }

    #[test]
    #[should_panic(expected = "Cannot divide by 0!")]
    fn unsigned_unsigned_rem() {
        let largeint1 = new(String::from("0"), Sign::Negative);
        let largeint2 = new(String::from("0"), Sign::Positive);
        let largeint3 = largeint1.rem(&largeint2);
        let largeint4 = new(String::from("7"), Sign::Negative);
        assert_eq!(largeint3,largeint4)
    }
}
