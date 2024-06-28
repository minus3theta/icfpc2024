use anyhow::bail;

pub fn decode(stream: impl Iterator<Item = u8>) -> anyhow::Result<i64> {
    let mut ret = 0;
    for b in stream {
        match b {
            b'!'..=b'~' => {
                ret *= 94;
                ret += (b - b'!') as i64;
            }
            _ => bail!("Unexpected char"),
        }
    }
    Ok(ret)
}
