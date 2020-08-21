pub struct DataPermutations {
    initial: Vec<u8>,
    reverse: Vec<u8>
}

impl DataPermutations {
    pub fn new() -> DataPermutations {
        Permutations {
            initial: vec![
        58, 50, 42, 34, 26, 18, 10, 2,
        60, 52, 44, 36, 28, 20, 12, 4,
        64, 56, 48, 40, 32, 24, 16, 8,
        57, 49, 41, 33, 25, 17, 9, 1,
        59, 51, 43, 35, 27, 19, 11, 3,
        61, 53, 45, 37, 29, 21, 13, 5,
        63, 55, 47, 39, 31, 23, 15, 7
            ],
            reverse: vec![
        40, 8, 48, 16, 56, 24, 64, 32,
        39, 7, 47, 15, 55, 23, 63, 31,
        38, 6, 46, 14, 54, 22, 62, 30,
        37, 5, 45, 13, 53, 21, 61, 29,
        36, 4, 44, 12, 52, 20, 60, 28,
        35, 3, 43, 11, 51, 19, 59, 27,
        34, 2, 42, 10, 50, 18, 58, 26,
        33, 1, 41, 9, 49, 17, 57, 25,
            ]
        }
    }

    pub fn permutate(&self, stage: bool, data: &Vec<u8>) -> Vec<u8> {
        let permutation_table = if stage {&self.initial} else {&self.reverse};
        let mut input: Vec<u8> = vec![0; 64];
        for i in 0..data.len() {
            input[i] = data[i];
        }
        (0..64).map(|x| input[x]).collect()
    }
}

pub struct KeyPermutations {
   pc_1: Vec<u8>,
   pc_2: Vec<u8>
}

impl KeyPermutations {
    pub fn new() -> KeyPermutations {
        KeyPermutations {
            pc_1: vec![
            ],
            pc_2: vec![
            ]
        }
    } 
}
