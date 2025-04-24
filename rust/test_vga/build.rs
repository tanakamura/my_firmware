#[path = "../build_common/build_common.rs"]
mod build_common;

fn main() {
    build_common::build(build_common::BuildType::BOOT);
    //build_common::build(build_common::BuildType::BINARY);
}
