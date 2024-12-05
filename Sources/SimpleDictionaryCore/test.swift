import Foundation

public struct WordsManager {
    public static let shared: WordsManager = .init()
    public var allWordsDictionary: [String: [String]] = [:]
    public var allWords: [String.SubSequence] = []
    private init() {
        let bundle = Bundle.module
        guard let jsonURL = bundle.url(forResource: "all_words_dictionary", withExtension: "json") else { fatalError("word dictionary file not found!") }
        guard let contentData = try? Data(contentsOf: jsonURL) else { fatalError() }
        guard let dic = try? JSONDecoder().decode([String: [String]].self, from: contentData) else { fatalError() }

        allWordsDictionary = dic
//        dic.forEach { allWords += $0.value }
        
//        let allWordsTxt = bundle.url(forResource: "words_alpha", withExtension: "txt")!
//        let txtData = try! Data(contentsOf: allWordsTxt)
//        let txtContent = String(data: txtData, encoding: .ascii)!
//        allWords = txtContent.split(separator: "\r\n")
    }
}
