#[cfg(windows)]
fn main() {
    // 只有在 Windows 上才执行
    thunk::thunk();  // 你的 thunk 代码
}

#[cfg(not(windows))]
fn main() {
    // 其他操作系统的处理
    println!("This is not Windows.");
}
