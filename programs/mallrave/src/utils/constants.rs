pub const ANCHOR_BUFFER: usize = 8;
pub const MAIN_ACCOUNT: &[u8; 12] = b"MAIN_ACCOUNT";
pub const MAX_NAME: usize = 32;
pub const MAX_DESCRIPTION: usize = 200;
// why 9971 accounts unless than 10000?
// 9971 = 311 * 32(pubkey) -> the max number of account in the AccountData struct
pub const MAX_PRODUCTS: usize = 9971;
// why 9686 accounts unless than 10000?
// 9686 = 10000 - all static space in Product struct
pub const MAX_IMAGE_URL: usize = 9686;
