use gtk::prelude::*;
use glib::clone;
use std::path::PathBuf;
use std::time::SystemTime;

use super::navigator::{NavState, ViewMode};

// ── Data Model ──────────────────────────────────────────────────────────────

#[derive(Clone)]
pub enum FileKind {
    Folder,
    Image,
    Document,
    Audio,
    Video,
    Archive,
    Generic,
}

impl FileKind {
    pub fn icon_name(&self) -> &'static str {
        match self {
            FileKind::Folder   => "folder-symbolic",
            FileKind::Image    => "image-x-generic-symbolic",
            FileKind::Document => "x-office-document-symbolic",
            FileKind::Audio    => "audio-x-generic-symbolic",
            FileKind::Video    => "video-x-generic-symbolic",
            FileKind::Archive  => "package-x-generic-symbolic",
            FileKind::Generic  => "text-x-generic-symbolic",
        }
    }

    pub fn from_path(path: &PathBuf) -> FileKind {
        if path.is_dir() {
            return FileKind::Folder;
        }
        let ext = path.extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_lowercase();
        match ext.as_str() {
            "jpg" | "jpeg" | "png" | "gif" | "bmp" | "webp" | "svg" | "heic" | "avif"
                => FileKind::Image,
            "pdf" | "doc" | "docx" | "odt" | "txt" | "md" | "rtf" | "xls" | "xlsx"
            | "ppt" | "pptx" | "csv" | "ods" | "odp"
                => FileKind::Document,
            "mp3" | "flac" | "ogg" | "wav" | "aac" | "opus" | "m4a" | "wma"
                => FileKind::Audio,
            "mp4" | "mkv" | "avi" | "mov" | "webm" | "flv" | "wmv" | "m4v"
                => FileKind::Video,
            "zip" | "tar" | "gz" | "bz2" | "xz" | "rar" | "7z" | "deb" | "rpm"
                => FileKind::Archive,
            _ => FileKind::Generic,
        }
    }
}

#[derive(Clone)]
pub struct FileEntry {
    pub name: String,
    pub path: PathBuf,
    pub kind: FileKind,
    pub size: Option<u64>,
    pub modified: Option<SystemTime>,
}

impl FileEntry {
    pub fn size_string(&self) -> String {
        match self.size {
            None => String::new(),
            Some(b) => {
                let kb = b as f64 / 1024.0;
                let mb = kb / 1024.0;
                let gb = mb / 1024.0;
                if gb >= 1.0      { format!("{:.1} GB", gb) }
                else if mb >= 1.0 { format!("{:.1} MB", mb) }
                else if kb >= 1.0 { format!("{:.1} KB", kb) }
                else              { format!("{} B", b) }
            }
        }
    }

    pub fn modified_string(&self) -> String {
        use std::time::UNIX_EPOCH;
        match self.modified {
            None => String::new(),
            Some(t) => {
                match t.duration_since(UNIX_EPOCH) {
                    Err(_) => String::new(),
                    Ok(d) => {
                        let secs = d.as_secs();
                        let now = SystemTime::now()
                            .duration_since(UNIX_EPOCH)
                            .map(|x| x.as_secs())
                            .unwrap_or(0);
                        let diff = now.saturating_sub(secs);
                        if diff < 60 {
                            "Just now".to_string()
                        } else if diff < 3600 {
                            format!("{} min ago", diff / 60)
                        } else if diff < 86400 {
                            format!("{} hr ago", diff / 3600)
                        } else if diff < 86400 * 2 {
                            "Yesterday".to_string()
                        } else {
                            format!("{} days ago", diff / 86400)
                        }
                    }
                }
            }
        }
    }
}

// ── Directory Scanner ────────────────────────────────────────────────────────

pub fn scan_directory(path: &PathBuf, show_hidden: bool) -> Vec<FileEntry> {
    let mut entries = Vec::new();
    let Ok(read) = std::fs::read_dir(path) else { return entries; };

    for entry in read.flatten() {
        let fpath = entry.path();
        let fname = fpath.file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_default();

        if !show_hidden && fname.starts_with('.') {
            continue;
        }

        let meta = std::fs::metadata(&fpath);
        let size = meta.as_ref().ok().and_then(|m| {
            if m.is_file() { Some(m.len()) } else { None }
        });
        let modified = meta.as_ref().ok().and_then(|m| m.modified().ok());
        let kind = FileKind::from_path(&fpath);

        entries.push(FileEntry { name: fname, path: fpath, kind, size, modified });
    }

    // Sort: folders first, then alphabetical
    entries.sort_by(|a, b| {
        let a_is_dir = matches!(a.kind, FileKind::Folder);
        let b_is_dir = matches!(b.kind, FileKind::Folder);
        b_is_dir.cmp(&a_is_dir).then(a.name.to_lowercase().cmp(&b.name.to_lowercase()))
    });

    entries
}

