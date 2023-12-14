mod modules;
use modules::json::structs::structs::objects::Produto;

fn main() {
    Produto::new("nf-xml-files/nfe-pessoa-juridica.xml");

    // println!("{:?}", input)
}