use cc::Build;
use std::io;

fn main() -> io::Result<()> {
    println!("cargo:rerun-if-changed=asm");
    println!("cargo:rerun-if-changed=../link.lds");

    Build::new()
        .file("asm/init.s")
        .file("asm/cache.s")
        .file("asm/raminit.s")
        .compile("asminit");

    Ok(())
}
