use std::io;

fn main() -> io::Result<()> {
    println!("cargo:rerun-if-changed=../link.lds");
    println!("cargo::rustc-link-arg-bin=d945gclf_sdram=-T./link.lds");
    println!("cargo::rustc-link-arg-bin=d945gclf_sdram=-Map=d945gclf_sdram.map");
    println!("cargo::rustc-link-arg-bin=d945gclf_sdram=-ereset");

    Ok(())
}
