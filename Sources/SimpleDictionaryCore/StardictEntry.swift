//
//  Untitled.swift
//  alfred-eudic-workflow
//
//  Created by Hanley Lee on 2024/12/7.
//

/// A structure to represent a single row in the `stardict` table
public struct StardictEntry {
    public let id: Int
    public let word: String
    public let sw: String
    public let phonetic: String?
    public let definition: String?
    public let translation: String?
    public let pos: String?
    public let collins: Int?
    public let oxford: Int?
    public let tag: String?
    public let bnc: Int
    public let frq: Int
    public let exchange: String?
    public let detail: String?
    public let audio: String?
}

extension StardictEntry {
    /// parse exchange field, the format like `d:perceived/p:perceived/3:perceives/i:perceiving`
    public var exchangeInfo: String? {
        guard let exchange else { return nil }
        var infos: [String] = []
        let pairs = exchange.split(separator: "/")
        for pair in pairs {
            let keyValue = pair.split(separator: ":")
            let (key, value) = (keyValue[0], keyValue[1])
            switch key {
            case "p": infos.append("过去式: \(value)")
            case "d": infos.append("过去分词: \(value)")
            case "i": infos.append("现在分词: \(value)")
            case "3": infos.append("第三人称单数: \(value)")
            case "r": infos.append("形容词比较级: \(value)")
            case "t": infos.append("形容词最高级: \(value)")
            case "s": infos.append("名词复数形式: \(value)")
            case "0": infos.append("lemma: \(value)")
            case "1": infos.append("lemma transform: \(value)")
            default: break
            }
        }
        return infos.joined(separator: "; ")
    }
}
