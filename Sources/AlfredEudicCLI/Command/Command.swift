//
//  Command.swift
//  alfred-eudic-workflow
//
//  Created by Hanley Lee on 2024/12/5.
//

import ArgumentParser
import SimpleDictionaryCore

struct Command: AsyncParsableCommand {
    @OptionGroup()
    var options: Options

    static let configuration = CommandConfiguration(
        commandName: "alfred-eudic",
        abstract: "Tool used to quickly search matched words by partial query",
        discussion: "",
        subcommands: [
            SearchCommand.self,
            UpdateCommand.self,
        ]
    )

    func run() async throws {
        print("Main command run!")
    }
}

extension Command {
    struct Options: ParsableArguments {
        // MARK: - Package Loading
    }
}
