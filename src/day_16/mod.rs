
use hex;
use bitvec::prelude::*;
pub struct PacketDecoder {

}

// 375479

// Header
// first 3 - version
// 3 - type ID
// 


// Type ID
// 4 - literal value
// any other - operator


impl crate::Advent for PacketDecoder {
    fn new(data: &str) -> PacketDecoder {
        let line = data
            .lines()
            .next()
            .unwrap();
        let hex = hex::decode(line);              
        println!("Hex data: {:?}", hex);
        let bit_vec = if let Ok(hex) = hex {
            BitVec::<_, Msb0>::try_from_vec(hex).unwrap()
        } else {
            BitVec::new()
        };

        println!("Bit vec: {:?}", bit_vec);
        

        PacketDecoder {}
    }

    fn part1(&mut self) -> usize {
        1
    }

    fn part2(&mut self) -> usize {
        2
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn get_bit() {
        let byte: u8 = 32;
        assert_eq!(byte.count_ones(), 1);
    }
}