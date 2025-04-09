use utils::resource_manager::{check_resource, download_resource};

fn main() {
    if !check_resource("happy-tree.png") {
        download_resource("https://sotrh.github.io/learn-wgpu/assets/img/happy-tree.bdff8a19.png", "happy-tree.png").expect("Failed to download resource");
    }
    println!("Hello, world!");
}
