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

pub fn encode(mut value: i64) -> anyhow::Result<String> {
    if value < 0 {
        bail!("Value must be non-negative");
    }

    let mut result = String::new();
    while value > 0 {
        let remainder = (value % 94) as u8;
        result.push((remainder + b'!') as char);
        value /= 94;
    }

    // 逆順にする必要があるため、結果を反転
    let result: String = result.chars().rev().collect();

    Ok(result)
}
