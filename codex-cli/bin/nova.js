#!/usr/bin/env node
// Unified entry point for the Nova Shield CLI.

import path from "path";
import { fileURLToPath } from "url";

// __dirname equivalent in ESM
const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

const { platform, arch } = process;

let targetTriple = null;
switch (platform) {
  case "linux":
  case "android":
    switch (arch) {
      case "x64":
        targetTriple = "x86_64-unknown-linux-musl";
        break;
      case "arm64":
        targetTriple = "aarch64-unknown-linux-musl";
        break;
      default:
        break;
    }
    break;
  case "darwin":
    switch (arch) {
      case "x64":
        targetTriple = "x86_64-apple-darwin";
        break;
      case "arm64":
        targetTriple = "aarch64-apple-darwin";
        break;
      default:
        break;
    }
    break;
  case "win32":
    switch (arch) {
      case "x64":
        targetTriple = "x86_64-pc-windows-msvc.exe";
        break;
      case "arm64":
      // We do not build this today, fall through...
      default:
        break;
    }
    break;
  default:
    break;
}

if (!targetTriple) {
  throw new Error(`Unsupported platform: ${platform} (${arch})`);
}

// If a system `codex` binary is available, delegate to it so `nova`
// matches Codex behavior/TUI exactly.
import { spawn as nodeSpawn, spawnSync as nodeSpawnSync } from "child_process";

function trySpawnSync(cmd, args) {
  try {
    return nodeSpawnSync(cmd, args, { stdio: "ignore" });
  } catch {
    return { error: true };
  }
}

const codexExists = (() => {
  const probe = trySpawnSync(process.platform === "win32" ? "codex.exe" : "codex", ["--version"]);
  return !probe?.error && typeof probe.status === "number";
})();

function needsChatCompletionsFallback(argv) {
  const base = process.env.OPENAI_BASE_URL || "";
  if (!base.trim()) return false;
  // If user already specified a model_provider via -c, do not override.
  const hasExplicitProvider = argv.some((a, i) => {
    if (a === "-c" || a === "--config") {
      const nxt = argv[i + 1] || "";
      return /(^|\.)model_provider\s*=/.test(nxt);
    }
    // Allow inline -cmodel_provider=... forms just in case
    if (a.startsWith("-c") || a.startsWith("--config")) {
      return /(^|\.)model_provider\s*=/.test(a);
    }
    return false;
  });
  if (hasExplicitProvider) return false;

  // Consider anything not on an OpenAI domain as a likely proxy that
  // only supports Chat Completions.
  try {
    const u = new URL(base);
    const host = (u.host || "").toLowerCase();
    const isOpenAI = host.endsWith("openai.com") || host.endsWith("openai.org");
    return !isOpenAI;
  } catch {
    // If not a valid URL, err on the side of not forcing a fallback.
    return false;
  }
}

// Handle uninstall command
if (process.argv.includes("--uninstall")) {
  await uninstallNova();
  process.exit(0);
}

const forwardedArgv = (() => {
  const argv = process.argv.slice(2);
  if (needsChatCompletionsFallback(argv)) {
    // Add a config override to force Chat Completions for proxies/Azure
    // that do not support the Responses API.
    return ["-c", "model_provider=\"openai-chat-completions\"", ...argv];
  }
  return argv;
})();

if (codexExists) {
  const codexCmd = process.platform === "win32" ? "codex.exe" : "codex";
  const child = nodeSpawn(codexCmd, forwardedArgv, {
    stdio: "inherit",
    env: { ...process.env, CODEX_MANAGED_BY_NPM: "1" },
  });
  child.on("exit", (code, signal) => {
    if (signal) {
      process.kill(process.pid, signal);
    } else {
      process.exit(code ?? 1);
    }
  });
  // Do not run the rest of this file when delegating to `codex`.
  // Early return.
  // eslint-disable-next-line no-process-exit
  await new Promise(() => {});
}

// Prefer local dev binary when present or when NOVA_DEV_BIN is set.
let binaryPath = process.env.NOVA_DEV_BIN;

