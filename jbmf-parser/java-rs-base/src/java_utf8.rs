use std::slice;

use crate::error::{Error, UtfConversionError};

static UTF8_CHAR_WIDTH: [u8; 256] = [
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, // 0x1F
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, // 0x3F
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, // 0x5F
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, // 0x7F
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 0x9F
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 0xBF
    0, 0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, // 0xDF
    3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, // 0xEF
    4, 4, 4, 4, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 0xFF
];

#[inline]
pub fn utf8_char_width(b: u8) -> usize {
    UTF8_CHAR_WIDTH[b as usize] as usize
}

const CONT_MASK: u8 = 0b00111111u8;
const TAG_CONT_U8: u8 = 0b10000000u8;

pub trait FromJavaUtf8Ext {
    fn from_java_utf8(vec: &[u8]) -> Result<Self, Error>
    where
        Self: std::marker::Sized;
}

pub trait ToJavaUtf8Ext {
    fn to_java_utf8(&self) -> Result<Vec<u8>, Error>;
}

impl FromJavaUtf8Ext for String {
    fn from_java_utf8(bytes: &[u8]) -> Result<Self, Error> {
        if !is_valid_java_utf8(bytes) {
            return Err(Error::UtfConversionError(UtfConversionError::InvalidJavaUtf8(
                bytes.to_vec(),
            )));
        }

        match std::str::from_utf8(bytes) {
            Ok(value) => Ok(String::from(value)),
            _ => {
                let mut decoded = Vec::with_capacity(bytes.len());

                match decode_from_java_utf8(&mut decoded, &mut bytes.iter()) {
                    Some(e) => Err(e),
                    None => unsafe { Ok(String::from_utf8_unchecked(decoded)) },
                }
            }
        }
    }
}

fn decode_from_java_utf8(decoded: &mut Vec<u8>, iter: &mut slice::Iter<u8>) -> Option<Error> {
    loop {
        let first = match iter.next() {
            Some(&b) => b,
            None => return None,
        };

        let next_continuation = |c: Option<&u8>| match c {
            None => Err(Error::UtfConversionError(UtfConversionError::UnexpectedEndOfData)),
            Some(b) => match *b {
                v if v & !CONT_MASK == TAG_CONT_U8 => Ok(v),
                _ => Err(Error::UtfConversionError(UtfConversionError::UnexpectedContinuation(
                    *b,
                ))),
            },
        };

        if 0 == first {
            return Some(Error::UtfConversionError(UtfConversionError::NullByteFound));
        } else if 128 >= first {
            decoded.push(first);
        } else if 0xc0 == first {
            match iter.next() {
                None => return Some(Error::UtfConversionError(UtfConversionError::UnexpectedEndOfData)),
                Some(b) => match *b {
                    0x80 => decoded.push(0),
                    _ => {
                        return Some(Error::UtfConversionError(UtfConversionError::UnexpectedContinuation(
                            *b,
                        )));
                    }
                },
            }
        } else {
            let width = utf8_char_width(first);
            let second = match next_continuation(iter.next()) {
                Err(e) => return Some(e),
                Ok(v) => v,
            };

            match width {
                2 => {
                    decoded.extend([first, second].iter().cloned());
                }
                3 => {
                    let third = match next_continuation(iter.next()) {
                        Err(e) => return Some(e),
                        Ok(v) => v,
                    };

                    match (first, second) {
                        (0xE0, 0xA0..=0xBF)
                        | (0xE1..=0xEC, 0x80..=0xBF)
                        | (0xED, 0x80..=0x9F)
                        | (0xEE..=0xEF, 0x80..=0xBF) => decoded.extend([first, second, third].iter().cloned()),

                        (0xED, 0xA0..=0xAF) => {
                            match iter.next() {
                                Some(&x) if x == 0xED => {
                                    return Some(Error::UtfConversionError(
                                        UtfConversionError::UnexpectedContinuation(x),
                                    ));
                                }
                                _ => {}
                            }

                            let fifth = match next_continuation(iter.next()) {
                                Err(e) => return Some(e),
                                Ok(v) => match v {
                                    v if v < 0xB0 || 0xBF < v => {
                                        return Some(Error::UtfConversionError(
                                            UtfConversionError::UnexpectedContinuation(v),
                                        ));
                                    }
                                    v => v,
                                },
                            };

                            let sixth = match next_continuation(iter.next()) {
                                Err(e) => return Some(e),
                                Ok(v) => v,
                            };

                            let surrogate = decode_surrogates(second, third, fifth, sixth);
                            decoded.extend(surrogate.iter().cloned());
                        }
                        _ => {
                            return Some(Error::UtfConversionError(UtfConversionError::UnexpectedContinuation(
                                second,
                            )));
                        }
                    }
                }
                _ => {
                    return Some(Error::UtfConversionError(UtfConversionError::UnexpectedContinuation(
                        first,
                    )));
                }
            }
        }
    }
}

