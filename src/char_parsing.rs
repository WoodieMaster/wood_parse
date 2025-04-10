use std::io::Read;

use anyhow::Result;

#[inline]
const fn utf8_first_byte(byte: u8, width: u32) -> u32 {
    (byte & (0x7F >> width)) as u32
}

fn read_byte(reader: &mut impl Read) -> Option<Result<u8>> {
    let mut buf = [0u8; 1];
    let l = match reader.read(&mut buf) {
        Ok(l) => l,
        Err(err) => return Some(Err(err.into())),
    };
    if l == 0 {
        return None;
    }
    Some(Ok(buf[0]))
}

const CONT_MASK: u8 = 0b0011_1111;

#[inline]
const fn utf8_acc_cont_byte(ch: u32, byte: u8) -> u32 {
    (ch << 6) | (byte & CONT_MASK) as u32
}

pub fn read_char(reader: &mut impl Read) -> Option<Result<char>> {
    let x = match read_byte(reader) {
        Some(Ok(x)) => x,
        Some(Err(err)) => return Some(Err(err)),
        None => return None,
    };
    if x < 128 {
        return Some(Ok(x as char));
    }

    let init = utf8_first_byte(x, 2);

    let y = match read_byte(reader) {
        Some(Ok(x)) => x,
        Some(Err(err)) => return Some(Err(err)),
        None => return None,
    };
    let mut ch = utf8_acc_cont_byte(init, y);
    if x >= 0xE0 {
        let z = match read_byte(reader) {
            Some(Ok(x)) => x,
            Some(Err(err)) => return Some(Err(err)),
            None => return None,
        };
        let y_z = utf8_acc_cont_byte((y & CONT_MASK) as u32, z);
        ch = init << 12 | y_z;
        if x >= 0xF0 {
            let w = match read_byte(reader) {
                Some(Ok(x)) => x,
                Some(Err(err)) => return Some(Err(err)),
                None => return None,
            };
            ch = (init & 7) << 18 | utf8_acc_cont_byte(y_z, w);
        }
    }

    char::from_u32(ch)
        .ok_or(anyhow::anyhow!("invalid utf-8"))
        .into()
}
