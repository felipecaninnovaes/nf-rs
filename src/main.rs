mod modules;
use modules::json::structs::objects::Produto;

fn main() {
    Produto::new("nf-xml-files/nfe-pessoa-juridica.xml");

    // println!("{:?}", input)
}