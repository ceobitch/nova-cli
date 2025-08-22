const tauri = window.__TAURI__;
const statusEl = document.getElementById('status');
const container = document.getElementById('term');

// Initialize xterm
const term = new window.Terminal({
  cursorBlink: true,
  convertEol: true,
  fontFamily: 'ui-monospace, Menlo, Monaco, SFMono-Regular',
  fontSize: 14,
  theme: {
    background: '#00000000',
    foreground: '#E6E6E6'
  }
});
const fitAddon = new window.FitAddon.FitAddon();
term.loadAddon(fitAddon);
term.open(container);
fitAddon.fit();

function write(s){ term.write(s); }

let startedNova = false;

async function start() {
  const platform = await tauri.os.platform();
  await tauri.event.listen('pty-data', (e) => write(e.payload));
  await tauri.event.listen('pty-exit', () => { if (startedNova) { tauri.window.getCurrent().close(); } });
  try {
    await tauri.core.invoke('pty_start', { command: platform === 'windows' ? 'cmd' : '/usr/bin/env', args: platform === 'windows' ? ['/c','echo','pty ok'] : ['echo','pty ok'], cols: term.cols, rows: term.rows });
  } catch (e) {
    write(`\r\nERR: Smoke test failed: ${String(e)}\r\n`);
  }
  try {
    write('\r\nLaunching Nova...\r\n');
    await tauri.core.invoke('pty_start', { command: './bin/nova', args: [], cols: term.cols, rows: term.rows });
    startedNova = true;
    statusEl.textContent = 'Running';
  } catch (e) {
    write(`\r\nERR: Failed to launch Nova: ${String(e)}\r\n`);
    statusEl.textContent = 'Error';
  }
}

window.addEventListener('resize', () => fitAddon.fit());
window.addEventListener('keydown', (e)=>{
  if (e.metaKey && e.key.toLowerCase() === 'k') { term.reset(); return; }
  if (e.key === 'Escape') { tauri.window.getCurrent().close(); return; }
});
term.onData(d => { tauri.core.invoke('pty_write', { data: d }); });

start();
