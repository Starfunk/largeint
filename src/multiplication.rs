use largeint::*;
use addition::addition_2;

pub fn multiplication_1(large1: &Vec<u8>, large2: &Vec<u8>) -> Vec<u8> {
    let mut output_vec: Vec<u8> = Vec::new();
    let mut counter: u8 = 0;
    let start = large2[large2.len() - 1];
    for i in large1.iter().rev() {
        if i * start + counter <= 9 {
            output_vec.insert(0, i * start + counter);
            counter = 0;
        } else if i * start + counter > 9 {
            let carry_output = carry(i * start + counter);
            output_vec.insert(0, carry_output[1]);
            counter = carry_output[0];
        }
    }
    if counter != 0 {
        output_vec.insert(0, counter);
    }
    output_vec
}

pub fn multiplication_2(large1: &str, large2: &str) -> String {
    let large1 = to_vec(&large1);
    let large2 = to_vec(&large2);
    let large2_length = large2.len();
    let mut output_vec = multiplication_1(&large1, &vec![large2[large2_length - 1]]);
    if large2_length > 1 {
        let mut counter = 1;
        for i in (0..large2.len() - 1).rev() {
            let mut output_vec2 = multiplication_1(&large1, &vec![large2[i]]);
            for _ in 0..counter {
                output_vec2.push(0);
            }
            counter += 1;
            output_vec = addition_2(output_vec2, output_vec);
        }
    }
    let output_str = vec_to_str(&output_vec);
    output_str
}

fn carry(num: u8) -> Vec<u8> {
    let output_vec = match num {
        10...19 => vec![1, num - 10],
        20...29 => vec![2, num - 20],
        30...39 => vec![3, num - 30],
        40...49 => vec![4, num - 40],
        50...59 => vec![5, num - 50],
        60...69 => vec![6, num - 60],
        70...79 => vec![7, num - 70],
        80...89 => vec![8, num - 80],
        _ => panic!("Something has gone very wrong in the multiplication step. i * j + 8
        should not equal more than 89! See carry in the multiplication module."),
    };
    output_vec
}
