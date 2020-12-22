#[cfg(feature = "assets")]
mod assets;
#[cfg(feature = "assets")]
fn main() {
    assets::main();
}

#[cfg(feature = "memory")]
mod memory;
#[cfg(feature = "memory")]
fn main() {
    memory::main();
}
