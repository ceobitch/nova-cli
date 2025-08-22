import SwiftUI
import Foundation

struct SimpleTerminalView: NSViewRepresentable {
    let apiKey: String
    let shouldStart: Bool

    func makeNSView(context: Context) -> NSView {
        let view = SimpleTerminalViewWrapper()
        view.configure(apiKey: apiKey, shouldStart: shouldStart)
        return view
    }

    func updateNSView(_ nsView: NSView, context: Context) {
        guard let v = nsView as? SimpleTerminalViewWrapper else { return }
        v.configure(apiKey: apiKey, shouldStart: shouldStart)
    }
}

class SimpleTerminalViewWrapper: NSView {
    private var scrollView: NSScrollView!
    private var textView: NSTextView!
    private var process: Process?
    private var pty: PseudoTerminal?
    private var currentCols: UInt16 = 120
    private var currentRows: UInt16 = 30
    private var sawOutput = false
    private var isRunningNova = false
    private var configuredApiKey: String = ""
    private var hasConfiguredOnce = false

    override init(frame frameRect: NSRect) {
        super.init(frame: frameRect)
        setupUI()
    }

    required init?(coder: NSCoder) {
        super.init(coder: coder)
        setupUI()
    }

    private func setupUI() {
        wantsLayer = true
        layer?.backgroundColor = NSColor.clear.cgColor

        // Glassy background
        let blur = NSVisualEffectView(frame: bounds)
        blur.translatesAutoresizingMaskIntoConstraints = false
        blur.material = .hudWindow
        blur.blendingMode = .behindWindow
        blur.state = .active
        addSubview(blur)

        scrollView = NSScrollView(frame: bounds)
        scrollView.translatesAutoresizingMaskIntoConstraints = false
        scrollView.hasVerticalScroller = true
        scrollView.hasHorizontalScroller = false
        addSubview(scrollView)

        textView = NSTextView(frame: bounds)
        textView.isEditable = false
        textView.isRichText = false
        textView.font = .monospacedSystemFont(ofSize: 12, weight: .regular)
        textView.drawsBackground = false
        textView.backgroundColor = .clear
        textView.textColor = NSColor.white.withAlphaComponent(0.92)
        textView.insertionPointColor = .white
        textView.string = ""

        scrollView.documentView = textView

        NSLayoutConstraint.activate([
            blur.topAnchor.constraint(equalTo: topAnchor),
            blur.leadingAnchor.constraint(equalTo: leadingAnchor),
            blur.trailingAnchor.constraint(equalTo: trailingAnchor),
            blur.bottomAnchor.constraint(equalTo: bottomAnchor),
            scrollView.topAnchor.constraint(equalTo: topAnchor),
            scrollView.leadingAnchor.constraint(equalTo: leadingAnchor),
            scrollView.trailingAnchor.constraint(equalTo: trailingAnchor),
            scrollView.bottomAnchor.constraint(equalTo: bottomAnchor),
        ])
    }

    func configure(apiKey: String, shouldStart: Bool) {
        configuredApiKey = apiKey
        if shouldStart && !hasConfiguredOnce {
            hasConfiguredOnce = true
            startNovaProcess()
        }
    }

    override var acceptsFirstResponder: Bool { true }
    override func becomeFirstResponder() -> Bool { true }

    override func keyDown(with event: NSEvent) {
        // Esc key quits the app
        if event.keyCode == 53 || event.characters == "\u{1b}" { // 53 = Esc
            NSApp.terminate(nil)
            return
        }
        guard let chars = event.characters, let data = chars.data(using: .utf8) else { return }
        pty?.writeToChild(data: data)
    }

    private func append(_ text: String) {
        let visible = stripANSI(text)
        if let textStorage = textView.textStorage {
            let attr = NSAttributedString(string: visible)
            textStorage.append(attr)
            textView.scrollToEndOfDocument(nil)
        }
    }

    private func stripANSI(_ s: String) -> String {
        // Conservative control-char filter (keeps newline and tab). Avoid Unicode.Properties APIs.
        var out = String()
        out.reserveCapacity(s.count)
        for ch in s.unicodeScalars {
            let v = ch.value
            // allow LF (10) and TAB (9)
            if v == 10 || v == 9 {
                out.unicodeScalars.append(ch)
                continue
            }
            // drop C0 controls (0-31) and DEL (127)
            if (0...31).contains(v) || v == 127 {
                continue
            }
            out.unicodeScalars.append(ch)
        }
        return out
    }

    private func startNovaProcess() {
        // Resolve helper path inside the app bundle
        let helperPath = (Bundle.main.bundlePath as NSString).appendingPathComponent("Contents/Helpers/nova")
        if !FileManager.default.isExecutableFile(atPath: helperPath) {
            append("Error: Could not find helper in app bundle at \(helperPath)\n")
            append("Hint: Ensure the helper is placed at Contents/Helpers/nova and is executable.\n")
            // Smoke test anyway to prove PTY works
            runCommand(path: "/usr/bin/env", args: ["echo", "Nova PTY OK (no helper found)"]) { [weak self] _ in
                // After smoke test, quit since helper is missing
                DispatchQueue.main.asyncAfter(deadline: .now() + 0.3) {
                    NSApp.terminate(nil)
                }
            }
            return
        }

        // First: smoke test the PTY with a known-good command
        runCommand(path: "/usr/bin/env", args: ["echo", "Nova PTY OK"]) { [weak self] _ in
            // After a short delay, swap to running Nova
            DispatchQueue.main.asyncAfter(deadline: .now() + 0.2) {
                self?.runNova(at: helperPath)
            }
        }
    }

