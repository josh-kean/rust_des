pub mod sbox;
pub mod permutations;
pub mod rounds;

#[allow(non_snake_case)]

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xor() {
        let x1 = vec![1,0,0];
        let x2 = vec![0,1,0];
        assert_eq!(vec_xor(&x1, &x2), vec![1,1,0]);
    }

    fn problem1(x1: Vec<u8>, x2: Vec<u8>) {
        //show that the result of xor sbox1(x1) and sbox1(x2) is different from sbox1(x1 xor x2)
        //compare the variables s1_xor_s2 and s_x1_xor_x2
        let sbox = sbox::SBox::new();
        let x1 = vec![0,0,0,0,0,0];
        let x2 = vec![0,0,0,0,0,1];
        let x1_xor_x2: Vec<u8> = vec_xor(&x1, &x2);
        let sbox_x1: u8 = sbox.sbox1(1, &x1);
        let sbox_x2: u8 = sbox.sbox1(1, &x2);
        let sbox_x1_xor_x2: u8 = sbox.sbox1(1, &x1_xor_x2);
        //comparing 2 integers
        assert_ne!(sbox_x1^sbox_x2, sbox_x1_xor_x2);
    }

    #[test]
    fn problem_1_1() {
        problem1(vec![0,0,0,0,0,0], vec![0,0,0,0,0,1]);
    }

    #[test]
    fn problem_1_2() {
        problem1(vec![1,1,1,1,1,1], vec![1,0,0,0,0,0]);
    }

    #[test]
    fn problem_1_3() {
        problem1(vec![1,0,1,0,1,0], vec![0,1,0,1,0,1]);
    }

    #[test]
    fn problem_2() {
        //verify that initial permutation encrypt is the same as initial permutate for decrypt
        let permutate = initial_permutation::Permutations::new();
        let mut input: Vec<u8> = vec![1,2,3,4,5];
        let initial: Vec<u8> = permutate.permutate(true, &input);
        let reverse: Vec<u8> = permutate.permutate(true, &initial);
        while input.len() < 64{ //pads input with zero's on the end
            input.insert(input.len(), 0);
        }
        assert_eq!(input, reverse);
    }

    #[test]
    fn problem_3() {
        let plaintext: Vec<u8> = vec![0;64];
        let key: Vec<u8> = vec![0;64];
    }

}


pub fn vec_xor(v1: &Vec<u8>, v2: &Vec<u8>) -> Vec<u8> {
    v1.iter().zip(v2.iter()).map(|x| x.0 != x.1).map(|x| if x {1} else {0}).collect()
}
