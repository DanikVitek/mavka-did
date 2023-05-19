fn main() {
    let true_result = mavka_did::parser::parse("так");
    println!("{:#?}", true_result);
    let false_result = mavka_did::parser::parse("ні");
    println!("{:#?}", false_result);
    let number_result = mavka_did::parser::parse("123");
    println!("{:#?}", number_result);
    let number_result = mavka_did::parser::parse("-123");
    println!("{:#?}", number_result);
    let number_result = mavka_did::parser::parse("-123.45");
    println!("{:#?}", number_result);
    let text_result = mavka_did::parser::parse("\"Hello, world!\"");
    println!("{:#?}", text_result);
    let text_result = mavka_did::parser::parse("\"\n\""); // error
    println!("{:#?}", text_result);
}
