use anyhow::bail;
use num_bigint::BigInt;

pub fn decode(stream: impl Iterator<Item = u8>) -> anyhow::Result<BigInt> {
    let mut ret = BigInt::ZERO;
    for b in stream {
        match b {
            b'!'..=b'~' => {
                ret *= 94;
                ret += b - b'!';
            }
            _ => bail!("Unexpected char"),
        }
    }
    Ok(ret)
}

pub fn encode(mut value: BigInt) -> anyhow::Result<String> {
    // value < 0
    if value < BigInt::ZERO {
        bail!("Value must be non-negative");
    }

    let mut result = String::new();
    while value > BigInt::ZERO {
        let remainder = u8::try_from(value.clone() % 94)?;
        result.push((remainder + b'!') as char);
        value /= 94;
    }
    if result.is_empty() {
        result.push('!');
    }

    // 逆順にする必要があるため、結果を反転
    let result: String = result.chars().rev().collect();

    Ok(result)
}
