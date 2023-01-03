use async_std::task;

fn main() {
    task::block_on(async {
        println!("Hello, world!");
    })
}
