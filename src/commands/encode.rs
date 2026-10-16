use anyhow::Result;
use clap::Args;

#[derive(Args)]
pub struct EncodeArgs {
    pub raw: String,
}

pub fn run(args: EncodeArgs, _verbose: bool) -> Result<()> {
    println!("Encoding String: {}", args.raw);
    println!("{}", encode_impl(&args.raw).unwrap_or("".to_string()));
    Ok(())
}

const TABLE: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

fn encode_impl(raw: &str) -> Result<String> {
    let bytes: &[u8] = raw.as_bytes();
    let mut out: String = String::new();

    for chunk in bytes.chunks(3) {
        let b0 = chunk[0] as u32;
        let b1 = *chunk.get(1).unwrap_or(&0) as u32;
        let b2 = *chunk.get(2).unwrap_or(&0) as u32;

        let n = (b0 << 16) | (b1 << 8) | b2;

        let mask = 0b111111;
        let i0 = (n >> 18) & mask;
        let i1 = (n >> 12) & mask;
        let i2 = (n >> 6) & mask;
        let i3 = n & mask;

        out.push(TABLE[i0 as usize] as char);
        out.push(TABLE[i1 as usize] as char);
        if chunk.len() > 1 {
            out.push(TABLE[i2 as usize] as char);
        } else {
            out.push('=');
        }
        if chunk.len() > 2 {
            out.push(TABLE[i3 as usize] as char);
        } else {
            out.push('=');
        }
    }

    Ok(out)
}
