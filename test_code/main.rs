#[path = "things/hello.rs"]
mod things;

fn main() {
    let output = things::foo();
    return output;
}
