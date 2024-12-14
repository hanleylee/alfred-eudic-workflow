import Foundation
import QuartzCore
import AlfredCore

public struct DictionaryConfig {
    public let completionFile: String
    public let dbFile: String?
    public init(completionFile: String, dbFile: String?) {
        self.completionFile = completionFile
        self.dbFile = dbFile
    }
}
public class DictionaryManager {

    private let config: DictionaryConfig
    private var database: StardictDatabase?
    
    public init(config: DictionaryConfig) {
        self.config = config
        if let dbFile = config.dbFile, FileManager.default.fileExists(atPath: dbFile) {
            self.database = StardictDatabase(databasePath: dbFile)
        }
    }

    // MARK: - Must be invoked before call any other function
    
    public func findMatchesInDB(spells: [String], limit: Int = 10) -> [StardictEntry] {
        AlfredUtils.log("database file: \(config.dbFile ?? "")")
        guard let database else { return [] }
        let res = database.searchWord(spells, limit: limit)
        return res
    }
    
    public func findMatchesInCompletion(spell: String, limit: Int = 10) async -> [String] {
        AlfredUtils.log("completion file: \(config.completionFile)")
        let completionFileURL = URL(filePath: config.completionFile)
        let completionData = try! Data(contentsOf: completionFileURL)
        let completionContent = String(data: completionData, encoding: .utf8)!
        let completionWords = await completionContent.splitConcurrently()

        guard let beginIndex = completionWords.binarySearchMatchPrefix(target: spell) else { return [] }

        let prefixLen = spell.count
        // 从找到的位置向后最多查找 `capacity` 个元素
        var result: [String] = []
        result.reserveCapacity(limit) // 提前分配空间

        for i in beginIndex ..< completionWords.endIndex {
            if completionWords[i].prefix(prefixLen) == spell {
                result.append(completionWords[i])
            }
            if result.count == limit {
                return result
            }
        }
        return result
    }
}

