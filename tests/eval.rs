use icfpc2024::ast::{Expr, Value};
use rstest::rstest;

#[rstest]
#[case("U- I$", (-3).into())]
#[case("U! T", false.into())]
#[case("U# S4%34", 15818151.into())]
#[case("U$ I4%34", "test".to_owned().into())]
#[case("? B> I# I$ S9%3 S./", "no".to_owned().into())]
#[case("B$ B$ L# L$ v# B. SB%,,/ S}Q/2,$_ IK", "Hello World!".to_owned().into())]
#[case(r#"B$ L# B$ L" B+ v" v" B* I$ I# v8"#, 12.into())]
#[case(r#"B$ B$ L" B$ L# B$ v" B$ v# v# L# B$ v" B$ v# v# L" L# ? B= v# I! I" B$ L$ B+ B$ v" v$ B$ v" v$ B- v# I" I%"#, 16.into())]
fn eval(#[case] expr: &str, #[case] expected: Value) -> anyhow::Result<()> {
    let expr: Expr = expr.parse()?;
    assert_eq!(expr.eval(&vec![])?, expected);
    Ok(())
}
