mod modules;
use modules::json::structs::produtos::Produto;

use crate::modules::json::structs::nfe::Nfe;

fn main() {
    let input = Nfe::new("/home/felipecn/PROJECTS/nf-rs/nfe/nf-xml-files-examples/nfe-pessoa-fisica.xml");
    println!("{:?}", input)
}
