use self::constants::punctuators;

pub mod constants {
    pub const WHITESPACE: &[u8; 2] = &[b' ', b'\n'];
    pub const NULL: &[u8] = "null".as_bytes();
    pub const TRUE: &[u8] = "true".as_bytes();
    pub const FALSE: &[u8] = "false".as_bytes();
    pub mod punctuators {
        pub const ARRAY_OPEN: u8 = b'[';
        pub const ARRAY_CLOSE: u8 = b']';
        pub const ARRAY_DELIMITER: u8 = b',';
        pub const OBJECT_OPEN: u8 = b'{';
        pub const OBJECT_CLOSE: u8 = b'}';
        // this isnt useful because its the same as array delim
        // pub const OBJECT_ENTRY_DELIMITER: u8 = b',';
        pub const OBJECT_KV_DELIMITER: u8 = b':';
        pub const STRING_BOUNDARY: u8 = b'"';
        pub const ESCAPE: u8 = b'\\';
        pub const NUMBER_DECIMAL_DELIMITER: u8 = b'.';
        pub const NEGATIVE: u8 = b'-';
    }
}

pub fn is_numeric(input: u8) -> bool {
    (b'0'..=b'9').contains(&input)
}

pub fn is_numeric_or_decimal_point(input: u8) -> bool {
    (b'0'..=b'9').contains(&input) || input == punctuators::NUMBER_DECIMAL_DELIMITER
}

pub fn is_numeric_or_negative(input: u8) -> bool {
    (b'0'..=b'9').contains(&input) || input == punctuators::NEGATIVE
}