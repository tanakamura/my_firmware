use std::io;

fn main() -> io::Result<()> {
    println!("cargo:rerun-if-changed=./sdram.lds");
    println!("cargo::rustc-link-arg-bin=sdram_helloworld=-T./sdram.lds");
    println !(
        "cargo::rustc-link-arg-bin=sdram_helloworld=-Map=sdram_helloworld.map");
    println!("cargo::rustc-link-arg-bin=sdram_helloworld=-e_start");

    Ok(())
}
