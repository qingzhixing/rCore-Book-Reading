fn main() {
    // 显示当前目录下的文件名
    let entries = std::fs::read_dir(".").unwrap();
    entries.for_each(|entry| {
        let entry = entry.unwrap();
        println!("{}", entry.file_name().to_string_lossy());
    });
}
