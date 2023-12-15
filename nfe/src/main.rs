mod modules;
use modules::json::structs::produtos::Produto;

fn main() {
    let input = Produto::new("/home/felipecn/PROJECTS/nf-rs/nfe/nf-xml-files-examples/nfe-pessoa-fisica.xml");
    println!("{:?}", input)
}
