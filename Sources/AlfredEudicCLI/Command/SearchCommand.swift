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
import QuartzCore
import SimpleDictionaryCore

struct SearchCommand: AsyncParsableCommand {
    static let configuration = CommandConfiguration(commandName: "search", abstract: "Perform a search query", discussion: "")

    @Option(help: ArgumentHelp("The file used for provide completeion item"))
    var completionFile: String = ProcessInfo.processInfo.environment["ALFRED_EUDIC_COMPLETION_FILE"] ?? ""

    @Option(help: ArgumentHelp("The database file used for provide explaination"))
    var dbFile: String? = ProcessInfo.processInfo.environment["ALFRED_EUDIC_DATABASE_FILE"]

    @Argument(help: "Spell of the word you want to query")
    var spell: String = "are"

    func validate() throws {}

    func run() async throws {
        guard spell.count > 1 else {
            ScriptFilter.item(Item(title: "Input more than one letter"))
            print(ScriptFilter.output())
            return
        }

        let t1 = CACurrentMediaTime()
        let config = DictionaryConfig(
            completionFile: completionFile,
            dbFile: dbFile
        )
        let dictionaryManager = DictionaryManager(config: config)

        var items: [Item] = []

        if config.dbFile == nil || config.dbFile!.isEmpty { // query completion list
            let matches = await dictionaryManager.findMatchesInCompletion(spell: spell, limit: 30)

            for word in matches {
                items.append(
                    Item(title: word)
                        .arg(word)
                )
            }
        } else { // query database
            let matches = dictionaryManager.findMatchesInDB(spells: spell.split(separator: " ").map { String($0) }, limit: 30)
            for entry in matches {
                items.append(
                    Item(title: entry.word)
                        .subtitle(entry.translation ?? entry.definition ?? "")
                        .arg(entry.word)
                        .mods(
                            Cmd().subtitle(entry.exchangeInfo ?? ""),
                            Alt().subtitle(entry.phonetic ?? "")
                        )
                )
            }
        }
        if items.isEmpty || items.first?.title.lowercased() != spell.lowercased() {
            items.insert(contentsOf: [Item(title: spell).arg(spell).subtitle("Type enter to check in Eudic")], at: 0)
        }
        items.forEach { ScriptFilter.item($0) }

        let t2 = CACurrentMediaTime()
        fputs("search time duration: \(t2 - t1)\n", stderr)

        print(ScriptFilter.output())
    }
}

extension SearchCommand {}
