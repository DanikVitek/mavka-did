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
}