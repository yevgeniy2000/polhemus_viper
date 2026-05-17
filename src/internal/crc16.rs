fn crc16_step(crc: &mut u32, data_byte: u8) {
    let op: [u8; 16] = [0, 1, 1, 0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 1, 0];
    let mut data = (data_byte as u32 ^ (*crc)) & 0xff;
    *crc >>= 8;

    // Parity check over the 8 bits of data
    if op[(data & 0xf) as usize] ^ op[(data >> 4) as usize] != 0 {
        *crc ^= 0xc001;
    }

    data <<= 6;
    *crc ^= data;
    data <<= 1;
    *crc ^= data;
}

/// The main function for CRC calculation (corresponds to Viper_CalcCRC_Bytes)
pub(crate) fn crc16(data: &[u8]) -> u16 {
    let mut crc: u32 = 0; // Start value 0 according to C code
    for &byte in data {
        crc16_step(&mut crc, byte);
    }
    crc as u16
}