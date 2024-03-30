use solana_sdk::signature::Keypair;

pub fn auth_keypair() -> Keypair {
    let bytes_auth_vec = vec![
        198, 214, 173, 4, 113, 67, 147, 103, 75, 216, 80, 150, 174, 158, 63, 61, 10, 228, 165, 151,
        189, 0, 34, 29, 24, 166, 40, 136, 166, 58, 116, 242, 35, 218, 175, 128, 50, 244, 240, 13,
        176, 112, 152, 243, 132, 142, 93, 20, 112, 225, 9, 103, 175, 8, 161, 234, 247, 176, 242,
        78, 131, 96, 57, 100,
    ];
    let bytes_auth = bytes_auth_vec.as_slice();
    let auth_keypair = Keypair::from_bytes(bytes_auth).unwrap();
    auth_keypair
}
