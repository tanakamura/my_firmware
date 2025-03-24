#[path = "../build_common/build_common.rs"]
mod build_common;

fn main() {
    build_common::build(build_common::BuildType::BOOT, "sdram_monitor");
}