// Attempt to auto-build the local Rust workspace when available so `nova`
// always reflects current repo changes. This is a no-op if the workspace
// is not present or NOVA_SKIP_BUILD is set.
const workspaceRoot = path.resolve(__dirname, "..", "..");
const fs = await import("fs");
const codexRsDir = path.join(workspaceRoot, "codex-rs");
const cargoTomlPath = path.join(codexRsDir, "Cargo.toml");
const shouldAutoBuild = !process.env.NOVA_SKIP_BUILD && fs.existsSync(cargoTomlPath);

if (shouldAutoBuild) {
  const runCargoBuild = async () => {
    const { spawn } = await import("child_process");
    const buildArgs = process.env.NOVA_RELEASE_BUILD
      ? ["build", "--release", "-p", "codex-tui"]
      : ["build", "-p", "codex-tui"];
    await new Promise((resolve, reject) => {
      const proc = spawn("cargo", buildArgs, {
        cwd: codexRsDir,
        stdio: "inherit",
        env: process.env,
      });
      proc.on("error", reject);
      proc.on("exit", (code) => (code === 0 ? resolve() : reject(new Error(`cargo build failed with code ${code}`))));
    });
  };
  try {
    await runCargoBuild();
  } catch (err) {
    // eslint-disable-next-line no-console
    console.error(`[Nova] Warning: auto-build failed; will try prebuilt binary.`, err?.message || err);
  }
}

if (!binaryPath) {
  // Try common dev build locations (debug first for faster iteration)
  const debugPath = path.join(
    workspaceRoot,
    "codex-rs",
    "target",
    "debug",
    process.platform === "win32" ? "codex-tui.exe" : "codex-tui"
  );
  const releasePath = path.join(
    workspaceRoot,
    "codex-rs",
    "target",
    "release",
    process.platform === "win32" ? "codex-tui.exe" : "codex-tui"
  );

  if (fs.existsSync(debugPath)) {
    binaryPath = debugPath;
  } else if (fs.existsSync(releasePath)) {
    binaryPath = releasePath;
  }
}

// Fallback to packaged prebuilt if no dev binary found
if (!binaryPath) {
  binaryPath = path.join(__dirname, `nova-${targetTriple}`);
}

// Use an asynchronous spawn instead of spawnSync so that Node is able to
// respond to signals (e.g. Ctrl-C / SIGINT) while the native binary is
// executing. This allows us to forward those signals to the child process
// and guarantees that when either the child terminates or the parent
// receives a fatal signal, both processes exit in a predictable manner.
const { spawn } = await import("child_process");

async function tryImport(moduleName) {
  try {
    // eslint-disable-next-line node/no-unsupported-features/es-syntax
    return await import(moduleName);
  } catch (err) {
    return null;
  }
}

async function resolveRgDir() {
  const ripgrep = await tryImport("@vscode/ripgrep");
  if (!ripgrep?.rgPath) {
    return null;
  }
  return path.dirname(ripgrep.rgPath);
}

function getUpdatedPath(newDirs) {
  const pathSep = process.platform === "win32" ? ";" : ":";
  const existingPath = process.env.PATH || "";
  const updatedPath = [
    ...newDirs,
    ...existingPath.split(pathSep).filter(Boolean),
  ].join(pathSep);
  return updatedPath;
}

const additionalDirs = [];
const rgDir = await resolveRgDir();
if (rgDir) {
  additionalDirs.push(rgDir);
}
const updatedPath = getUpdatedPath(additionalDirs);

const child = spawn(binaryPath, forwardedArgv, {
  stdio: "inherit",
  // Align with what codex-tui expects for update messaging, etc.
  env: { ...process.env, PATH: updatedPath, CODEX_MANAGED_BY_NPM: "1" },
});

child.on("error", (err) => {
  // Typically triggered when the binary is missing or not executable.
  // Re-throwing here will terminate the parent with a non-zero exit code
  // while still printing a helpful stack trace.
  // eslint-disable-next-line no-console
  console.error(err);
  process.exit(1);
});

