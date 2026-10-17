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
    let bytes = raw;
    if !bytes.len().is_multiple_of(4) {
        bail!(format!("Invalid base64 length: {}", bytes.len()));
    }

    let mut out = Vec::with_capacity(raw.len() / 3 * 4);

    for chunk in bytes.chunks_exact(4) {
        let c0 = decode_char(chunk[0])
            .ok_or_else(|| anyhow::anyhow!("Invalid character: {:?}", chunk[0] as char))?;
        let c1 = decode_char(chunk[1])
            .ok_or_else(|| anyhow::anyhow!("Invalid character: {:?}", chunk[1] as char))?;

        let (c2, c3, real) = if chunk[2] == b'=' {
            if chunk[3] != b'=' {
                bail!("Invalid padding");
            }
            (0, 0, 1)
        } else if chunk[3] == b'=' {
            (
                decode_char(chunk[2])
                    .ok_or_else(|| anyhow::anyhow!("Invalid character: {:?}", chunk[2] as char))?,
                0,
                2,
            )
        } else {
            (
                decode_char(chunk[2])
                    .ok_or_else(|| anyhow::anyhow!("Invalid character: {:?}", chunk[2] as char))?,
                decode_char(chunk[3])
                    .ok_or_else(|| anyhow::anyhow!("Invalid character: {:?}", chunk[3] as char))?,
                3,
            )
        };

        let n: u32 = (c0 << 18) | (c1 << 12) | (c2 << 6) | c3;
        out.push((n >> 16) as u8);
        if real >= 2 {
            out.push((n >> 8) as u8);
        }
        if real >= 3 {
            out.push(n as u8);
        }
    }

    Ok(out)
}