    private func runNova(at path: String) {
        isRunningNova = true
        append("Launching Nova...\n")
        var args: [String] = ["--debug"]
        // If no API key is set, rely on Codex's built-in onboarding/login flows.
        runCommand(path: path, args: args) { _ in
            // If Nova exits, quit the app
            DispatchQueue.main.async {
                NSApp.terminate(nil)
            }
        }
    }

    private func runCommand(path: String, args: [String], onExit: @escaping (Int32) -> Void) {
        do {
            if pty == nil {
                let term = try PseudoTerminal()
                self.pty = term
                updateTerminalSize()
                startReadingFromPTY()
            }

            let proc = Process()
            proc.executableURL = URL(fileURLWithPath: path)
            proc.arguments = args
            var env = ProcessInfo.processInfo.environment
            if env["TERM"] == nil { env["TERM"] = "xterm-256color" }
            if env["LANG"] == nil { env["LANG"] = "en_US.UTF-8" }
            if !configuredApiKey.isEmpty { env["OPENAI_API_KEY"] = configuredApiKey }
            proc.environment = env
            proc.standardInput = pty?.slaveFileHandle
            proc.standardOutput = pty?.slaveFileHandle
            proc.standardError = pty?.slaveFileHandle
            proc.terminationHandler = { p in
                onExit(p.terminationStatus)
            }
            try proc.run()
            self.process = proc
        } catch {
            append("Failed to launch command: \(path) — \(error.localizedDescription)\n")
        }
    }

    private func startReadingFromPTY() {
        guard let master = pty?.masterFileHandle else { return }
        master.readabilityHandler = { [weak self] handle in
            let data = handle.availableData
            if !data.isEmpty {
                // Lossy UTF‑8 decoding so partial/ANSI bytes are not dropped
                let s = String(decoding: data, as: UTF8.self)
                DispatchQueue.main.async {
                    self?.sawOutput = true
                    self?.append(s)
                }
            }
        }
    }

    override func viewDidMoveToWindow() {
        super.viewDidMoveToWindow()
        window?.makeFirstResponder(self)
    }

    override func layout() {
        super.layout()
        updateTerminalSize()
    }

    private func updateTerminalSize() {
        // Approximate character cell size based on current font
        let font = textView?.font ?? .monospacedSystemFont(ofSize: 12, weight: .regular)
        let charSize = "W".size(withAttributes: [.font: font])
        let cellWidth = max(charSize.width, 7)
        // Approximate line height from font metrics
        let approxLineHeight = (font.ascender - font.descender + font.leading)
        let cellHeight = max(CGFloat(approxLineHeight), 14)
        let cols = UInt16(max(20, Int(floor(bounds.width / cellWidth))))
        let rows = UInt16(max(8, Int(floor(bounds.height / cellHeight))))
        if cols != currentCols || rows != currentRows {
            currentCols = cols
            currentRows = rows
            pty?.resize(width: cols, height: rows)
        }
    }

    deinit {
        process?.terminate()
        pty?.masterFileHandle.readabilityHandler = nil
    }
}

// MARK: - PseudoTerminal Implementation
class PseudoTerminal {
    let masterFileHandle: FileHandle
    let slaveFileHandle: FileHandle
    private var masterFD: Int32
    private var slaveFD: Int32

    init() throws {
        var mfd: Int32 = -1
        var sfd: Int32 = -1

        mfd = posix_openpt(O_RDWR)
        if mfd == -1 { throw PTYError.failedToOpenMaster }
        if grantpt(mfd) == -1 { close(mfd); throw PTYError.failedToGrantAccess }
        if unlockpt(mfd) == -1 { close(mfd); throw PTYError.failedToUnlock }
        guard let slaveName = ptsname(mfd) else { close(mfd); throw PTYError.failedToGetSlaveName }
        sfd = open(slaveName, O_RDWR)
        if sfd == -1 { close(mfd); throw PTYError.failedToOpenSlave }

        masterFD = mfd
        slaveFD = sfd
        masterFileHandle = FileHandle(fileDescriptor: mfd)
        slaveFileHandle = FileHandle(fileDescriptor: sfd)

        configureTerminal()
    }

    private func configureTerminal() {
        var t = termios()
        tcgetattr(slaveFD, &t)
        t.c_lflag &= ~UInt(ECHO | ICANON | ISIG)
        t.c_iflag &= ~UInt(IXON | IXOFF)
        t.c_oflag &= ~UInt(OPOST)
        tcsetattr(slaveFD, TCSANOW, &t)
    }

    func resize(width: UInt16, height: UInt16) {
        var ws = winsize(ws_row: height, ws_col: width, ws_xpixel: 0, ws_ypixel: 0)
        ioctl(slaveFD, TIOCSWINSZ, &ws)
    }

    func writeToChild(data: Data) {
        do { try masterFileHandle.write(contentsOf: data) } catch { /* ignore */ }
    }

    deinit {
        masterFileHandle.closeFile()
        slaveFileHandle.closeFile()
        close(masterFD)
        close(slaveFD)
    }
}

enum PTYError: Error { case failedToOpenMaster, failedToGrantAccess, failedToUnlock, failedToGetSlaveName, failedToOpenSlave }


