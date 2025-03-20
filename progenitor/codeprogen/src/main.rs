use progenitor::generate_api;

fn main() {
    let src = "/Users/polyprogrammist/work/near/openapirpc/testokplain/transaction.json";
    println!("cargo:rerun-if-changed={}", src);
    let file = std::fs::File::open(src).unwrap();
    let spec = serde_json::from_reader(file).unwrap();
    let mut generator = progenitor::Generator::default();

    let tokens = generator.generate_tokens(&spec).unwrap();
    // generate_api!("/Users/polyprogrammist/work/near/openapirpc/testokplain/transaction.json");
}
