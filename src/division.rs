use largeint::*;
use multiplication::multiplication_1;

pub fn division(large1: &str, large2: &str) -> Vec<String> {
    let mut divided = String::from("");
    let mut remainder_largeint: LargeInt;
    let mut remainder_str = String::from("");
    let output_vec: Vec<String>;
    let sign = Sign::Positive;
    let large2 = new(String::from(large2), sign);
    let large1_length = large1.len();
    let mut large1_slice: &str;
    let mut flag = true;
    let mut counter = 0;
    while flag == true {
        for i in 1..large1_length + 1 {
            large1_slice = &large1[0..i];
            let large1_piece = new(String::from(large1_slice), sign);
            if large1_piece.compare(&large2) == Compare::Smaller {
                counter += 1;
                continue
            } else if large1_piece.compare(&large2) == Compare::Larger {
                for i in 1..10 {
                    let index = i.to_string();
                    let index_str= to_vec(&index[..]);
                    let large2_vec = to_vec(&large2.digits);
                    let product = multiplication_1(&large2_vec, &index_str);
                    let product_vec = vec_to_str(&product);
                    let product_largeint = new(product_vec, sign);
                    if large1_piece.compare(&product_largeint) == Compare::Smaller {
                        let index1 = i - 1;
                        let index1_str = index1.to_string();
                        divided.push_str(&index1.to_string());
                        let index1_largeint = to_vec(&index1_str);
                        let product_2 = multiplication_1(&large2_vec, &index1_largeint);
                        let product_vec_2 = vec_to_str(&product_2);
                        let product_largeint_2 = new(product_vec_2, sign);
                        remainder_largeint = large1_piece.sub(&product_largeint_2);
                        remainder_str = remainder_largeint.digits;
                        flag = false;
                        break
                    } else if large1_piece.compare(&product_largeint) == Compare::Equal {
                        remainder_str = String::from("0");
                        divided.push_str(&index);
                        flag = false;
                        break
                    }
                }
                counter += 1;
            } else if large1_piece.compare(&large2) == Compare::Equal {
                remainder_str.push_str("0");
                divided.push_str("1");
                counter += 1;
                flag = false;
            }
            break
        }
    }
    if counter != large1_length {
        for _ in counter..large1_length + 1 {
            if counter == large1_length {
                break
            }
            if remainder_str == String::from("0") && &large1[counter..counter+1] == "0" {
                remainder_str = String::from("0");
                divided.push_str("0");
                counter += 1;
            }
            else if remainder_str == String::from("0") && &large1[counter..counter+1] != "0" {
                remainder_str = large1[counter..counter+1].to_string();
                let remainder_largeint = new(remainder_str.clone(), sign);
                if remainder_largeint.compare(&large2) == Compare::Smaller {
                    divided.push_str("0");
                } else if remainder_largeint.compare(&large2) == Compare::Larger {
                    let output_tuple = find_the_highest_divider(&remainder_str, &large2, &mut divided);
                    divided = output_tuple.0;
                    remainder_str = output_tuple.1;
                } else {
                    remainder_str = String::from("0");
                    divided.push_str("0");
                }
                counter += 1;
            } else {
                remainder_str.push_str(&large1[counter..counter+1]);
                let mut remainder_largeint = new(remainder_str.clone(), sign);
                if remainder_largeint.compare(&large2) == Compare::Smaller {
                    divided.push_str("0");
                } else if remainder_largeint.compare(&large2) == Compare::Larger {
                    let output_tuple = find_the_highest_divider(&remainder_str, &large2, &mut divided);
                    divided = output_tuple.0;
                    remainder_str = output_tuple.1;
                } else {
                    remainder_str = String::from("0");
                    divided.push_str("1");
                }
                counter += 1;
            }
        }
    }
    output_vec = vec![divided, remainder_str];
    output_vec
}

fn find_the_highest_divider(remainder: &String, large2: &LargeInt, divided: &mut String) -> (String, String) {
    let mut remainder_str = String::from("0");
    let sign = Sign::Positive;
    let remainder_largeint = new(remainder.clone(), sign);
    let large2_vec = to_vec(&large2.digits);
    let remainder2_largeint: LargeInt;
    let mut index: String;
    let mut index_vec: Vec<u8>;
    let mut product: Vec<u8>;
    for i in 0..10 {
        index = i.to_string();
        index_vec = to_vec(&index[..]);
        if i == 0 {
            product = vec![0]
        } else {
            product = multiplication_1(&large2_vec, &index_vec);
        }
        let product_str = vec_to_str(&product);
        let product_largeint = new(product_str, sign);
        if remainder_largeint.compare(&product_largeint) == Compare::Smaller {
            let index1 = i - 1;
            let index1_vec: Vec<u8>;
            let index1_str: String;
            if index1 == 0 {
                index1_str = String::from("0");
            } else {
                index1_str = index1.to_string();
            }
            index1_vec = to_vec(&index1_str[..]);
            let product1 = multiplication_1(&large2_vec, &index1_vec);
            let product1_str = vec_to_str(&product1 );
            let product1_largeint = new(product1_str, sign);
            divided.push_str(&index1.to_string());
            remainder2_largeint = remainder_largeint.sub(&product1_largeint);
            remainder_str = remainder2_largeint.digits;
            break
        } else if i == 9 {
            divided.push_str(&index.to_string());
            remainder2_largeint = remainder_largeint.sub(&product_largeint);
            remainder_str = remainder2_largeint.digits;
            break
        }
        else if remainder_largeint.compare(&product_largeint) == Compare::Equal {
            divided.push_str(&index);
            remainder_str = String::from("0");
            break
        } else {
            remainder_str = String::from("0");
        }
    }
    (divided.clone(), remainder_str)
}
