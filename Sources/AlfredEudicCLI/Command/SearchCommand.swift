//
//  SearchCommand.swift
//  alfred-eudic-workflow
//
//  Created by Hanley Lee on 2024/12/5.
//

import AlfredWorkflowScriptFilter
import AlfredWorkflowUpdaterCore
import ArgumentParser
import Foundation
import SimpleDictionaryCore

extension Array {
    func findMatching(capacity: Int, where condition: (Element) -> Bool) -> [Element] {
        var result: [Element] = []
        result.reserveCapacity(capacity) // 提前分配容量，优化性能

        for element in self {
            if condition(element) {
                result.append(element)
                if result.count == capacity {
                    break // 找到前五个后立即停止
                }
            }
        }

        return result
    }
}

struct SearchCommand: AsyncParsableCommand {
    static let configuration = CommandConfiguration(commandName: "search", abstract: "Perform a search query", discussion: "")
    @Argument(help: "spell of the word you want to query")
    var spell: String = "he"

    func run() async throws {
//        print(WordsManager.shared.allWordsDictionary.count)
//        print(WordsManager.shared.allWords.count)
//        let matched = WordsManager.shared.allWords.findMatching(capacity: 10) { word in
//            print(word)
//            print(spell)
//            return word.hasPrefix(spell)
//        }
//        print(matched)
        let matched = WordsManager.shared.allWordsDictionary[String(spell.prefix(2))]?.findMatching(capacity: 10, where: { word in
//            print(word)
//            print(spell)
            return word.hasPrefix(spell)
        })
        print(matched)
//        loadWordDictionary()
    }
}

extension SearchCommand {}
