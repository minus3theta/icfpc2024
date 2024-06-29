use icfpc2024::ast::{Expr, Value};
use rstest::rstest;

#[rstest]
#[case::negation("U- I$", (-3).into())]
#[case::not("U! T", false.into())]
#[case::stoi("U# S4%34", 15818151.into())]
#[case::itos("U$ I4%34", "test".to_owned().into())]
#[case::add("B+ I# I$", 5.into())]
#[case::sub("B- I$ I#", 1.into())]
#[case::mul("B* I$ I#", 6.into())]
#[case::div("B/ U- I( I#", (-3).into())]
#[case::modulo("B% U- I( I#", (-1).into())]
#[case::less("B< I$ I#", false.into())]
#[case::greater("B> I$ I#", true.into())]
#[case::eq("B= I$ I#", false.into())]
#[case::or("B| T F", true.into())]
#[case::and("B& T F", false.into())]
#[case::concat("B. S4% S34", "test".to_owned().into())]
#[case::take("BT I$ S4%34", "tes".to_owned().into())]
#[case::drop("BD I$ S4%34", "t".to_owned().into())]
#[case::if_ex("? B> I# I$ S9%3 S./", "no".to_owned().into())]
#[case::lambda_ex("B$ B$ L# L$ v# B. SB%,,/ S}Q/2,$_ IK", "Hello World!".to_owned().into())]
#[case::eval_ex(r#"B$ L# B$ L" B+ v" v" B* I$ I# v8"#, 12.into())]
#[case::limits_ex(r#"B$ B$ L" B$ L# B$ v" B$ v# v# L# B$ v" B$ v# v# L" L# ? B= v# I! I" B$ L$ B+ B$ v" v$ B$ v" v$ B- v# I" I%"#, 16.into())]
fn eval(#[case] expr: &str, #[case] expected: Value) -> anyhow::Result<()> {
    let expr: Expr = expr.parse()?;
    assert_eq!(expr.eval()?, expected);
    Ok(())
}
