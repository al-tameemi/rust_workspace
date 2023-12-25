use char::Character;

const NUMBER_OFFSET: i8 = 4;
const LOWERCASE_OFFSET: i8 = -71;
const UPPERCASE_OFFSET: i8 = -65;
const PLUS_ID: u8 = 62;
const SLASH_ID: u8 = 63;
const PADDING_CHAR: char = '=';

pub fn decode(input: &str) -> Result<String, error::Error> {
    let mut final_word: String = String::from("");
    for chunk in input.as_bytes().chunks(4) {
        let chunk = chunk
            .iter()
            .map(get_byte_from_byte)
            .collect::<Result<Vec<Character>, error::Error>>()?;
        let bytes;
        match chunk.len() {
            4 => {
                bytes = convert_to_ascii_bytes_from_array(&chunk)?;
            }
            3 => bytes = convert_to_ascii_bytes(chunk[0], chunk[1], chunk[2], Character::Padding)?,
            2 => {
                bytes = convert_to_ascii_bytes(
                    chunk[0],
                    chunk[1],
                    Character::Padding,
                    Character::Padding,
                )?
            }
            1 => return Err(error::Error::InvalidBytes([chunk[0].into(), ' ', ' ', ' '])),
            _ => {
                unreachable!()
            }
        }

        final_word.push_str(&bytes.iter().filter_map(|c| *c).collect::<String>());
    }
    Ok(final_word)
}

fn convert_to_ascii_bytes_from_array(
    bytes: &[Character],
) -> Result<[Option<char>; 3], error::Error> {
    if bytes.len() == 4 {
        convert_to_ascii_bytes(bytes[0], bytes[1], bytes[2], bytes[3])
    } else {
        panic!("Invalid size of chunk. Implementation error.")
    }
}

/// Converts a 4 base64 bytes into three ascii bytes
fn convert_to_ascii_bytes(
    byte_1: Character,
    byte_2: Character,
    byte_3: Character,
    byte_4: Character,
) -> Result<[Option<char>; 3], error::Error> {
    match (byte_1, byte_2, byte_3, byte_4) {
        (
            Character::B64(byte_1),
            Character::B64(byte_2),
            Character::B64(byte_3),
            Character::B64(byte_4),
        ) => Ok([
            Some((byte_1 << 2 | byte_2 >> 4) as char),
            Some((byte_2 << 4 | byte_3 >> 2) as char),
            Some((byte_3 << 6 | byte_4) as char),
        ]),
        (
            Character::B64(byte_1),
            Character::B64(byte_2),
            Character::B64(byte_3),
            Character::Padding,
        ) => Ok([
            Some((byte_1 << 2 | byte_2 >> 4) as char),
            Some((byte_2 << 4 | byte_3 >> 2) as char),
            None,
        ]),
        (
            Character::B64(byte_1),
            Character::B64(byte_2),
            Character::Padding,
            Character::Padding,
        ) => Ok([Some((byte_1 << 2 | byte_2 >> 4) as char), None, None]),
        _ => Err(error::Error::InvalidBytes([
            byte_1.into(),
            byte_2.into(),
            byte_3.into(),
            byte_4.into(),
        ])),
    }
}

/// Converts a base64 character into its binary representation.
pub fn get_byte(character: char) -> Result<Character, error::Error> {
    let char_code = character as u32;
    if char_code > 127 {
        return Err(error::Error::InvalidCharacter(character));
    }
    get_byte_from_byte(&(char_code as u8))
}

/// Converts a base64 character as a byte into its binary representation.
fn get_byte_from_byte(char_code: &u8) -> Result<Character, error::Error> {
    let char_code = *char_code;
    match char_code {
        65..=90 => {
            // Uppercase Character
            Ok((char_code as i8 + UPPERCASE_OFFSET).try_into()?)
        }
        97..=122 => {
            // Lowercase Character
            Ok((char_code as i8 + LOWERCASE_OFFSET).try_into()?)
        }
        48..=57 => {
            // Number
            Ok((char_code as i8 + NUMBER_OFFSET).try_into()?)
        }
        b'+' => Ok(PLUS_ID.into()),
        b'/' => Ok(SLASH_ID.into()),
        b'=' => Ok(Character::Padding),
        _ => Err(error::Error::InvalidCharacter(
            char::from_u32(char_code.into()).unwrap(),
        )),
    }
}

mod char {
    use crate::error::Error;

    #[derive(Debug, Clone, Copy)]
    pub enum Character {
        B64(u8),
        Padding,
    }

    impl TryFrom<i8> for Character {
        type Error = Error;
        fn try_from(value: i8) -> Result<Self, Self::Error> {
            if value < 0 {
                return Err(Error::ExpectedPositiveInteger(value));
            }
            Ok(Character::B64(value.try_into().unwrap()))
        }
    }

    impl From<u8> for Character {
        fn from(value: u8) -> Self {
            Character::B64(value)
        }
    }

    #[allow(clippy::from_over_into)]
    impl Into<char> for Character {
        fn into(self) -> char {
            match self {
                Self::Padding => '=',
                Self::B64(c) => c.into(),
            }
        }
    }
}

mod error {
    use std::fmt::Display;

    use crate::PADDING_CHAR;

    #[derive(Debug, PartialEq)]
    pub enum Error {
        ExpectedPadded(String),
        ExpectedUnpadded(String),
        InvalidCharacter(char),
        ExpectedPositiveInteger(i8),
        InvalidBytes([char; 4]),
    }

    impl Display for Error {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Error::ExpectedPadded(culprit) => {
                    write!(f, "Expected padded base64, got unpadded: `{culprit}`\n\
                    Padded base64 needs to consist of only 4 char chunks and should be padded with `{PADDING_CHAR}`")
                }
                Error::ExpectedUnpadded(culprit) => {
                    write!(f, "Expected unpadded base64, got padded: `{culprit}`\n\
                    Unpadded base64 should not contain the padding character (`{PADDING_CHAR}`) when they're not made of 4 char chunks")
                }
                Error::InvalidCharacter(culprit) => {
                    write!(f, "Invalid character found: `{culprit}`")
                }
                Error::ExpectedPositiveInteger(culprit) => {
                    write!(f, "Expected a positive integer, got `{culprit}` instead.")
                }
                Error::InvalidBytes([byte_1, byte_2, byte_3, byte_4]) => {
                    write!(f, "Following characters cannot be decoded [`{byte_1}`,`{byte_2}`,`{byte_3}`,`{byte_4}`]")
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::decode;

    #[test]
    fn test_decode_padded() {
        // With padding
        let message = "SGVsbG8gMTIzNSAtIElzIHRoaXMgd29ya2luZz8=";
        let message_decoded = decode(message).unwrap();
        assert_eq!(message_decoded, "Hello 1235 - Is this working?");

        // Padding omitted
        let message = "SGVsbG8gMTIzNSAtIElzIHRoaXMgd29ya2luZz8";
        let message_decoded = decode(message).unwrap();
        assert_eq!(message_decoded, "Hello 1235 - Is this working?");

        // Invalid
        let invalid = "SGVsbG8gMTIzNSAtIElzIHRoaXMgd29ya2luZz8==";
        let message_decoded = decode(invalid);
        assert!(message_decoded.is_err());
    }
}
