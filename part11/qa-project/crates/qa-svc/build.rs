use std::ffi::OsStr;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 推荐下面的方式生成grpc rust代码
    // 1.读取proto目录下的*.proto
    let proto_dir: PathBuf = "../../proto".into(); // proto文件所在目录
    let mut file_list = Vec::new(); // 存放proto文件名
    let lists = proto_dir.read_dir().expect("read proto dir failed");
    for entry_path in lists {
        if entry_path.as_ref().unwrap().path().is_file() {
            file_list.push(entry_path.unwrap().path())
        }
    }

    let out_dir = Path::new("../pb/src"); // 存放grpc rust代码生成的目录

    // let _ = fs::remove_dir_all(out_dir); // 删除原来的pb目录，可以根据实际情况打开注释
    let _ = fs::create_dir_all(out_dir); // 创建目录

    // grpc reflection 描述信息这是一个二进制文件，主要用于协议描述
    let descriptor_path = Path::new("../qa-svc").join("rpc_descriptor.bin");

    // 2.生成rust grpc代码
    // 指定rust grpc 代码生成的目录
    tonic_build::configure()
        .file_descriptor_set_path(&descriptor_path)
        .out_dir(out_dir)
        .compile(&file_list, &[proto_dir])?;

    // 3.生成mod.rs文件
    // 用下面的rust方式生成mod.rs
    // 拓展名是proto的文件名写入mod.rs中，作为pub mod xxx;导出模块
    let ext: Option<&OsStr> = Some(&OsStr::new("proto"));
    let mut mod_file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open(out_dir.join("lib.rs"))
        .expect("create lib.rs failed");
    // 先清空crates/pb/src/lib.rs文件内容
    mod_file.set_len(0).expect("failed to clear lib.rs");

    let header = String::from("// @generated by tonic-build.Do not edit it!!!\n");
    let _ = mod_file.write(header.as_bytes());
    for file in &file_list.iter().next() {
        if file.extension().eq(&ext) {
            if let Some(file) = file.file_name() {
                let f = file.to_str().unwrap();
                let filename = f.replace(".proto", "");
                println!("current filename: {}", f);
                let _ = mod_file.write(format!("pub mod {};\n", filename).as_bytes());

                // 实现message serde encode/decode
                let filename = out_dir.join(f.replace(".proto", ".rs"));
                let mut buffer = fs::read_to_string(&filename).unwrap();
                buffer = buffer.replace(
                    "prost::Message",
                    "prost::Message, serde::Serialize, serde::Deserialize",
                );
                fs::write(&filename, buffer).expect("write file content failed");
            }
        }
    }

    Ok(())
}
