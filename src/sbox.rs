pub struct SBox {
    pub box1: Vec<u8>,
    pub box2: Vec<u8>,
    pub box3: Vec<u8>,
    pub box4: Vec<u8>,
    pub box5: Vec<u8>,
    pub box6: Vec<u8>,
    pub box7: Vec<u8>,
    pub box8: Vec<u8>,
}

impl SBox {
    pub fn new() -> SBox {
        SBox {
            box1: vec![
                14,4,13,1,2,15,11,8,3,10,6,12,5,9,0,7,
                0,15,7,4,14,2,13,1,10,6,12,11,9,5,3,8,
                4,1,14,8,13,6,2,11,15,12,9,7,3,10,5,0,
                15,12,8,2,4,9,1,7,5,11,3,14,10,0,6,13
            ],
            box2: vec![1],
            box3: vec![1],
            box4: vec![1],
            box5: vec![1],
            box6: vec![1],
            box7: vec![1],
            box8: vec![1],
        }
    }

    //takes in a 6 bit list and returns an xbox integer
    pub fn sbox1(&self, _sbox: u8, data: &Vec<u8>) -> u8 {
        //determine switch box
        let row : u8= data[0]+data[5];
        let column: u8= (data[1]..data[5]).sum();
        self.box1[(row*column) as usize]
    }
}
