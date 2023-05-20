fn main() {
    let input = r#"Людина(
  імʼя="Давид",
  прізвище="Когут",
  вік=0,
  параметри=(
    висота=175,
    вага=69
  ),
  зацікавлення=["творення", "життя"]
)"#;
    let result = mavka_did::parser::parse(input);
    println!("{:#?}", result);

    let input = "[1, -2, 3.14159264, 9875205987345098230897103895701839405130]";
    let result = mavka_did::parser::parse(input);
    println!("{:#?}", result);
}