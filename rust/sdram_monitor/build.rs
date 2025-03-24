use std::io;

fn main() -> io::Result<()> {
    println!("cargo:rerun-if-changed=../sdram.lds");
    println!("cargo::rustc-link-arg-bin=sdram_monitor=-T./sdram.lds");
    println!("cargo::rustc-link-arg-bin=sdram_monitor=-Map=sdram_monitor.map");
    println!("cargo::rustc-link-arg-bin=sdram_monitor=-e_start");

    Ok(())
}
