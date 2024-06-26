// mod utils;

use wasm_bindgen::prelude::*;
use std::{fs, io};
// use std::path::Path;
// use serde_json;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn hello() {
    alert("Hello, rust-package!");
}

#[wasm_bindgen]
pub fn testAdd(a: usize, b: usize) -> usize {
    a + b
}


#[wasm_bindgen]
pub fn getDirList(path: &str) -> Vec<String> {
    let mut files: Vec<String> = Vec::new();

    // 读取目录
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries {
            if let Ok(entry) = entry {
                if let Some(file_name) = entry.file_name().to_str() {
                    files.push(file_name.to_string());
                }
            }
        }

        // 打印文件列表
        // for file in files {
        //     println!("{}", file);
        // }
    } else {
        eprintln!("Failed to read directory");
    }

    println!("{:?}", files);

    files

    // 目标目录路径
    // let directory_path = Path::new(path);
    
    // 确保目录存在
    // if !directory_path.exists() || !directory_path.is_dir() {
    //     println!("The specified directory does not exist or is not a directory.");
    //     return;
    // }

     // 读取并过滤出所有子目录名称
    // let entries = fs::read_dir(path);

    // print!("entries{}", entries)

    // let directories = entries.map(|entry| entry.map(|e| e.path()));
        // .map(|entries| {
        //     entries
        //         // .filter_map(|entry| entry.ok())
        //         // .filter(|entry| entry.path().is_dir())
        //         // .map(|entry| entry.file_name().into_string().unwrap())
        //         .collect()
        // });
        // .unwrap_or_else(|e| {
        //     println!("Error reading directory: {}", e);
        //     Vec::new()
        // });

    //    entriesc
    
    
     // 转换为JSON字符串
    // let json_string = serde_json::to_string(&directories).unwrap();
    
    // 输出JSON
    // println!("{}", json_string);
}

mod tests {
    use super::*;

    #[test]
    fn it_works() {
        getDirList("E:\\TapGameProject\\imba_template2\\editor\\table\\entry_data");
        let result: usize = testAdd(2, 2);
        assert_eq!(result, 4);
    }
}