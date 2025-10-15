fn read_version(transaction_hex: &str) -> u32 {
    let transaction_bytes = hex::decode(transaction_hex).unwrap();
    let version_bytes = &transaction_bytes[0..4];
    println!("version_bytes: {:?}", version_bytes);
    1
}
fn main() {
    let version = read_version("02000000000101d09027b7704128d6168122212afbcb5d960e572f59a879d60dc53370f382cab40000000000fdffffff0227be16000000000016001498779af6da1c0b3bf20be047a1b6e3491ab991790000000000000000156a5d1214011400ff7f818cec82d08bc0a88281d215024730440220233215e028be213467a97eac37951a8e9bd2b41e6630107ad3b8f5ceb7dbcc4a0220320e24c70fafcab86dbc4fcdd6f3fddcd6e6f54e785631ddfbe00d1a89fff4d20121024f5b5d052d4d76b514bacbd8407e6297f4043739e9f359ee3c9baa716332aafc00000000");
    println!("version: {}", version);
}


