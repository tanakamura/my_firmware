#[allow(dead_code)]
pub enum BuildType {
    BINARY,
    BOOT,
}

pub fn build(build_type: BuildType, name: &str) {
    match build_type {
        BuildType::BINARY => {
            println!("cargo:rerun-if-changed=../sdram.lds");
            println!("cargo::rustc-link-arg-bin={}=-T./sdram.lds", name);
            println!("cargo::rustc-link-arg-bin={}=-Map={}.map", name, name);
            println!("cargo::rustc-link-arg-bin={}=-e_start", name);
        }
        BuildType::BOOT => {
            println!("cargo:rerun-if-changed=../link.lds");
            println!("cargo::rustc-link-arg-bin={}=-T./link.lds", name);
            println!("cargo::rustc-link-arg-bin={}=-Map={}.map", name, name);
            println!("cargo::rustc-link-arg-bin={}=-ereset", name);
        }
    }
}
