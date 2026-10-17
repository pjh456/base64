use anyhow::Result;
use clap::Args;

#[derive(Args)]
pub struct EncodeArgs {
    pub raw: String,
}

pub fn run(args: &EncodeArgs) -> Result<()> {
    let out = encode_impl(args.raw.as_bytes(), 0).unwrap_or_default();
    println!("{}", String::from_utf8_lossy(&out));
    Ok(())
}

pub fn encode_impl(raw: &[u8], wrap: usize) -> Result<Vec<u8>> {
    const TABLE: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

    let n = raw.len().div_ceil(3) * 4;
    let cap = if wrap > 0 { n + n / wrap + 1 } else { n };
    let mut out: Vec<u8> = Vec::with_capacity(cap);
    let mut count = 0;

    for chunk in raw.chunks(3) {
        let mut buf = [0u8; 3];
        buf[..chunk.len()].copy_from_slice(chunk);

        let u32n = u32::from_be_bytes([0, buf[0], buf[1], buf[2]]);

        let mask = 0b111111;
        let indices: [u32; 4] = std::array::from_fn(|i| u32n >> ((3 - i) * 6) & mask);

        let real = chunk.len() + 1;
        for (i, &idx) in indices.iter().enumerate() {
            if wrap > 0 && count == wrap {
                out.push(b'\n');
                count = 0;
            }
            out.push(if i < real { TABLE[idx as usize] } else { b'=' });
            count += 1;
        }
    }

    if wrap > 0 && count > 0 {
        out.push(b'\n');
    }

    Ok(out)
}
