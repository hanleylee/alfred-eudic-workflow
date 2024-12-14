//
//  SearchCommand.swift
//  alfred-eudic-workflow
//
//  Created by Hanley Lee on 2024/12/5.
//

import AlfredCore
import AlfredWorkflowScriptFilter
import AlfredWorkflowUpdaterCore
import AppKit
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

    let limit: Int = 30

    func validate() throws {}

    func run() async throws {
        guard spell.count > 1 else {
            ScriptFilter.item(Item(title: "Input more than one letter"))
            AlfredUtils.output(ScriptFilter.output())
            return
        }

        let t1 = CACurrentMediaTime()
        let config = DictionaryConfig(
            completionFile: completionFile,
            dbFile: dbFile
        )
        let dictionaryManager = DictionaryManager(config: config)

        var items: [Item] = []

        items.insert(Item(title: spell).arg(spell).subtitle("Type enter to check in Eudic"), at: 0)

        if let dbFile = config.dbFile, !dbFile.isEmpty { // query database
            if FileManager.default.fileExists(atPath: dbFile) {
                let matches = dictionaryManager.findMatchesInDB(spells: spell.split(separator: " ").map { String($0) }, limit: limit)
                for entry in matches {
                    let explainations = (entry.translation ?? entry.definition)?.components(separatedBy: .newlines).joined(separator: "; ") ?? ""
                    let phonetic = entry.phonetic ?? ""
                    let collinsRate = String(repeating: "⭐️", count: entry.collins ?? 0)
                    var importanceInfo: [String] = []
                    if let collins = entry.collins {
                        importanceInfo.append("COLLINS: \(collinsRate)")
                    }
                    if let _ = entry.oxford {
                        importanceInfo.append("OXFORD 3000")
                    }
                    if let bnc = entry.bnc, bnc != 0 {
                        importanceInfo.append("BNC: \(bnc)")
                    }
                    if let frq = entry.frq, frq != 0 {
                        importanceInfo.append("COCA: \(frq)")
                    }
                    if let tagInfo = entry.tagInfo {
                        importanceInfo.append(tagInfo)
                    }
                    let title = WorkflowUtils.alignedText(left: entry.word, right: "\(collinsRate)", component: .title)
                    let subtitle = WorkflowUtils.alignedText(left: explainations, right: "\(phonetic)", component: .subtitle)
                    items.append(
                        Item(title: title)
                            .subtitle(subtitle)
                            .arg(entry.word)
                            .mods(
                                Cmd().subtitle(entry.exchangeInfo ?? ""),
                                Alt().subtitle(importanceInfo.joined(separator: "; "))
                            )
                    )
                }
            } else {
                items.append(Item(title: "dbFile not exist: \(dbFile)"))
            }
        } else { // query completion list
            let matches = await dictionaryManager.findMatchesInCompletion(spell: spell, limit: limit)

            for word in matches {
                items.append(
                    Item(title: word)
                        .arg(word)
                )
            }
        }
//        if items.isEmpty || !items.contains(where: { $0.title.lowercased() == spell.lowercased() }) {
//        }
        items.forEach { ScriptFilter.item($0) }

        let t2 = CACurrentMediaTime()
        AlfredUtils.log("search time duration: \(t2 - t1)")

        let updater = Updater(githubRepo: CommonTools.githubRepo, workflowAssetName: CommonTools.workflowAssetName, checkInterval: 60*60*24)

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

        AlfredUtils.output(ScriptFilter.output())

        if !updater.cacheValid() {
            AlfredUtils.log("cache invalid")
            checkForUpdateSilently()
        }
    }
}

extension SearchCommand {
    func checkForUpdateSilently() {
        do {
            let process = Process()
            let executablePath = CommandLine.arguments[0]
            // child process will exit if parent process exited, so we must use nohup to execute external command
            process.executableURL = URL(fileURLWithPath: "/usr/bin/nohup")
            process.arguments = [executablePath, "update", "--action", "check"]

            process.standardOutput = FileHandle(forWritingAtPath: "/dev/null")
            process.standardError = FileHandle(forWritingAtPath: "/dev/null")

            try process.run()
//            process.waitUntilExit()
            AlfredUtils.log("Update check completed in the background")
        } catch {
            AlfredUtils.log("Failed to start update process: \(error)")
        }
    }
}
