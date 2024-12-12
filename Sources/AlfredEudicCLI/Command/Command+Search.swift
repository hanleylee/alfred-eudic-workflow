//
//  SearchCommand.swift
//  alfred-eudic-workflow
//
//  Created by Hanley Lee on 2024/12/5.
//

import AlfredCore
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

        if let dbFile = config.dbFile, FileManager.default.fileExists(atPath: dbFile) { // query database
            let matches = dictionaryManager.findMatchesInDB(spells: spell.split(separator: " ").map { String($0) }, limit: 30)
            for entry in matches {
                let explainations = (entry.translation ?? entry.definition)?.components(separatedBy: .newlines).joined(separator: "; ")
                items.append(
                    Item(title: entry.word)
                        .subtitle(explainations ?? "")
                        .arg(entry.word)
                        .mods(
                            Cmd().subtitle(entry.exchangeInfo ?? ""),
                            Alt().subtitle(entry.phonetic ?? "")
                        )
                )
            }
        } else { // query completion list
            let matches = await dictionaryManager.findMatchesInCompletion(spell: spell, limit: 30)

            for word in matches {
                items.append(
                    Item(title: word)
                        .arg(word)
                )
            }
        }
        if items.isEmpty || !items.contains(where: { $0.title.lowercased() == spell.lowercased() }) {
            items.insert(contentsOf: [Item(title: spell).arg(spell).subtitle("Type enter to check in Eudic")], at: 0)
        }
        items.forEach { ScriptFilter.item($0) }

        let t2 = CACurrentMediaTime()
        fputs("search time duration: \(t2 - t1)\n", stderr)

        let updater = Updater(githubRepo: CommonTools.githubRepo, workflowAssetName: CommonTools.workflowAssetName)

        if let release = updater.latestReleaseInfo, let currentVersion = AlfredConst.workflowVersion {
            if currentVersion.compare(release.tagName, options: .numeric) == .orderedAscending {
                ScriptFilter.item(
                    Item(title: "New version available on GitHub, type [Enter] to update")
                        .subtitle("current version: \(currentVersion), remote version: \(release.tagName)")
                        .arg("update")
                        .variable(.init(name: "HAS_UPDATE", value: "1"))
                )
            }
        }
        Task {
            try await updater.check(maxCacheAge: 1440)
        }

        print(ScriptFilter.output())
    }
}

extension SearchCommand {}
