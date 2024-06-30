use anyhow::bail;
use num_bigint::BigInt;
use num_traits::Euclid;

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
    let bar = indicatif::ProgressBar::new(value.bits());
    let divisor = BigInt::from(94);
    while value > BigInt::ZERO {
        bar.set_position(value.bits());
        let (quotient, reminder) = value.div_rem_euclid(&divisor);
        let remainder = u8::try_from(&reminder)?;
        result.push((remainder + b'!') as char);
        value = quotient;
    }
    if result.is_empty() {
        result.push('!');
    }

    // 逆順にする必要があるため、結果を反転
    let result: String = result.chars().rev().collect();

    Ok(result)
}
