fn main() {
    let (tx, join_handle, instance) =
        better_control_panel::ipc::startup(better_control_panel::app_id!()).unwrap();
    println!("程序启动成功，单例运行中...");
    for message in tx {
        println!("收到来自其他程序的消息：{}", message);
    }
}
