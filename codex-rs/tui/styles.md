# Headers, primary, and secondary text

- **Headers:** Use `bold`. For markdown with various header levels, leave in the `#` signs.
- **Primary text:** Default.
- **Secondary text:** Use `dim`.

# Foreground colors

- **Default:** Most of the time, just use the default foreground color. `reset` can help get it back.
- **User input tips, selection, and status indicators:** Use ANSI `red` (orange theme).
- **Success and additions:** Use ANSI `green`.
- **Errors, failures and deletions:** Use ANSI `red`.
- Use ANSI `red` (orange/red theme) for branded highlights.

# Avoid

- Avoid custom colors because there's no guarantee that they'll contrast well or look good in various terminal color themes. (`shimmer.rs` is an exception that works well because we take the default colors and just adjust their levels.)
- Avoid ANSI `black` & `white` as foreground colors because the default terminal theme color will do a better job. (Use `reset` if you need to in order to get those.) The exception is if you need contrast rendering over a manually colored background.
- Avoid ANSI `blue` and `yellow` because for now the style guide doesn't use them. Prefer a foreground color mentioned above.

(There are some rules to try to catch this in `clippy.toml`.)

# Dark theme palette (for integrated shell)

When rendered inside the Nova macOS shell or Tauri shell, prefer a cohesive dark palette:

- Background: `#0b0f14`
- Surface: `#11161d`
- Border subtle: `#1a212b`
- Primary text: `#e6eef8`
- Secondary text: `#9fb0c3`
- Accent: `#6ea8fe`

Prefer Ratatui defaults mapped to these via nearest ANSI where custom colors are unavailable.
