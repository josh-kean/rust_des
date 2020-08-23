pub mod permutations {
    fn permutate(data: &Vec<u8>, vector: &Vec<u8>) -> Vec<u8> {
        vector.iter().map(|x| data[*x as usize]).collect()
    }

    //converts key from 64 to 56 bits
    pub fn pc_1(data: &Vec<u8>) -> Vec<u8> {
        let pc_1 = vec![
            57 ,49 ,41 ,33 ,25 ,17 ,9 ,1,
            58 ,50 ,42 ,34 ,26 ,18 ,10 ,2,
            59 ,51 ,43 ,35 ,27 ,19 ,11 ,3,
            60 ,52 ,44 ,36 ,63 ,55 ,47 ,39,
            31 ,23 ,15 ,7 ,62 ,54 ,46 ,38,
            30 ,22 ,14 ,6 ,61 ,53 ,45 ,37,
            29 ,21 ,13 ,5 ,28 ,20 ,12 ,4,
        ];
        permutate(data, &pc_1)
    }

    //converts key from 56 to 48 bits
    pub fn pc_2(data: &Vec<u8>) -> Vec<u8> {
        let pc_2 = vec![
            14 ,17 ,11 ,24 ,01 ,05 ,03 ,28,
            15 ,06 ,21 ,10 ,23 ,19 ,12 ,04,
            26 ,08 ,16 ,07 ,27 ,20 ,13 ,02,
            41 ,52 ,31 ,37 ,47 ,55 ,30 ,40,
            51 ,45 ,33 ,48 ,44 ,49 ,39 ,56,
            34 ,53 ,46 ,42 ,50 ,36 ,29 ,32,
        ];
        permutate(data, &pc_2)
    }

    //diffusion done at the end of a round
    pub fn diffusion(data: &Vec<u8>) -> Vec<u8> {
        let diffusion_vector: Vec<u8> = vec![
            16 ,7 ,20 ,21 ,29 ,12 ,28 ,17,
            1 ,15 ,23 ,26 ,5 ,18 ,31 ,10,
            2 ,8 ,24 ,14 ,32 ,27 ,3 ,9,
            19 ,13 ,30 ,6 ,22 ,11 ,4 ,25,
        ];
        permutate(data, &diffusion_vector)
    }

    //done at the start of an encryption cycle, only done once
    pub fn encryption_permutation(data: &Vec<u8>) -> Vec<u8> {
        let initial_vector: Vec<u8> = vec![
            58, 50, 42, 34, 26, 18, 10, 2,
            60, 52, 44, 36, 28, 20, 12, 4,
            64, 56, 48, 40, 32, 24, 16, 8,
            57, 49, 41, 33, 25, 17, 9, 1,
            59, 51, 43, 35, 27, 19, 11, 3,
            61, 53, 45, 37, 29, 21, 13, 5,
            63, 55, 47, 39, 31, 23, 15, 7
        ];
        permutate(data, &initial_vector)
    }
    
    //done at the start of an decryption cycle, only done once
    pub fn decription_permutation(data: &Vec<u8>) -> Vec<u8> {
        let initial_vector: Vec<u8> = vec![
            40, 8, 48, 16, 56, 24, 64, 32,
            39, 7, 47, 15, 55, 23, 63, 31,
            38, 6, 46, 14, 54, 22, 62, 30,
            37, 5, 45, 13, 53, 21, 61, 29,
            36, 4, 44, 12, 52, 20, 60, 28,
            35, 3, 43, 11, 51, 19, 59, 27,
            34, 2, 42, 10, 50, 18, 58, 26,
            33, 1, 41, 9, 49, 17, 57, 25,
        ];
        permutate(data, &initial_vector)
    }

    //done at the start of each round to expand from 36 to 48 bits
    pub fn expansion(data: &Vec<u8>) -> Vec<u8> {
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
        permutate(data, &expansion_vector)
    }
}
