#[allow(dead_code)]
pub enum BuildType {
    BINARY,
    BOOT,
}

pub fn build(build_type: BuildType) {
    let name = std::env::var("CARGO_PKG_NAME").unwrap();
    println!("cargo::rustc-link-arg-bin={}=-Map={}.map", name, name);
    let lds = match build_type {
        BuildType::BINARY => {
            println!("cargo::rustc-link-arg-bin={}=-e_start", name);
            "sdram.lds"
        }
        BuildType::BOOT => {
            println!("cargo::rustc-link-arg-bin={}=-ereset", name);
            "link.lds"
        }
    };

    println!("cargo:rerun-if-changed=../{}", lds);
    println!("cargo::rustc-link-arg-bin={}=-T./{}", name, lds);
}
