//! This is the build script for both tests7 and tests8.
//!
//! You should modify this file to make both exercises pass.
use std::time::SystemTime;
use std::env;
fn main() {
    // In tests7, we should set up an environment variable
    // called `TEST_FOO`. Print in the standard output to let
    // Cargo do it.
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs(); // What's the use of this timestamp here?
    // let your_command = format!(
    //     //"Your command here with {}, please checkout exercises/tests/build.rs",
    //
    //     timestamp
    // );
    // println!("cargo:{}", your_command);
    //
    // // In tests8, we should enable "pass" feature to make the
    // // testcase return early. Fill in the command to tell
    // // Cargo about that.
    // let your_command = "Your command here, please checkout exercises/tests/build.rs";
    // println!("cargo:{}", your_command);
    // 修正：直接使用 Cargo 专属格式设置 TEST_FOO 环境变量（移除错误的 format! 嵌套 println!）
    println!("cargo:rustc-env=TEST_FOO={}", timestamp);

    // In tests8, we should enable "pass" feature to make the
    // testcase return early. Fill in the command to tell
    // Cargo about that.
    // 启用 "pass" 特性的 Cargo 指令
    let your_command = "rustc-cfg=feature=\"pass\"";
    println!("cargo:{}", your_command);
}
