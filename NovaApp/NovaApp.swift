import SwiftUI
// import Sparkle

@main
struct NovaApp: App {
    @NSApplicationDelegateAdaptor(AppDelegate.self) var appDelegate
    
    var body: some Scene {
        WindowGroup {
            ContentView()
                .frame(minWidth: 960, minHeight: 640)
        }
        .windowStyle(.hiddenTitleBar)
        .commands {
            CommandGroup(replacing: .newItem) { }
            CommandGroup(replacing: .appInfo) {
                Button("About Nova") {
                    NSApp.orderFrontStandardAboutPanel()
                }
            }
            CommandGroup(replacing: .systemServices) { }
            CommandGroup(replacing: .windowSize) { }
            CommandGroup(replacing: .windowArrangement) { }
        }
    }
}

class AppDelegate: NSObject, NSApplicationDelegate {
    // var updater: SPUUpdater?
    
    func applicationDidFinishLaunching(_ notification: Notification) {
        // Sparkle auto-updates disabled for local builds
        
        // Set up the app icon and name
        NSApp.setActivationPolicy(.regular)
        NSApp.activate(ignoringOtherApps: true)
        
        // Create the main window
        if let window = NSApp.windows.first {
            window.title = "Nova - AI Cybersecurity Companion"
            window.isMovableByWindowBackground = true
            window.isOpaque = false
            window.backgroundColor = NSColor.clear
            window.titleVisibility = .hidden
            window.titlebarAppearsTransparent = true
            window.isReleasedWhenClosed = false
            window.standardWindowButton(.closeButton)?.isHidden = true
            window.standardWindowButton(.miniaturizeButton)?.isHidden = true
            window.standardWindowButton(.zoomButton)?.isHidden = true
            // Rounded corners for a modern look
            window.contentView?.wantsLayer = true
            window.contentView?.layer?.cornerRadius = 12
            window.contentView?.layer?.masksToBounds = true
        }
    }
    
    func applicationShouldTerminateAfterLastWindowClosed(_ sender: NSApplication) -> Bool {
        return true
    }
    
    func applicationWillTerminate(_ notification: Notification) {
        // Clean up any resources
    }
}