// Forward common termination signals to the child so that it shuts down
// gracefully. In the handler we temporarily disable the default behavior of
// exiting immediately; once the child has been signaled we simply wait for
// its exit event which will in turn terminate the parent (see below).
const forwardSignal = (signal) => {
  if (child.killed) {
    return;
  }
  try {
    child.kill(signal);
  } catch {
    /* ignore */
  }
};

["SIGINT", "SIGTERM", "SIGHUP"].forEach((sig) => {
  process.on(sig, () => forwardSignal(sig));
});

// When the child exits, mirror its termination reason in the parent so that
// shell scripts and other tooling observe the correct exit status.
// Wrap the lifetime of the child process in a Promise so that we can await
// its termination in a structured way. The Promise resolves with an object
// describing how the child exited: either via exit code or due to a signal.
const childResult = await new Promise((resolve) => {
  child.on("exit", (code, signal) => {
    if (signal) {
      resolve({ type: "signal", signal });
    } else {
      resolve({ type: "code", exitCode: code ?? 1 });
    }
  });
});

if (childResult.type === "signal") {
  // Re-emit the same signal so that the parent terminates with the expected
  // semantics (this also sets the correct exit code of 128 + n).
  process.kill(process.pid, childResult.signal);
} else {
  process.exit(childResult.exitCode);
}

// Uninstall function for nova --uninstall
async function uninstallNova() {
  const { spawn: nodeSpawn, spawnSync } = await import("child_process");
  const fs = await import("fs");
  const os = await import("os");
  
  console.log("🧹 Uninstalling Nova CLI...");
  
  try {
    // Stop running processes
    console.log("Stopping running processes...");
    spawnSync("pkill", ["-f", "nova|codex"], { stdio: "ignore" });
    
    // Remove npm package
    console.log("Removing npm packages...");
    spawnSync("npm", ["uninstall", "-g", "nova-cli"], { stdio: "ignore" });
    
    // Remove binaries
    console.log("Removing binaries...");
    const binaryPaths = [
      "/usr/local/bin/nova",
      "/opt/homebrew/bin/nova",
      "/opt/homebrew/bin/nova-cli", 
      "/opt/homebrew/bin/novacli",
      "/usr/bin/nova",
      "/usr/bin/codex"
    ];
    
    for (const binPath of binaryPaths) {
      try {
        if (fs.existsSync(binPath)) {
          spawnSync("sudo", ["rm", "-f", binPath], { stdio: "inherit" });
        }
      } catch (e) {
        // Ignore errors for individual file removal
      }
    }
    
    // Remove config directories
    console.log("Removing configuration directories...");
    const configDirs = [
      path.join(os.homedir(), ".codex"),
      path.join(os.homedir(), ".nova")
    ];
    
    for (const configDir of configDirs) {
      try {
        if (fs.existsSync(configDir)) {
          fs.rmSync(configDir, { recursive: true, force: true });
        }
      } catch (e) {
        // Ignore errors for individual directory removal
      }
    }
    
    // Clean shell configuration
    console.log("Cleaning shell configuration...");
    const shellFiles = [
      path.join(os.homedir(), ".zshrc"),
      path.join(os.homedir(), ".bashrc"), 
      path.join(os.homedir(), ".bash_profile")
    ];
    
    for (const shellFile of shellFiles) {
      try {
        if (fs.existsSync(shellFile)) {
          const content = fs.readFileSync(shellFile, "utf8");
          const cleanedContent = content
            .split("\n")
            .filter(line => !line.toLowerCase().includes("nova") && !line.toLowerCase().includes("codex"))
            .join("\n");
          fs.writeFileSync(shellFile, cleanedContent);
        }
      } catch (e) {
        // Ignore errors for shell config cleanup
      }
    }
    
    console.log("✅ Nova CLI uninstalled successfully!");
    console.log("Please restart your terminal or run 'source ~/.zshrc' to update your PATH.");
    
  } catch (error) {
    console.error("❌ Error during uninstall:", error.message);
    process.exit(1);
  }
}
