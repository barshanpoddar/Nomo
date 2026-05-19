# NAMO :wave:

NAMO is a cross-distro Linux file manager that blends the familiarity of Ubuntu Files with the productivity of Windows Explorer. The name comes from a Hindi word meaning “namaste.”

## Goals :dart:
- Fast, predictable file management for everyday Linux users
- Dual-pane productivity with a GNOME-first look and feel
- System-theme support without custom theme overrides in v1

## Scope (v1) :white_check_mark:
- Core file ops: copy, move, rename, delete, create folder
- Trash with undo
- Tabs and split view
- Sidebar shortcuts (Home, Downloads, Drives, Network)
- Search + filters (name, type, size, date)
- Archive create/extract
- Preview pane for media and PDFs

## UI Theory :art:
- Overall feel: close to Ubuntu Files with Windows-style dual-pane workflows
- Header bar: back/forward, breadcrumb path, grouped actions, and search
- Sidebar: compact “Places/Devices/Network” sections with clear icons
- Main area: dual list view with optional preview pane
- Theme: follows system theme (light/dark) and GNOME styling

## Tech Stack :wrench:
- Language: Rust
- UI: GTK4 + libadwaita
- File ops: gio/glib
- Archives: libarchive (planned)
- Previews: GTK widgets, GStreamer, and Poppler (planned)

## Project Layout :open_file_folder:
- src/main.rs: App shell, layout, and UI placeholders
- AI_CONTEXT.md: AI builder context and implementation plan

## Build Dependencies :package:
- Rust toolchain (stable)
- pkg-config
- GTK4 + libadwaita dev packages
- gdk-pixbuf dev package

On Ubuntu/Debian:

```
sudo apt install -y pkg-config libgtk-4-dev libadwaita-1-dev libgdk-pixbuf-2.0-dev
```

## Build :hammer:

```
cargo build
```

## Run :rocket:

```
cargo run
```

## Roadmap :triangular_flag_on_post:
1. Real directory listing + selection model
2. File operations with progress + undo
3. Tabs and split view state handling
4. Search + filters
5. Preview pane + archive support
6. Packaging (Flatpak/AppImage) and desktop integration

## Quick Actions :sparkles:
- [ ] Star the repo when you are happy with the progress :star:
- [ ] Test on another distro (Fedora, Arch, or openSUSE) :penguin:
- [ ] Share feedback or feature requests :speech_balloon:
