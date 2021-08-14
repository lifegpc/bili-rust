use std::collections::HashMap;

lazy_static! {
    static ref TABLE: Vec<char> = vec![
        'f', 'Z', 'o', 'd', 'R', '9', 'X', 'Q', 'D', 'S', 'U', 'm', '2', '1', 'y', 'C', 'k', 'r',
        '6', 'z', 'B', 'q', 'i', 'v', 'e', 'Y', 'a', 'h', '8', 'b', 't', '4', 'x', 's', 'W', 'p',
        'H', 'n', 'J', 'E', '7', 'j', 'L', '5', 'V', 'G', '3', 'g', 'u', 'M', 'T', 'K', 'N', 'P',
        'A', 'w', 'c', 'F'
    ];
    static ref TR: HashMap<char, usize> = {
        let mut tr: HashMap<char, usize> = HashMap::new();
        for i in 0..TABLE.len() {
            tr.insert(TABLE[i], i);
        }
        tr
    };
    static ref SS: Vec<usize> = vec![11, 10, 3, 8, 4, 6];
    static ref XOR: usize = 177451812;
    static ref ADD: usize = 8728348608;
}

/// Convert av number to bv number
/// * `av` - Av number
pub fn av_to_bv(av: usize) -> String {
    let x = (av ^ *XOR) + *ADD;
    let mut r = vec!['B', 'V', '1', ' ', ' ', '4', ' ', '1', ' ', '7', ' ', ' '];
    for i in 0..6 {
        let ind = SS[i];
        let ind2 = x / (58 as usize).pow(i as u32) % 58;
        r[ind] = TABLE[ind2];
    }
    r.into_iter().collect()
}

pub fn bv_to_av(bv: String) -> usize {
    let s = if bv.len() == 11 {
        String::from("BV1") + &bv[2..]
    } else {
        bv.clone()
    };
    let bv: Vec<char> = s.chars().collect();
    let mut r = 0;
    for i in 0..6 {
        let c = bv[SS[i]];
        r += TR.get(&c).unwrap() * (58 as usize).pow(i as u32);
    }
    (r - *ADD) ^ *XOR
}

#[test]
fn test_av_to_bv() {
    assert_eq!("BV17x411w7KC", av_to_bv(170001));
    assert_eq!("BV1xx411c7mC", av_to_bv(9));
}

#[test]
fn test_bv_to_av() {
    assert_eq!(170001, bv_to_av(String::from("BV17x411w7KC")));
    assert_eq!(9, bv_to_av(String::from("BV1xx411c7mC")));
    assert_eq!(170001, bv_to_av(String::from("BV7x411w7KC")));
    assert_eq!(9, bv_to_av(String::from("BVxx411c7mC")));
}
