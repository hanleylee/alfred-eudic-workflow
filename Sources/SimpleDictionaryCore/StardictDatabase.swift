//
//  DatabaseManager.swift
//  alfred-eudic-workflow
//
//  Created by Hanley Lee on 2024/12/7.
//

import SQLite
import stdio_h

class StardictDatabase {
    private var db: Connection?
    private let stardict = Table("stardict")

    // Define table columns
    private let id = Expression<Int>("id")
    private let word = Expression<String>("word")
    private let sw = Expression<String>("sw")
    private let phonetic = Expression<String?>("phonetic")
    private let definition = Expression<String?>("definition")
    private let translation = Expression<String?>("translation")
    private let pos = Expression<String?>("pos")
    private let collins = Expression<Int?>("collins")
    private let oxford = Expression<Int?>("oxford")
    private let tag = Expression<String?>("tag")
    private let bnc = Expression<Int>("bnc")
    private let frq = Expression<Int>("frq")
    private let exchange = Expression<String?>("exchange")
    private let detail = Expression<String?>("detail")
    private let audio = Expression<String?>("audio")

    init(databasePath: String) {
        do {
            db = try Connection(databasePath)
            fputs("Connected to database at \(databasePath)\n", stderr)
        } catch {
            fputs("Unable to connect to database: \(error)\n", stderr)
        }
    }

    /// Fetch all words from the stardict table
    func fetchAllWords() -> [StardictEntry] {
        var results: [StardictEntry] = []

        do {
            for row in try db!.prepare(stardict) {
                let entry = StardictEntry(
                    id: row[id],
                    word: row[word],
                    sw: row[sw],
                    phonetic: row[phonetic],
                    definition: row[definition],
                    translation: row[translation],
                    pos: row[pos],
                    collins: row[collins],
                    oxford: row[oxford],
                    tag: row[tag],
                    bnc: row[bnc],
                    frq: row[frq],
                    exchange: row[exchange],
                    detail: row[detail],
                    audio: row[audio]
                )
                results.append(entry)
            }
        } catch {
            fputs("Failed to fetch words: \(error)\n", stderr)
        }

        return results
    }

    /// Search for a word in the stardict table
    func searchWord(_ spells: [String], limit: Int) -> [StardictEntry] {
        guard !spells.isEmpty else { return [] }
        var results: [Row] = []

        var searchQuery: Table = stardict.limit(limit)

        if spells.count == 1 { // only get items which begin with spell when spell doesn't contain space
            searchQuery = searchQuery.filter(sw.like("\(spells[0])%"))
        } else {
            spells.forEach { searchQuery = searchQuery.filter(sw.like("%\($0)%")) }
        }

        do {
            results += try db!.prepare(searchQuery)
        } catch {
            fputs("Failed to fetch prefix results: \(error)\n", stderr)
        }

        if results.count < limit, spells.count == 1 {
            let remainingCount = limit - results.count

            let finalQuery = stardict.filter(sw.like("%\(searchQuery)%")).limit(remainingCount)
            do {
                results += try db!.prepare(finalQuery)
            } catch {
                fputs("Failed to fetch prefix results: \(error)\n", stderr)
            }
        }

        var entries: [StardictEntry] = []
        for row in results {
            let entry = StardictEntry(
                id: row[id],
                word: row[word],
                sw: row[sw],
                phonetic: row[phonetic],
                definition: row[definition],
                translation: row[translation],
                pos: row[pos],
                collins: row[collins],
                oxford: row[oxford],
                tag: row[tag],
                bnc: row[bnc],
                frq: row[frq],
                exchange: row[exchange],
                detail: row[detail],
                audio: row[audio]
            )
            entries.append(entry)
        }

        return entries
    }
}
