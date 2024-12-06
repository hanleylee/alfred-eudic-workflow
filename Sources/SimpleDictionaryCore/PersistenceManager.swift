//
//  PersistenceManager.swift
//  alfred-eudic-workflow
//
//  Created by Hanley Lee on 2024/12/7.
//

import AlfredWorkflowUpdaterCore
import CryptoKit
import Foundation
import QuartzCore

public class PersistenceManager {
    public private(set) var allWords: [String] = []
    private let config: DictionaryConfig
    
    private init(config: DictionaryConfig) {
        self.config = config
        let completionFileURL = URL(filePath: config.completionFile)
        let completionData = try! Data(contentsOf: completionFileURL)
        let completionContent = String(data: completionData, encoding: .utf8)!
        let completionWords = completionContent.components(separatedBy: .newlines)
        
        allWords = completionWords
        
        if hasCache() {
            allWords = wordsFromCache()
        } else {
            allWords = makeWordsCache()
        }
//        let newData = try! Data(contentsOf: URL(filePath: cachePath()))
//        let newWords: [String] = convert_to_array(from: newData)
//        let t2 = CACurrentMediaTime()
//        print("config time duration: \(t2 - t1)")

        #if DEBUG
        
//        let data1 = convert_to_data(from: self.allWords)
//        let t1 = CACurrentMediaTime()
//        let newWors: [String] = convert_to_array(from: data1, capacity: allWords.count)
//        print(newWors.count)
//        let t2 = CACurrentMediaTime()
//        print("duration of split string: \(t2 - t1)")
        #endif
    }

    func cachePath() -> String {
        let cacheRoot = URL(fileURLWithPath: AlfredConst.workflowCache!)
        var cacheFilename = "completion_cache_\(calculateFileMD5(at: config.completionFile)!)"
        if let dbFile = config.dbFile {
            cacheFilename.append("_db_cache_\(calculateFileMD5(at: dbFile)!)")
        }
        return cacheRoot.appendingPathComponent(cacheFilename).path
    }
    
//    func cacheSizeKey() -> String {
//        return "SIZE_\(cachePath())"
//    }
//
    func hasCache() -> Bool {
        let cachePath = cachePath()
        return FileManager.default.fileExists(atPath: cachePath)
    }

    func makeWordsCache() -> [String] {
        let completionFileURL = URL(filePath: config.completionFile)
        let completionData = try! Data(contentsOf: completionFileURL)
        let completionContent = String(data: completionData, encoding: .utf8)!
        // FIXME: if array's string length greater than 15, it can't serialize to file
        let completionWords = completionContent.components(separatedBy: .newlines).map { String($0.prefix(16)) }
        let cachePath = cachePath()
        
        if let dbFile = config.dbFile {
            let database = StardictDatabase(databasePath: dbFile)
            let databaseAllWords = database.fetchAllWords().map { $0.sw }
            let allWords = (completionWords + databaseAllWords)
            let cacheData = convert_to_data(from: allWords)
//            let newWors: [String] = convert_to_array(from: cacheData, capacity: allWords.count)
            try! cacheData.write(to: URL(filePath: cachePath))
            return allWords
        } else {
            let cacheData = convert_to_data(from: completionWords)
            try! cacheData.write(to: URL(filePath: cachePath))
            return completionWords
        }
    }
    
    func wordsFromCache() -> [String] {
        let cachePath = cachePath()
        let cacheURL = URL(filePath: cachePath)
        let cacheData = try! Data(contentsOf: cacheURL)
//        let t1 = CACurrentMediaTime()
        let words: [String] = convert_to_array(from: cacheData)
//        let words = deserializeStringArray(from: cacheData)
        
//        let t2 = CACurrentMediaTime()
//        print("convert data to array: \(t2 - t1)")
        return words
    }

    /// 计算文件的 MD5 哈希值
    /// - Parameter filePath: 文件的路径
    /// - Returns: 文件的 MD5 值（十六进制字符串），如果文件不存在则返回 `nil`
    func calculateFileMD5(at filePath: String) -> String? {
        let fileURL = URL(fileURLWithPath: filePath)
        
        // 确保文件存在
        guard FileManager.default.fileExists(atPath: filePath) else {
            print("File does not exist at path: \(filePath)")
            return nil
        }
        
        do {
            // 读取文件数据
            let fileData = try Data(contentsOf: fileURL)
            
            // 使用 CryptoKit 计算 MD5 哈希值
            let hash = Insecure.MD5.hash(data: fileData)
            
            // 转换为十六进制字符串
            return hash.map { String(format: "%02hhx", $0) }.joined()
        } catch {
            print("Failed to read file or calculate MD5: \(error)")
            return nil
        }
    }
}

extension PersistenceManager {
    func convert_to_data<T>(from array: [T]) -> Data {
        var p: UnsafeBufferPointer<T>? = nil
        array.withUnsafeBufferPointer { p = $0 }
        guard p != nil else {
            return Data()
        }
        return Data(buffer: p!)
    }
    
    func convert_to_array<T>(from data: Data) -> [T] {
//        let array = data.withUnsafeBytes {
//                            (pointer: UnsafePointer<T>) -> [T] in
//            let buffer = UnsafeBufferPointer(start: pointer,
//                                             count: data.count/MemoryLayout<T>.size)
//            return Array<T>(buffer)
//        }
//        return array

        let capacity = data.count / MemoryLayout<T>.size
        let result = [T](unsafeUninitializedCapacity: capacity) { pointer, copied_count in
            let length_written = data.copyBytes(to: pointer)
            copied_count = length_written / MemoryLayout<T>.size
            assert(copied_count == capacity)
        }
        return result
    }
//
//    func serializeStringArray(_ array: [String]) -> Data {
//        var data = Data()
//
//        // 写入字符串数组的长度
//        var count = UInt32(array.count)
//        data.append(UnsafeBufferPointer(start: &count, count: 1))
//
//        // 写入每个字符串的长度和内容
//        for string in array {
//            guard let utf8Data = string.data(using: .utf8) else { continue }
//
//            var length = UInt32(utf8Data.count)
//            data.append(UnsafeBufferPointer(start: &length, count: 1))
//            data.append(utf8Data)
//        }
//
//        return data
//    }
//
    ////    func convert_to_array<T>(from data: Data) -> [T] {
    ////        let count = data.count / MemoryLayout<T>.size
    ////        var array = [T](repeating: T.init(), count: count)
    ////        data.copyBytes(to: UnsafeMutableBufferPointer(start: &array, count: count))
    ////        return array
    ////    }
//
    ////    func deserializeStringArray(from data: Data) -> [String] {
    ////        var result: [String] = []
    ////        var offset = 0
    ////
    ////        // 读取数组长度
    ////        let count: UInt32 = data.withUnsafeBytes {
    ////            $0.load(fromByteOffset: offset, as: UInt32.self)
    ////        }
    ////        offset += MemoryLayout<UInt32>.size
    ////
    ////        // 读取每个字符串
    ////        for _ in 0..<count {
    ////            // 读取字符串长度
    ////            let length: UInt32 = data.withUnsafeBytes {
    ////                $0.load(fromByteOffset: offset, as: UInt32.self)
    ////            }
    ////            offset += MemoryLayout<UInt32>.size
    ////
    ////            // 读取字符串内容
    ////            let stringData = data.subdata(in: offset..<offset + Int(length))
    ////            if let string = String(data: stringData, encoding: .utf8) {
    ////                result.append(string)
    ////            }
    ////            offset += Int(length)
    ////        }
    ////
    ////        return result
    ////    }
}
