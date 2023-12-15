mod modules;
use modules::json::structs::structs::objects::Produto;

fn main() {
    let input = Produto::new("nf-xml-files/nfe-pessoa-fisica.xml");
    
    println!("{:?}", input)
}