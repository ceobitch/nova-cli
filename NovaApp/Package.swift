// swift-tools-version: 5.9
// The swift-tools-version declares the minimum version of Swift required to build this package.

import PackageDescription

let package = Package(
    name: "NovaApp",
    platforms: [
        .macOS(.v13)
    ],
    products: [
        .executable(
            name: "NovaApp",
            targets: ["NovaApp"]
        )
    ],
    dependencies: [
        .package(url: "https://github.com/migueldeicaza/SwiftTerm.git", from: "1.0.0")
    ],
    targets: [
        .executableTarget(
            name: "NovaApp",
            dependencies: [
                "SwiftTerm"
            ],
            path: ".",
            sources: ["NovaApp.swift", "ContentView.swift", "TerminalView.swift"]
        )
    ]
)

