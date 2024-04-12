use nfse::modules::nfse::get_nfse;

fn main() {
    let nfse_layout_folder_path = "/home/felipecn/Desktop/PROJECTS/nf-rs/nfse/src/layouts";
    let nfse_json_path = "/home/felipecn/Desktop/PROJECTS/nf-rs/nfse/src/models/model_nfse_catanduva_01_normal_com_rps.xml";
    let result = get_nfse(nfse_layout_folder_path, nfse_json_path);
    println!("{}", result.expect("error getting nfse"));
}
