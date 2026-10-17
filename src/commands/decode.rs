use anyhow::{Result, bail};
use clap::Args;

#[derive(Args)]
pub struct DecodeArgs {
    pub raw: String,
}

pub fn run(args: DecodeArgs) -> Result<()> {
    let bytes = decode_impl(args.raw.as_bytes(), false)?;
    let text = String::from_utf8(bytes)?;
    println!("{}", text);
    Ok(())
}

fn decode_char(ch: u8) -> Option<u32> {
    match ch {
        b'A'..=b'Z' => Some((ch - b'A') as u32),
        b'a'..=b'z' => Some((ch - b'a' + 26) as u32),
        b'0'..=b'9' => Some((ch - b'0' + 52) as u32),
        b'+' => Some(62),
        b'/' => Some(63),
        _ => None,
    }
}

pub fn decode_impl(raw: &[u8], ignore_garbage: bool) -> Result<Vec<u8>> {
    let mut out = Vec::with_capacity(raw.len() / 3 * 4);
    let mut bits: u32 = 0;
    let mut nbits: u32 = 0;
    let mut pads: u32 = 0;

    for &b in raw {
        if b == b'\n' {
            continue;
        }
        if b == b'=' {
            pads += 1;
            if pads > 2 {
                bail!("Too many pads")
            }
            continue;
        }

        match decode_char(b) {
            Some(v) => {
                if pads > 0 {
                    bail!("\'{}\' after \'=\'", b)
                }
                bits = (bits << 6) | v;
                nbits += 6;
                if nbits == 24 {
                    out.push((bits >> 16) as u8);
                    out.push((bits >> 8) as u8);
                    out.push(bits as u8);
                    bits = 0;
                    nbits = 0;
                }
            }
            None => {
                if !ignore_garbage {
                    bail!("Garbage character");
                }
            }
        }
    }

    match (nbits, pads) {
        (0, 0) => {}
        (12, 0) | (12, 2) => {
            if bits & 0b1111 != 0 {
                bail!("Still some bits left");
            }
            out.push((bits >> 4) as u8);
        }
        (18, 0) | (18, 1) => {
            if bits & 0b11 != 0 {
                bail!("Still some bits left");
            }
            out.push((bits >> 10) as u8);
            out.push((bits >> 2) as u8);
        }
        _ => {
            bail!("Invalid variant (nbits={}, pads={})", nbits, pads);
        }
    }

    Ok(out)
}
