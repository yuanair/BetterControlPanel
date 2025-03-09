use ndk_glue::*;
use log::{info, error};

// Vulkan 相关（示例）
use ash::{vk, Entry, Instance};

// 定义 android_main 入口函数
#[no_mangle]
pub extern "C" fn android_main(app: ndk_glue::AndroidApp) {
    // 初始化日志
    android_logger::init_once(
        android_logger::Config::default().with_tag("RustVulkan")
    );
    info!("Rust android_main started!");

    // 处理 Android 事件循环
    let mut event_loop = app.event_loop();
}