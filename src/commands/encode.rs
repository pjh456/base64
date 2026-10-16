use anyhow::Result;
use clap::Args;

#[derive(Args)]
pub struct EncodeArgs {
    pub raw: String,
}

pub fn run(args: EncodeArgs, _verbose: bool) -> Result<()> {
    println!("{}", encode_impl(&args.raw).unwrap_or("".to_string()));
    Ok(())
}

const TABLE: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

fn encode_impl(raw: &str) -> Result<String> {
    let bytes: &[u8] = raw.as_bytes();
    let mut out: String = String::with_capacity((raw.len() + 2) / 3 * 4);

    for chunk in bytes.chunks(3) {
        let mut buf = [0u8; 3];
        buf[..chunk.len()].copy_from_slice(chunk);

        let n = u32::from_be_bytes([0, buf[0], buf[1], buf[2]]);

        let mask = 0b111111;
        let indices = [18, 12, 6, 0].map(|shift| (n >> shift) & mask);

        let real = chunk.len() + 1;
        for (i, &idx) in indices.iter().enumerate() {
            out.push(if i < real {
                TABLE[idx as usize] as char
            } else {
                '='
            });
        }
    }

    Ok(out)
}
