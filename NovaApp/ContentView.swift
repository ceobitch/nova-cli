import SwiftUI

struct ContentView: View {
    @State private var apiKey: String = UserDefaults.standard.string(forKey: "OPENAI_API_KEY") ?? ""
    @State private var showApiKeySheet: Bool = false
    @State private var shouldStartTerminal: Bool = false

    var body: some View {
        ZStack(alignment: .top) {
            // Background glass gradient for the whole window
            LinearGradient(gradient: Gradient(colors: [Color.black.opacity(0.75), Color.black.opacity(0.55)]), startPoint: .topLeading, endPoint: .bottomTrailing)
                .ignoresSafeArea()

            VStack(spacing: 0) {
                // Custom glass title bar with traffic lights
                HStack(spacing: 12) {
                    HStack(spacing: 8) {
                        Circle()
                            .fill(Color(red: 0.99, green: 0.27, blue: 0.22))
                            .frame(width: 12, height: 12)
                            .onTapGesture { NSApp.terminate(nil) }
                        Circle()
                            .fill(Color(red: 1.00, green: 0.78, blue: 0.16))
                            .frame(width: 12, height: 12)
                            .onTapGesture { NSApp.windows.first?.performMiniaturize(nil) }
                        Circle()
                            .fill(Color(red: 0.20, green: 0.84, blue: 0.25))
                            .frame(width: 12, height: 12)
                            .onTapGesture { NSApp.windows.first?.performZoom(nil) }
                    }

                    Text("Nova - AI Cybersecurity Companion")
                        .font(.system(size: 13, weight: .semibold))
                        .foregroundColor(Color.white.opacity(0.85))
                        .padding(.leading, 4)

                    Spacer()

                    Button("Settings") { showApiKeySheet = true }
                        .buttonStyle(.plain)
                        .foregroundColor(Color.white.opacity(0.7))
                        .padding(.trailing, 8)
                }
                .padding(.horizontal, 12)
                .padding(.vertical, 10)
                .background(
                    Color.black.opacity(0.25)
                        .blur(radius: 12)
                        .overlay(
                            Rectangle()
                                .fill(Color.white.opacity(0.08))
                                .frame(height: 1), alignment: .bottom
                        )
                )

                // Terminal view starts only after API key captured (or user continues without one)
                SimpleTerminalView(apiKey: apiKey, shouldStart: shouldStartTerminal)
                    .background(Color.clear)
            }
        }
        .onAppear {
            if apiKey.isEmpty {
                showApiKeySheet = true
            } else {
                shouldStartTerminal = true
            }
        }
        .sheet(isPresented: $showApiKeySheet, onDismiss: {
            // If user closed sheet without saving, we still allow starting without key
            if !shouldStartTerminal { shouldStartTerminal = true }
        }) {
            ApiKeySheet(apiKey: $apiKey, onSave: { key in
                UserDefaults.standard.set(key, forKey: "OPENAI_API_KEY")
                apiKey = key
                shouldStartTerminal = true
                showApiKeySheet = false
            }, onContinueWithoutKey: {
                shouldStartTerminal = true
                showApiKeySheet = false
            })
        }
        .preferredColorScheme(.dark)
    }
}

struct ApiKeySheet: View {
    @Binding var apiKey: String
    var onSave: (String) -> Void
    var onContinueWithoutKey: () -> Void

    var body: some View {
        VStack(alignment: .leading, spacing: 16) {
            Text("Connect to OpenAI")
                .font(.title3).bold()
            Text("Enter your OpenAI API key to enable Nova's AI features. You can add this later in Settings.")
                .foregroundColor(.secondary)

            SecureField("sk-...", text: $apiKey)
                .textFieldStyle(.roundedBorder)
                .frame(minWidth: 420)

            HStack {
                Button("Save & Start") { onSave(apiKey) }
                    .keyboardShortcut(.defaultAction)
                Button("Continue without key") { onContinueWithoutKey() }
                    .buttonStyle(.plain)
                Spacer()
                Link("Get an API key", destination: URL(string: "https://platform.openai.com/account/api-keys")!)
                    .foregroundColor(.blue)
            }
            .padding(.top, 4)
        }
        .padding(24)
    }
}

#Preview {
    ContentView()
}
