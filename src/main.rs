use serde::Deserialize;
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Deserialize)]
struct AllWordsDictionary(HashMap<String, Vec<String>>);

fn main() {
    // JSON 文件路径
    let file_path = "all_words_dictionary.json";

    // 读取 JSON 文件
    let json_data = fs::read_to_string(file_path).expect("Failed to read the JSON file");

    // 反序列化 JSON 数据到 HashMap
    let dictionary: HashMap<String, Vec<String>> =
        serde_json::from_str(&json_data).expect("Failed to parse JSON");

    // 打印反序列化后的数据
    println!("Deserialized dictionary: {:?}", dictionary);

    // 示例：访问某个 key 的值
    if let Some(words) = dictionary.get("example_key") {
        println!("Words for 'example_key': {:?}", words);
    } else {
        println!("Key 'example_key' not found in the dictionary");
    }
}
