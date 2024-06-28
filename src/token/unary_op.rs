#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum UnaryOp {
    // ~ Integer negation
    Neg,
    // \! Boolean not
    Not,
    // # string-to-int: interpret a string as a base-94 number
    ToInt,
    // $ int-to-string: inverse of the above
    ToString,
}

pub fn decode(stream: impl Iterator<Item = u8>) -> anyhow::Result<UnaryOp> {
    
}


// pub fn decode(stream: impl Iterator<Item = u8>) -> anyhow::Result<BinaryOp> {
//     for b in stream {
//         match b {
//             b'+' => BinaryOp::Add,
//             b'-' => BinaryOp::Sub,
//             b'*' => BinaryOp::Mul,
//             b'/' => BinaryOp::Div,
//             b'%' => BinaryOp::Mod,
//             b'<' => BinaryOp::Less,
//             b'>' => BinaryOp::Greater,
//             b'=' => BinaryOp::Equal,
//             b'|' => BinaryOp::Or,
//             b'&' => BinaryOp::And,
//             b'.' => BinaryOp::Concat,
//             b'T' => BinaryOp::Take,
//             b'D' => BinaryOp::Drop,
//             b'$' => BinaryOp::Apply,
//             _ => bail!("Unexpected char"),
//         }
//     }
//     todo!()
// }