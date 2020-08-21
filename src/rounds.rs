pub use crate::sbox;

//before any round happens the initial permutation happens

pub mod round {
    use super::*;
    use sbox::boxes::SBox;
    fn expansion(data: &Vec<u8>) -> Vec<u8> {
        let expansion_vector: Vec<u8> = vec![
            32 ,1 ,2 ,3 ,4 ,5,
            4 ,5 ,6 ,7 ,8 ,9,
            8 ,9 ,10 ,11 ,12 ,13,
            12 ,13 ,14 ,15 ,16 ,17,
            16 ,17 ,18 ,19 ,20 ,21,
            20 ,21 ,22 ,23 ,24 ,25,
            24 ,25 ,26 ,27 ,28 ,29,
            28 ,29 ,30 ,31 ,32 ,1
        ];
        expansion_vector.iter().map(|x| data[(*x as usize)]).collect()
    }

    //xor function
    fn xor_with_key(data: &Vec<u8>, key: &Vec<u8>) -> Vec<u8> {
        data.iter().zip(key.iter()).map(|x| if x.0 != x.1 {1} else {0}).collect()
    }

    //splits 48 bit data into 6 sets of 8 bit chunks, and returns the sbox value of each
    fn chunk_data(data: &Vec<u8>) -> Vec<u8> {
        let sbox = SBox::new();
        let mut count = 1;
        let mut sbox_result: Vec<u8> = Vec::new();
        data.as_slice().chunks(6)
            .enumerate()
            .map(|x| sbox.diffusion((x.0 as u8), &Vec::from(x.1))).collect()
    }

    pub fn round(
    use super::*;
}