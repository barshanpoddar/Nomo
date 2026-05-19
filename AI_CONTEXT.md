# NAMO AI Builder Context

## Goal
Create NAMO, a native Linux file manager using GTK4/libadwaita that blends Ubuntu Files familiarity with Windows-style dual-pane productivity.

## Scope (v1)
- Core file operations: copy, move, rename, delete, create folder
- Trash with undo
- Tabs and split view
- Search and filters (name, type, size, date)
- Sidebar shortcuts (Home, Downloads, Drives, Network)
- Archive create/extract
- Preview pane for common media and PDF
- Theme follows system settings

## UX Principles
- GNOME-first look and feel with a compact, grouped action bar
- Fast navigation with breadcrumb path and dual-pane option
- Minimal clutter, strong keyboard shortcuts
- Predictable context menus and drag/drop

## Architecture
- UI layer: GTK4/libadwaita widgets
- View models: selection state, current path, filters
- File services: gio/glib for file operations and monitoring
- Operation queue: async tasks with progress and undo stack
- Preview providers: images, media, documents
- Archive service: libarchive wrappers

## Implementation Plan
1. App shell and layout: header bar, sidebar, main area, status bar
2. Folder listing + selection model
3. File ops with progress and undo stack
4. Tabs + split view
5. Search + filters
6. Preview pane
7. Archive create/extract
8. Theming hooks and CSS overrides
9. Packaging (Flatpak/AppImage) and desktop integration

## Coding Guidelines
- Prefer async file operations to keep UI responsive
- Keep UI components small and reusable
- Avoid blocking the main thread
- Use system theme colors and spacing
- Add concise comments only for complex logic
