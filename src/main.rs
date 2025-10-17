use std::io::Read;


fn read_compact_size(transaction_bytes: &mut &[u8]) -> u64 {
    let mut compact_size = [0_u8; 1];
    transaction_bytes.read(&mut compact_size).unwrap();

    match compact_size[0] {
        0..=252 => compact_size[0] as u64,
        253 => {
            let mut buffer = [0; 2];
            transaction_bytes.read(&mut buffer).unwrap();
            u16::from_le_bytes(buffer) as u64
        },
        254 => {
            let mut buffer = [0; 4];
            transaction_bytes.read(&mut buffer).unwrap();
            u32::from_le_bytes(buffer) as u64
        },
        255 => {
            let mut buffer = [0; 8];
            transaction_bytes.read(&mut buffer).unwrap();
            u64::from_le_bytes(buffer)
        }
    }
}
fn read_version(transaction_bytes: &mut &[u8]) -> u32 {

    let mut buffer = [0; 4];
    transaction_bytes.read(&mut buffer).unwrap();
    u32::from_le_bytes(buffer)
}
fn main() {
    let transaction_hex = "02000000000101d09027b7704128d6168122212afbcb5d960e572f59a879d60dc53370f382cab40000000000fdffffff0227be16000000000016001498779af6da1c0b3bf20be047a1b6e3491ab991790000000000000000156a5d1214011400ff7f818cec82d08bc0a88281d215024730440220233215e028be213467a97eac37951a8e9bd2b41e6630107ad3b8f5ceb7dbcc4a0220320e24c70fafcab86dbc4fcdd6f3fddcd6e6f54e785631ddfbe00d1a89fff4d20121024f5b5d052d4d76b514bacbd8407e6297f4043739e9f359ee3c9baa716332aafc00000000";

    let transaction_bytes = hex::decode(transaction_hex).unwrap();
    let mut bytes_slice = transaction_bytes.as_slice();

    let version = read_version(&mut bytes_slice);

    // Skip marker and flag (2 bytes)
    let mut segwit_header = [0u8; 2];
    bytes_slice.read(&mut segwit_header).unwrap();


    let input_length = read_compact_size(&mut bytes_slice);

    println!("version: {}", version);
    println!("input_length: {}", input_length);
}


