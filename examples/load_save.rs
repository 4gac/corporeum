use corporum::Corporeum;

fn main() {
    let corp = Corporeum::load(&"hello.corp.gz").unwrap();
    corp.save(false).unwrap();
}
