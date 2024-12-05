// swift-tools-version: 5.10
// The swift-tools-version declares the minimum version of Swift required to build this package.

import PackageDescription

let isLocalDebug = true

let dependency_AlfredWorkflowUtils: Package.Dependency = isLocalDebug ?
    .package(path: "../alfred-workflow-utils") :
    .package(url: "https://github.com/hanleylee/alfred-workflow-utils.git", .upToNextMajor(from: "0.0.1"))

let package = Package(
    name: "alfred-eudic-workflow",
    platforms: [
        .macOS(.v13),
    ],
    products: [
        .executable(name: "alfred-eudic", targets: ["AlfredEudicCLI"]),
    ],
    dependencies: [
        .package(url: "https://github.com/apple/swift-argument-parser.git", .upToNextMajor(from: "1.5.0")),
        dependency_AlfredWorkflowUtils,
    ],
    targets: [
        .executableTarget(
            name: "AlfredEudicCLI",
            dependencies: [
                "SimpleDictionaryCore",
                .product(name: "AlfredWorkflowUpdater", package: "alfred-workflow-utils"),
                .product(name: "AlfredWorkflowScriptFilter", package: "alfred-workflow-utils"),
                .product(name: "ArgumentParser", package: "swift-argument-parser"),
            ],
            path: "Sources/AlfredEudicCLI"
        ),
        .target(
            name: "SimpleDictionaryCore",
            dependencies: [
                .product(name: "AlfredWorkflowUpdater", package: "alfred-workflow-utils"),
            ],
            path: "Sources/SimpleDictionaryCore",
            resources: [
                .process("Resources"),
            ]

        ),

        // MARK: - TEST -

        .testTarget(
            name: "AlfredEudicTests",
            dependencies: [
                "SimpleDictionaryCore",
            ],
            path: "Tests/AlfredEudicTests"
        ),
    ]
)