// ── Public: Build Full File View ─────────────────────────────────────────────

/// Builds the scrollable file browser content for the current navigator path.
/// Returns the widget to display, plus a callback to request a re-render
/// (the caller owns the container and swaps children).
pub fn build_file_view(nav: NavState, on_navigate: impl Fn() + Clone + 'static) -> gtk::Widget {
    let nav_ref = nav.borrow();
    let path = nav_ref.current_path.clone();
    let mode = nav_ref.view_mode.clone();
    let show_hidden = nav_ref.show_hidden;
    drop(nav_ref);

    let entries = scan_directory(&path, show_hidden);

    let scroller = gtk::ScrolledWindow::new();
    scroller.set_vexpand(true);
    scroller.set_hexpand(true);

    if entries.is_empty() {
        let empty = build_empty_state();
        scroller.set_child(Some(&empty));
    } else {
        match mode {
            ViewMode::List => {
                let list = build_list_view(&entries, nav.clone(), on_navigate.clone());
                scroller.set_child(Some(&list));
            }
            ViewMode::Grid => {
                let grid = build_grid_view(&entries, nav.clone(), on_navigate.clone());
                scroller.set_child(Some(&grid));
            }
        }
    }

    scroller.upcast()
}

// ── Empty State ───────────────────────────────────────────────────────────────

fn build_empty_state() -> gtk::Widget {
    let icon = gtk::Image::from_icon_name("folder-open-symbolic");
    icon.set_pixel_size(64);
    icon.add_css_class("empty-folder-icon");

    let label = gtk::Label::new(Some("This folder is empty"));
    label.add_css_class("empty-folder-label");

    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 12);
    vbox.set_valign(gtk::Align::Center);
    vbox.set_halign(gtk::Align::Center);
    vbox.set_vexpand(true);
    vbox.set_hexpand(true);
    vbox.append(&icon);
    vbox.append(&label);
    vbox.upcast()
}

// ── List View ─────────────────────────────────────────────────────────────────

fn build_list_view(
    entries: &[FileEntry],
    nav: NavState,
    on_navigate: impl Fn() + Clone + 'static,
) -> gtk::Widget {
    let list = gtk::ListBox::new();
    list.add_css_class("file-list");
    list.set_selection_mode(gtk::SelectionMode::Single);

    for entry in entries {
        let row = build_list_row(entry);
        add_context_menu(row.upcast_ref(), entry.clone(), nav.clone(), on_navigate.clone());
        list.append(&row);
    }

    let entries_clone: Vec<FileEntry> = entries.to_vec();
    list.connect_row_activated(clone!(@strong nav, @strong on_navigate => move |_, row| {
        let idx = row.index() as usize;
        if let Some(entry) = entries_clone.get(idx) {
            handle_entry_activate(entry, &nav, &on_navigate);
        }
    }));

    list.upcast()
}

fn build_list_row(entry: &FileEntry) -> gtk::ListBoxRow {
    let row = gtk::ListBoxRow::new();
    row.add_css_class("file-row");

    let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 12);
    hbox.set_margin_start(8);
    hbox.set_margin_end(12);
    hbox.set_margin_top(6);
    hbox.set_margin_bottom(6);

    // Icon
    let icon = gtk::Image::from_icon_name(entry.kind.icon_name());
    icon.set_pixel_size(28);
    if matches!(entry.kind, FileKind::Folder) {
        icon.add_css_class("folder-icon");
    } else {
        icon.add_css_class("file-icon");
    }

    // Name
    let name_label = gtk::Label::new(Some(&entry.name));
    name_label.set_xalign(0.0);
    name_label.set_hexpand(true);
    name_label.set_ellipsize(gtk::pango::EllipsizeMode::End);
    if matches!(entry.kind, FileKind::Folder) {
        name_label.add_css_class("file-name-folder");
    } else {
        name_label.add_css_class("file-name");
    }

    // Modified
    let mod_label = gtk::Label::new(Some(&entry.modified_string()));
    mod_label.add_css_class("dim-label");
    mod_label.set_width_chars(12);
    mod_label.set_xalign(1.0);

    // Size
    let size_label = gtk::Label::new(Some(&entry.size_string()));
    size_label.add_css_class("dim-label");
    size_label.set_width_chars(8);
    size_label.set_xalign(1.0);

    hbox.append(&icon);
    hbox.append(&name_label);
    hbox.append(&mod_label);
    hbox.append(&size_label);

    row.set_child(Some(&hbox));
    row
}

// ── Grid View ─────────────────────────────────────────────────────────────────

