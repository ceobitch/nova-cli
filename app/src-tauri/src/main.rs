#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::io::{Read, Write};
use std::sync::{Arc, Mutex};
use tauri::{AppHandle, Emitter, Manager};
use portable_pty::{CommandBuilder, native_pty_system, PtySize};

struct PtyState {
  writer: Option<Box<dyn Write + Send>>,
}

#[tauri::command]
async fn pty_start(app: AppHandle, state: tauri::State<'_, Arc<Mutex<PtyState>>>, command: String, args: Option<Vec<String>>, cols: Option<u16>, rows: Option<u16>) -> Result<(), String> {
  let args = args.unwrap_or_default();
  let pty_system = native_pty_system();
  let pair = pty_system.openpty(PtySize { rows: rows.unwrap_or(32), cols: cols.unwrap_or(120), pixel_width: 0, pixel_height: 0 })
    .map_err(|e| format!("openpty error: {e}"))?;

  // Resolve command path; map sidecar to bundled resource path
  let resolved_cmd = if command == "./bin/nova" {
    if let Ok(res_dir) = app.path().resource_dir() {
      let name = if cfg!(target_os = "windows") { "nova.exe" } else { "nova" };
      res_dir.join("sidecar").join(name).to_string_lossy().to_string()
    } else {
      command.clone()
    }
  } else { command.clone() };

  let mut cmd = CommandBuilder::new(resolved_cmd);
  cmd.args(args);
  let _child = pair.slave.spawn_command(cmd).map_err(|e| format!("spawn error: {e}"))?;

  // Cache writer for subsequent writes
  let writer = pair.master.take_writer().map_err(|e| format!("writer error: {e}"))?;
  {
    let mut s = state.lock().unwrap();
    s.writer = Some(writer);
  }

  let app_for_thread = app.clone();
  std::thread::spawn(move || {
    let mut reader = pair.master.try_clone_reader().expect("reader");
    let mut buf = [0u8; 8192];
    loop {
      match reader.read(&mut buf) {
        Ok(0) => break,
        Ok(n) => { let _ = app_for_thread.emit("pty-data", String::from_utf8_lossy(&buf[..n]).to_string()); },
        Err(_) => break,
      }
    }
    let _ = app_for_thread.emit("pty-exit", ());
  });
  Ok(())
}

#[tauri::command]
async fn pty_write(state: tauri::State<'_, Arc<Mutex<PtyState>>>, data: String) -> Result<(), String> {
  let mut s = state.lock().unwrap();
  if let Some(w) = s.writer.as_mut() {
    w.write_all(data.as_bytes()).map_err(|e| e.to_string())?;
    w.flush().ok();
  }
  Ok(())
}

fn main() {
  tauri::Builder::default()
    .manage(Arc::new(Mutex::new(PtyState { writer: None })))
    .invoke_handler(tauri::generate_handler![pty_start, pty_write])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
