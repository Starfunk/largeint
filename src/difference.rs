use largeint::*;

pub fn difference(large1: &str, large2: &str) -> String {
    let mut large1 = to_vec(&large1);
    let large2 = to_vec(&large2);
    let mut output_vec: Vec<u8> = Vec::new();
    let mut output_str =  String::from("");
    let len1 = large1.len();
    let len2 = large2.len();
    if len1 > len2 {
        let index = len1 - len2;
        for mut i in (0..len2).rev() {
            if large1[i + index] >= large2[i] {
                output_vec.insert(0, large1[i + index] - large2[i])
            } else if large2[i] > large1[i + index] {
                let i_2 = i;
                let mut count = 0;
                while i >= 1 && large2[i] >= large1[i + index]{
                    i = i - 1;
                    count = count + 1;
                } if i == 0 && large2[i] >= large1[i + index] {
                    let mut loopback = 0;
                    while large1[i + index - 1 - loopback] == 0 {
                        count = count + 1;
                        loopback = loopback + 1;
                    }
                    for _ in 0..count+1 {
                        large1[i + index - 1 - loopback] = large1[i + index - 1 - loopback] - 1;
                        large1[i + index - loopback] = large1[i + index - loopback] + 10;
                        if i < count {
                            i = i + 1;
                        }
                    }
                } else {
                    for _ in 0..count {
                        large1[i + index] = large1[i + index] - 1;
                        large1[i + index + 1] = large1[i + index + 1] + 10;
                        i = i + 1;
                    }
                }
                i = i_2;
                output_vec.insert(0, large1[i + index] - large2[i]);
                }
            }
            for k in (0..index).rev() {
                output_vec.insert(0, large1[k]);
        }
        output_str = vec_to_str(&output_vec);
        output_str = String::from(output_str.trim_left_matches("0"));
    } else if &large1.len() == &large2.len() {
            for mut i in (0..len1).rev() {
                if large1[i] >= large2[i] {
                    output_vec.insert(0, large1[i] - large2[i])
                } else if large2[i] > large1[i] {
                    let mut count = 0;
                    while i >= 1 && large2[i] >= large1[i]{
                        i = i - 1;
                        count = count + 1;
                    }
                        for _ in 0..count {
                        large1[i] = large1[i] - 1;
                        large1[i + 1] = large1[i + 1] + 10;
                        i = i + 1;
                        }
                    output_vec.insert(0, large1[i] - large2[i]);
                    }
                }
                output_str = vec_to_str(&output_vec);
                output_str = String::from(output_str.trim_left_matches("0"));
        }
        output_str
    }
