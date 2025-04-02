use std::io::Read;

use anyhow::{Error, Result};

use crate::util::END;

#[inline]
const fn utf8_first_byte(byte: u8, width: u32) -> u32 {
    (byte & (0x7F >> width)) as u32
}

fn read_byte(reader: &mut impl Read) -> Result<u8> {
    let mut buf = [0u8; 1];
    let l = reader.read(&mut buf)?;
    if l == 0 {
        return Err(Error::from(END()));
    }
    Ok(buf[0])
}

const CONT_MASK: u8 = 0b0011_1111;

#[inline]
const fn utf8_acc_cont_byte(ch: u32, byte: u8) -> u32 {
    (ch << 6) | (byte & CONT_MASK) as u32
}

pub fn read_char(reader: &mut impl Read) -> Result<char> {
    let x = read_byte(reader)?;
    if x < 128 {
        return Ok(x as char);
    }

    let init = utf8_first_byte(x, 2);

    let y = read_byte(reader)?;
    let mut ch = utf8_acc_cont_byte(init, y);
    if x >= 0xE0 {
        let z = read_byte(reader)?;
        let y_z = utf8_acc_cont_byte((y & CONT_MASK) as u32, z);
        ch = init << 12 | y_z;
        if x >= 0xF0 {
            let w = read_byte(reader)?;
            ch = (init & 7) << 18 | utf8_acc_cont_byte(y_z, w);
        }
    }

    char::from_u32(ch).ok_or(anyhow::anyhow!("invalid utf-8"))
}
