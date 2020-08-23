pub use crate::sbox;
pub use crate::permutations;

//before any round happens the initial permutation happens

pub struct SplitData {
    left: Vec<u8>,
    right: Vec<u8>
}

pub mod round {
    /* the following happens each round
     * the 32 bit input is expanded to 48 bits
     * the48 bits are xored with the key schedule
     * the 48 bits are split into 8 groups of 6 bits
     * the 6bit groups are processed through sboxes turning them into 4 bit numbers
     * the 4 bit numbers are put through a diffusion permutation table
     */
    use super::*;
    use sbox::boxes::SBox;
    use permutations::permutations::permutations;
    
    //xor function
    fn xor_with_key(data: &Vec<u8>, key: &Vec<u8>) -> Vec<u8> {
        data.iter().zip(key.iter()).map(|x| if x.0 != x.1 {1} else {0}).collect()
    }

    //splits data into 2 equal halves
    fn split_data(data: &Vec<u8>) -> SplitData {
        let (left, right) = data.split_at(data.len());
        SplitData{
            left: left.to_vec(),
            right: right.to_vec()
        }
    }

    //keys are rotated in this function
    fn rotate_keys(round: u8, keys: SplitData) -> SplitData {
        let mut left = keys.left;
        let mut right = keys.right;
        left.rotate_left(1);
        right.rotate_left(1);
        if vec![2,3,4,5,6,7,9 ,10,11,12,13,14].contains(&round) {
            left.rotate_left(1);
            right.rotate_left(1);
        }
        SplitData{
            left: left.to_vec(),
            right: right.to_vec()
        }
    }

    //splits 48 bit data into 6 sets of 8 bit chunks, and returns the sbox value of each
    //want this to return 36 bit vector
    fn chunk_data(data: &Vec<u8>) -> Vec<u8> {
        let sbox = SBox::new();
        let result: Vec<u8> = data.as_slice()
            .chunks(6)
            .enumerate()
            .map(|x| sbox.diffusion(x.0 as u8, &Vec::from(x.1))).collect();
        let result: Vec<String> = result.iter().map(|x| format!("{:04b}", x)).collect();
        let result: Vec<char> = result.iter().flat_map(|x| x.chars()).collect();
        let result: Vec<u8> = result.iter().map(|x| x.to_digit(10).unwrap() as u8).collect();
        result
    }
    fn initialization(data: Vec<u8>, key: Vec<u8>) -> (SplitData, SplitData) {
        let data = permutations::encryption_permutation(&data);
        let key = permutations::pc_1(&key);
        let split_key = split_data(&key);
        let split_input = split_data(&data);
        (split_input, split_key)
    }

    pub fn round(round: u8, data: SplitData, key: SplitData) -> (SplitData, SplitData) {
        //get round key
        let key = rotate_keys(round, key); //rotate the keys for the round
        let rk = [key.left.as_slice(), key.right.as_slice()].join(&0);
        let rk = permutations::pc_2(&rk); //round key to be used for this specific round
        //expand data from 36 to 48 bits
        let expanded_data = permutations::expansion(&data.right); 
        let x_or_data = xor_with_key(&expanded_data, &rk);
        let sbox_result = chunk_data(&x_or_data);
        let right_data = permutations::diffusion(&sbox_result);
        let left_data = data.right;
        (SplitData {
            left: left_data,
            right: right_data
        }, key)
    }

    pub fn encryption(data: Vec<u8>, key: Vec<u8>) -> (SplitData, SplitData) {
        let (mut data, mut key) = initialization(data, key);
        for r in 0..8 {
            let result = round(r, data, key);
            data = result.0;
            key = result.1;
        }
        (data, key)
    }
}