fn decode_surrogate(second: u8, third: u8) -> u32 {
    0xD000u32 | ((second & CONT_MASK) as u32) << 6 | (third & CONT_MASK) as u32
}

fn decode_surrogates(second: u8, third: u8, fifth: u8, sixth: u8) -> [u8; 4] {
    let c1 = decode_surrogate(second, third);
    let c2 = decode_surrogate(fifth, sixth);
    let c = 0x10000 + (((c1 - 0xD800) << 10) | (c2 - 0xDC00));

    [
        0b1111_0000u8 | ((c & 0b1_1100_0000_0000_0000_0000) >> 18) as u8,
        TAG_CONT_U8 | ((c & 0b0_0011_1111_0000_0000_0000) >> 12) as u8,
        TAG_CONT_U8 | ((c & 0b0_0000_0000_1111_1100_0000) >> 6) as u8,
        TAG_CONT_U8 | (c & 0b0_0000_0000_0000_0011_1111) as u8,
    ]
}

impl ToJavaUtf8Ext for String {
    fn to_java_utf8(&self) -> Result<Vec<u8>, Error> {
        if is_valid_java_utf8(self.as_bytes()) {
            Ok(self.as_bytes().into())
        } else {
            let bytes = self.bytes();
            let mut encoded = Vec::with_capacity((bytes.len() + bytes.len()) >> 2);

            if let Some(err) = to_java_utf8(&mut encoded, &mut bytes.collect::<Vec<u8>>().iter()) {
                return Err(err);
            }

            Ok(encoded)
        }
    }
}

fn is_valid_java_utf8(text: &[u8]) -> bool {
    for b in text {
        if (*b & !CONT_MASK) == TAG_CONT_U8 {
            continue;
        }

        if utf8_char_width(*b) > 3 {
            return false;
        }
    }

    !text.contains(&0u8)
}

fn to_java_utf8(encoded: &mut Vec<u8>, iter: &mut slice::Iter<u8>) -> Option<Error> {
    loop {
        let b = match iter.next() {
            Some(&b) => b,
            None => return None,
        };

        if b == 0 {
            encoded.push(0xc0);
            encoded.push(0x80);
        } else if b < 128 {
            encoded.push(b);
        } else {
            let width = utf8_char_width(b);
            let next_bytes = match next_x_from_iter(width - 1, iter) {
                Ok(v) => v,
                Err(e) => return Some(e),
            };

            if width != 4 {
                encoded.push(b);
                encoded.extend(next_bytes.iter());
            } else {
                let str_bytes = {
                    let mut str_bytes = Vec::with_capacity(width);
                    str_bytes.push(b);
                    str_bytes.extend(next_bytes.iter());

                    str_bytes
                };

                let str = unsafe { String::from_utf8_unchecked(str_bytes) };

                let c = str.chars().next().unwrap() as u32 - 0x10000;

                let mut s = [0u16, 2];
                s[0] = ((c >> 10) as u16) | 0xD800;
                s[1] = ((c & 0x3FF) as u16) | 0xDC00;

                encoded.extend(encode_surrogate(s[0]).iter());
                encoded.extend(encode_surrogate(s[1]).iter());
            }
        }
    }
}

fn next_x_from_iter<T>(x: usize, iter: &mut slice::Iter<T>) -> Result<Vec<T>, Error>
where
    T: Clone,
{
    let mut items = Vec::with_capacity(x);

    for _ in 0..x {
        match iter.next() {
            Some(v) => items.push(v.clone()),
            None => return Err(Error::UtfConversionError(UtfConversionError::UnexpectedEndOfData)),
        }
    }

    Ok(items)
}

fn encode_surrogate(surrogate: u16) -> [u8; 3] {
    [
        0b11100010 | ((surrogate &  0b11110000_00000000) >> 12) as u8,
        TAG_CONT_U8 | ((surrogate & 0b00001111_11000000) >> 6) as u8,
        TAG_CONT_U8 | (surrogate &  0b00000000_00111111) as u8,
    ]
}
