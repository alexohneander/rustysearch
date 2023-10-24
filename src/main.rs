use rustysearch::search::Rustysearch;

fn main() {
    println!("Hello, world!");
    let search = Rustysearch::new("/tmp/rustysearch");
    search.setup();
}