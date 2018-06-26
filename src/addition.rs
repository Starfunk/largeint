
use largeint::*;

pub fn addition(large1: &str, large2: &str) -> String {
    let large1 = to_vec(&large1);
    let large2 = to_vec(&large2);
    let large3: &Vec<u8>;
    let large4: &Vec<u8>;
    let mut sum = vec![];
    let mut counter: u8 = 0;
    let mut input: u8;

    if large1.len() >= large2.len() {
        large3 = &large1;
        large4 = &large2;
    } else {
        large3 = &large2;
        large4 = &large1;
    }
    let len_1 = large3.len();
    let len_2 = large4.len();
        for i in 0..len_2 {
            input = large3[len_1 - i - 1] + large4[len_2 - i - 1] + counter;
            if input >= 10 {
                input = input - 10;
                sum.insert(0, input);
                counter = 1;
            } else {
                sum.insert(0, input);
                counter = 0;
            }
        }
        if len_1 > len_2 {
            for i in (0..len_1 - len_2).rev() {
                input = large3[i] + counter;

                if input == 10 {
                    sum.insert(0, 0);
                    counter = 1;
                } else {
                    sum.insert(0, input);
                    counter = 0;
                }
            }
            if counter == 1 {
                sum.insert(0, 1)
            }
        }
        if len_1 == len_2 && counter == 1 {
            sum.insert(0, 1);
        }
    let digits = vec_to_str(&sum);
    digits
}


pub fn addition_2(large1: Vec<u8>, large2: Vec<u8>) -> Vec<u8> {
    let large3: &Vec<u8>;
    let large4: &Vec<u8>;
    let mut sum = vec![];
    let mut counter: u8 = 0;
    let mut input: u8;

    if large1.len() >= large2.len() {
        large3 = &large1;
        large4 = &large2;
    } else {
        large3 = &large2;
        large4 = &large1;
    }
    let len_1 = large3.len();
    let len_2 = large4.len();
        for i in 0..len_2 {
            input = large3[len_1 - i - 1] + large4[len_2 - i - 1] + counter;
            if input >= 10 {
                input = input - 10;
                sum.insert(0, input);
                counter = 1;
            } else {
                sum.insert(0, input);
                counter = 0;
            }
        }
        if len_1 > len_2 {
            for i in (0..len_1 - len_2).rev() {
                input = large3[i] + counter;

                if input == 10 {
                    sum.insert(0, 0);
                    counter = 1;
                } else {
                    sum.insert(0, input);
                    counter = 0;
                }
            }
            if counter == 1 {
                sum.insert(0, 1)
            }
        }
        if len_1 == len_2 && counter == 1 {
            sum.insert(0, 1);
        }
    sum
}
