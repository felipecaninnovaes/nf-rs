mod modules;
use modules::json::structs::produtos::Produto;

fn main() {
    let input = Produto::new("nf-xml-files/nfe-pessoa-fisica.xml");
    println!("{:?}", input)
}