fn build_grid_view(
    entries: &[FileEntry],
    nav: NavState,
    on_navigate: impl Fn() + Clone + 'static,
) -> gtk::Widget {
    let flow = gtk::FlowBox::new();
    flow.set_homogeneous(true);
    flow.set_min_children_per_line(3);
    flow.set_max_children_per_line(12);
    flow.set_column_spacing(8);
    flow.set_row_spacing(8);
    flow.set_margin_start(12);
    flow.set_margin_end(12);
    flow.set_margin_top(12);
    flow.set_margin_bottom(12);
    flow.set_selection_mode(gtk::SelectionMode::Single);

    for entry in entries {
        let child = build_grid_card(entry);
        add_context_menu(child.upcast_ref(), entry.clone(), nav.clone(), on_navigate.clone());
        flow.insert(&child, -1);
    }

    let entries_clone: Vec<FileEntry> = entries.to_vec();
    flow.connect_child_activated(clone!(@strong nav, @strong on_navigate => move |_, child| {
        let idx = child.index() as usize;
        if let Some(entry) = entries_clone.get(idx) {
            handle_entry_activate(entry, &nav, &on_navigate);
        }
    }));

    flow.upcast()
}

fn build_grid_card(entry: &FileEntry) -> gtk::FlowBoxChild {
    let child = gtk::FlowBoxChild::new();
    child.add_css_class("file-card");

    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 6);
    vbox.set_halign(gtk::Align::Center);
    vbox.set_margin_top(8);
    vbox.set_margin_bottom(8);
    vbox.set_margin_start(4);
    vbox.set_margin_end(4);

    let icon = gtk::Image::from_icon_name(entry.kind.icon_name());
    icon.set_pixel_size(52);
    if matches!(entry.kind, FileKind::Folder) {
        icon.add_css_class("folder-icon");
    } else {
        icon.add_css_class("file-icon");
    }
    icon.set_halign(gtk::Align::Center);

    let name_label = gtk::Label::new(Some(&entry.name));
    name_label.set_halign(gtk::Align::Center);
    name_label.set_max_width_chars(14);
    name_label.set_wrap(true);
    name_label.set_justify(gtk::Justification::Center);
    if matches!(entry.kind, FileKind::Folder) {
        name_label.add_css_class("file-name-folder");
    } else {
        name_label.add_css_class("file-name");
    }

    let size_label = gtk::Label::new(Some(&entry.size_string()));
    size_label.add_css_class("dim-label");
    size_label.set_halign(gtk::Align::Center);

    vbox.append(&icon);
    vbox.append(&name_label);
    if !entry.size_string().is_empty() {
        vbox.append(&size_label);
    }

    child.set_child(Some(&vbox));
    child
}

fn add_context_menu(
    widget: &gtk::Widget,
    entry: FileEntry,
    nav: NavState,
    on_navigate: impl Fn() + Clone + 'static,
) {
    let popover = build_context_popover(entry, nav, on_navigate);
    popover.set_parent(widget);
    popover.set_autohide(true);

    let click = gtk::GestureClick::new();
    click.set_button(3);
    click.connect_pressed(clone!(@weak popover => move |_, _, x, y| {
        popover.set_pointing_to(Some(&gtk::gdk::Rectangle::new(
            x as i32,
            y as i32,
            1,
            1,
        )));
        popover.popup();
    }));
    widget.add_controller(click);
}

fn build_context_popover(
    entry: FileEntry,
    nav: NavState,
    on_navigate: impl Fn() + Clone + 'static,
) -> gtk::Popover {
    let popover = gtk::Popover::new();
    popover.set_has_arrow(false);

    let content = gtk::Box::new(gtk::Orientation::Vertical, 6);
    content.set_margin_top(6);
    content.set_margin_bottom(6);
    content.set_margin_start(6);
    content.set_margin_end(6);

    let open_btn = gtk::Button::with_label("Open");
    open_btn.add_css_class("flat");
    open_btn.connect_clicked(clone!(@strong entry, @strong nav, @strong on_navigate, @weak popover => move |_| {
        handle_entry_activate(&entry, &nav, &on_navigate);
        popover.popdown();
    }));

    let copy_btn = gtk::Button::with_label("Copy path");
    copy_btn.add_css_class("flat");
    copy_btn.connect_clicked(clone!(@strong entry, @weak popover => move |_| {
        if let Some(display) = gtk::gdk::Display::default() {
            display.clipboard().set_text(&entry.path.to_string_lossy());
        }
        popover.popdown();
    }));

    content.append(&open_btn);
    content.append(&copy_btn);
    popover.set_child(Some(&content));
    popover
}

// ── Activation Handler ────────────────────────────────────────────────────────

fn handle_entry_activate(entry: &FileEntry, nav: &NavState, on_navigate: &impl Fn()) {
    if matches!(entry.kind, FileKind::Folder) {
        nav.borrow_mut().navigate_to(entry.path.clone());
        on_navigate();
    } else {
        // Open file with default system application
        let uri = format!("file://{}", entry.path.to_string_lossy());
        if let Err(e) = open::that(&entry.path) {
            eprintln!("Failed to open file '{}': {}", uri, e);
        }
    }
}
