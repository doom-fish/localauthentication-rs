// swift-tools-version:5.9
import PackageDescription

let package = Package(
    name: "LocalAuthenticationBridge",
    platforms: [
        .macOS(.v13)
    ],
    products: [
        .library(
            name: "LocalAuthenticationBridge",
            type: .static,
            targets: ["LocalAuthenticationBridge"])
    ],
    targets: [
        .target(
            name: "LocalAuthenticationBridge",
            path: "Sources/LocalAuthenticationBridge",
            publicHeadersPath: "include")
    ]
)
