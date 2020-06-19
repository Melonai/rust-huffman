use std::collections::BTreeMap;
use std::convert::TryInto;

pub fn decode(mut buffer: Vec<u8>) {
    let (map_length, payload) = buffer.split_first_mut().unwrap();
    let indices = read_map(*map_length, payload);
}

fn read_map(map_length: u8, mut payload: &mut [u8]) -> BTreeMap<u8, Vec<bool>> {
    let mut indices = BTreeMap::new();
    for _ in 0..map_length {
        let (info, payload_no_info) = payload.split_at_mut(2);
        let &mut [character, bit_length]: &mut [u8; 2] = info.try_into().unwrap();
        let byte_length = (bit_length as f32 / 8.0).ceil() as usize;

        let (index_bytes, new_payload) = payload_no_info.split_at_mut(byte_length);

        let mut index = Vec::new();
        for bit in 0..bit_length {
            let byte_index = bit / 8;
            let bit_index = (bit_length - 1 - byte_index * 8).min(7) - bit % 8;
            index.push(index_bytes[byte_index as usize] & (1 << bit_index) != 0);
        }

        indices.insert(character, index);
        payload = new_payload;
    }
    indices
}
