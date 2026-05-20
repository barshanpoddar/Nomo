use adw::prelude::*;
use glib::{clone, source::timeout_add_local_once};
use std::path::PathBuf;
use std::time::Duration;

mod ui;

use ui::navigator::{NavState, Navigator, ViewMode};
use ui::file_view::build_file_view;

fn main() {
    let app = adw::Application::new(Some("com.namo.FileManager"), Default::default());
    app.connect_activate(build_ui);
    app.run();
}

fn home_dir() -> PathBuf {
    PathBuf::from(std::env::var("HOME").unwrap_or_else(|_| "/".to_string()))
}

// ── Sidebar section root paths ───────────────────────────────────────────────

fn section_root(idx: i32) -> Option<PathBuf> {
    let home = home_dir();
    match idx {
        2 => Some(home),
        3 => Some(home_dir().join("Downloads")),
        4 => Some(home_dir().join("Documents")),
        5 => Some(home_dir().join("Pictures")),
        6 => Some(home_dir().join("Music")),
        7 => Some(home_dir().join("Videos")),
        _ => None,
    }
}

fn section_info(idx: i32) -> (&'static str, &'static str) {
    match idx {
        1 => ("Recent", "document-open-recent-symbolic"),
        2 => ("Home", "user-home-symbolic"),
        3 => ("Downloads", "folder-download-symbolic"),
        4 => ("Documents", "folder-documents-symbolic"),
        5 => ("Pictures", "folder-pictures-symbolic"),
        6 => ("Audio", "folder-music-symbolic"),
        7 => ("Video", "folder-videos-symbolic"),
        9 => ("Drives & Network", "drive-harddisk-symbolic"),
        _ => ("", ""),
    }
}

// ── Main refresh function ────────────────────────────────────────────────────

fn refresh_view(
    content_host: &gtk::Box,
    nav: &NavState,
    back_btn: &gtk::Button,
    forward_btn: &gtk::Button,
    path_bar_label: &gtk::Label,
    path_bar_icon: &gtk::Image,
) {
    // Remove old content
    while let Some(child) = content_host.first_child() {
        content_host.remove(&child);
    }

    let nav_ref = nav.borrow();
    let path = nav_ref.current_path.clone();
    let can_back = nav_ref.can_go_back();
    let can_fwd = nav_ref.can_go_forward();
    let dir_name = nav_ref.current_dir_name();
    let is_grid = matches!(nav_ref.view_mode, ViewMode::Grid);
    drop(nav_ref);

    back_btn.set_sensitive(can_back);
    forward_btn.set_sensitive(can_fwd);

    // Update path bar
    path_bar_label.set_text(&dir_name);

    // Guess icon from path
    let icon = path_icon_name(&path);
    path_bar_icon.set_icon_name(Some(icon));

    // Build file view - note we must clone refs for the callback
    let content_host_weak = content_host.downgrade();
    let back_btn_weak = back_btn.downgrade();
    let forward_btn_weak = forward_btn.downgrade();
    let path_bar_label_weak = path_bar_label.downgrade();
    let path_bar_icon_weak = path_bar_icon.downgrade();
    let nav_clone = nav.clone();

    let on_navigate = move || {
        if let (Some(ch), Some(bb), Some(fb), Some(pbl), Some(pbi)) = (
            content_host_weak.upgrade(),
            back_btn_weak.upgrade(),
            forward_btn_weak.upgrade(),
            path_bar_label_weak.upgrade(),
            path_bar_icon_weak.upgrade(),
        ) {
            refresh_view(&ch, &nav_clone, &bb, &fb, &pbl, &pbi);
        }
    };

    let _ = is_grid; // used by ViewMode in build_file_view internally
    let view = build_file_view(nav.clone(), on_navigate);
    content_host.append(&view);
}

fn path_icon_name(path: &PathBuf) -> &'static str {
    let s = path.to_string_lossy();
    let home = home_dir().to_string_lossy().to_string();
    if s == home           { "user-home-symbolic" }
    else if s.ends_with("Downloads")  { "folder-download-symbolic" }
    else if s.ends_with("Documents")  { "folder-documents-symbolic" }
    else if s.ends_with("Pictures")   { "folder-pictures-symbolic" }
    else if s.ends_with("Music")      { "folder-music-symbolic" }
    else if s.ends_with("Videos")     { "folder-videos-symbolic" }
    else if s == "/"       { "drive-harddisk-symbolic" }
    else                   { "folder-symbolic" }
}

// ── Build UI ─────────────────────────────────────────────────────────────────

fn build_ui(app: &adw::Application) {
    load_css();

    // ── Sidebar header ────────────────────────────────────────────────
    let sidebar_header = adw::HeaderBar::new();
    sidebar_header.add_css_class("flat");
    sidebar_header.set_show_end_title_buttons(false);

    let app_title = gtk::Label::new(Some("NAMO"));
    app_title.add_css_class("sidebar-title");
    sidebar_header.set_title_widget(Some(&app_title));

    let search_btn = gtk::Button::new();
    search_btn.set_icon_name("system-search-symbolic");
    search_btn.add_css_class("flat");
    sidebar_header.pack_end(&search_btn);

    let menu_btn = gtk::ToggleButton::new();
    menu_btn.set_icon_name("sidebar-show-symbolic");
    menu_btn.set_active(true);
    menu_btn.set_tooltip_text(Some("Toggle sidebar"));
    sidebar_header.pack_start(&menu_btn);

    // ── Content header ────────────────────────────────────────────────
    let content_header = adw::HeaderBar::new();
    content_header.add_css_class("flat");
    content_header.set_show_start_title_buttons(false);

    let refresh_btn = gtk::Button::new();
    refresh_btn.add_css_class("flat");
    refresh_btn.set_tooltip_text(Some("Refresh"));

    let refresh_icon = gtk::Image::from_icon_name("view-refresh-symbolic");
    refresh_icon.set_pixel_size(16);

    let refresh_spinner = gtk::Spinner::new();
    refresh_spinner.set_size_request(16, 16);

    let refresh_stack = gtk::Stack::new();
    refresh_stack.add_named(&refresh_icon, Some("icon"));
    refresh_stack.add_named(&refresh_spinner, Some("spinner"));
    refresh_stack.set_visible_child_name("icon");

    refresh_btn.set_child(Some(&refresh_stack));
    content_header.pack_start(&refresh_btn);

    let back_btn = gtk::Button::new();
    back_btn.set_icon_name("go-previous-symbolic");
    back_btn.add_css_class("flat");
    back_btn.set_sensitive(false);
    content_header.pack_start(&back_btn);

    let forward_btn = gtk::Button::new();
    forward_btn.set_icon_name("go-next-symbolic");
    forward_btn.add_css_class("flat");
    forward_btn.set_sensitive(false);
    content_header.pack_start(&forward_btn);

    // Path bar pill
    let path_bar = gtk::Box::new(gtk::Orientation::Horizontal, 8);
    path_bar.add_css_class("path-bar-pill");
    path_bar.set_width_request(350);

    let path_bar_icon = gtk::Image::from_icon_name("document-open-recent-symbolic");
    let path_bar_label = gtk::Label::new(Some("Recent"));
    path_bar_label.set_hexpand(true);
    path_bar_label.set_xalign(0.0);
    path_bar_label.add_css_class("path-bar-text");

    let path_bar_more = gtk::Image::from_icon_name("view-more-symbolic");
    path_bar_more.add_css_class("dim-label");

    path_bar.append(&path_bar_icon);
    path_bar.append(&path_bar_label);
    path_bar.append(&path_bar_more);
    content_header.set_title_widget(Some(&path_bar));

    // End buttons
    let caret_btn = gtk::Button::new();
    caret_btn.set_icon_name("pan-down-symbolic");
    caret_btn.add_css_class("flat");
    content_header.pack_end(&caret_btn);

    let header_sep = gtk::Separator::new(gtk::Orientation::Vertical);
    header_sep.set_margin_top(12);
    header_sep.set_margin_bottom(12);
    content_header.pack_end(&header_sep);

    let view_toggle = gtk::ToggleButton::new();
    view_toggle.set_icon_name("view-grid-symbolic");
    view_toggle.add_css_class("flat");
    view_toggle.set_tooltip_text(Some("Toggle grid/list view"));
    content_header.pack_end(&view_toggle);

    let search_header_btn = gtk::Button::new();
    search_header_btn.set_icon_name("system-search-symbolic");
    search_header_btn.add_css_class("flat");
    content_header.pack_end(&search_header_btn);

    // ── Sidebar ───────────────────────────────────────────────────────
    let (sidebar, sidebar_list, sidebar_labels, sidebar_containers, search_row) =
        ui::sidebar::build_sidebar();
    sidebar.set_vexpand(true);

    let sidebar_box = gtk::Box::new(gtk::Orientation::Vertical, 0);
    sidebar_box.append(&sidebar_header);
    sidebar_box.append(&sidebar);

    // ── Navigator state ───────────────────────────────────────────────
    // Start on Recent view (no directory path — special case)
    let nav = Navigator::new(home_dir());

    // ── Content host & status ─────────────────────────────────────────
    let content_host = gtk::Box::new(gtk::Orientation::Vertical, 0);
    content_host.set_hexpand(true);
    content_host.set_vexpand(true);

    // Drives panel (separate non-navigator view)
    let drives_panel = ui::options::drives::build_drives_network_view();

    // Recent panel
    let recent_panel = ui::options::recent::build_recent_list();

    // Main stack — switches between special pages and the live file view
    let stack = gtk::Stack::new();
    stack.set_hexpand(true);
    stack.set_vexpand(true);
    stack.add_named(&recent_panel, Some("recent"));
    stack.add_named(&content_host, Some("files"));
    stack.add_named(&drives_panel, Some("drives_network"));
    stack.set_visible_child_name("recent");

    // Initial load for Home
    // (we won't auto-populate since we start on Recent)

    // ── Sidebar collapse toggle ───────────────────────────────────────
    menu_btn.connect_toggled(clone!(@weak sidebar, @weak app_title, @weak search_btn, @weak search_row => move |btn| {
        let is_expanded = btn.is_active();
        app_title.set_visible(is_expanded);
        search_btn.set_visible(is_expanded);
        search_row.set_visible(!is_expanded);
        for label in &sidebar_labels {
            label.set_visible(is_expanded);
        }
        for container in &sidebar_containers {
            if is_expanded {
                container.set_halign(gtk::Align::Fill);
            } else {
                container.set_halign(gtk::Align::Center);
            }
        }
        sidebar.set_min_content_width(if is_expanded { 220 } else { 32 });
    }));

    // ── View toggle (grid ↔ list) ─────────────────────────────────────
    view_toggle.connect_toggled(clone!(
        @weak content_host, @weak back_btn, @weak forward_btn,
        @weak path_bar_label, @weak path_bar_icon,
        @strong nav => move |btn| {
            nav.borrow_mut().toggle_view_mode();
            btn.set_icon_name(if btn.is_active() {
                "view-list-bullet-symbolic"
            } else {
                "view-grid-symbolic"
            });
            refresh_view(&content_host, &nav, &back_btn, &forward_btn,
                &path_bar_label, &path_bar_icon);
    }));

    // ── Back button ───────────────────────────────────────────────────
    back_btn.connect_clicked(clone!(
        @weak content_host, @weak back_btn, @weak forward_btn,
        @weak path_bar_label, @weak path_bar_icon, @weak stack,
        @strong nav => move |_| {
            let went = nav.borrow_mut().go_back();
            if went {
                stack.set_visible_child_name("files");
                refresh_view(&content_host, &nav, &back_btn, &forward_btn,
                    &path_bar_label, &path_bar_icon);
            }
    }));

    // ── Forward button ────────────────────────────────────────────────
    forward_btn.connect_clicked(clone!(
        @weak content_host, @weak back_btn, @weak forward_btn,
        @weak path_bar_label, @weak path_bar_icon, @weak stack,
        @strong nav => move |_| {
            let went = nav.borrow_mut().go_forward();
            if went {
                stack.set_visible_child_name("files");
                refresh_view(&content_host, &nav, &back_btn, &forward_btn,
                    &path_bar_label, &path_bar_icon);
            }
    }));

    // ── Refresh button ───────────────────────────────────────────────
    refresh_btn.connect_clicked(clone!(
        @weak refresh_spinner, @weak refresh_stack,
        @weak content_host, @weak back_btn, @weak forward_btn,
        @weak path_bar_label, @weak path_bar_icon,
        @strong nav => move |_| {
            refresh_spinner.start();
            refresh_stack.set_visible_child_name("spinner");
            refresh_view(&content_host, &nav, &back_btn, &forward_btn,
                &path_bar_label, &path_bar_icon);
            timeout_add_local_once(Duration::from_millis(350), clone!(@weak refresh_spinner, @weak refresh_stack => move || {
                refresh_spinner.stop();
                refresh_stack.set_visible_child_name("icon");
            }));
    }));

    // ── Sidebar row selection ─────────────────────────────────────────
    if let Some(row) = sidebar_list.row_at_index(1) {
        sidebar_list.select_row(Some(&row));
    }

    sidebar_list.connect_row_selected(clone!(
        @weak stack, @weak content_host, @weak back_btn, @weak forward_btn,
        @weak path_bar_label, @weak path_bar_icon,
        @strong nav => move |_, row| {
            let Some(row) = row else { return; };
            let idx = row.index();
            if idx == 0 || idx == 8 { return; }

            let (title, icon) = section_info(idx);

            match idx {
                1 => {
                    // Recent — special static view
                    path_bar_label.set_text(title);
                    path_bar_icon.set_icon_name(Some(icon));
                    back_btn.set_sensitive(nav.borrow().can_go_back());
                    forward_btn.set_sensitive(nav.borrow().can_go_forward());
                    stack.set_visible_child_name("recent");
                }
                9 => {
                    // Drives & Network — special static view
                    path_bar_label.set_text(title);
                    path_bar_icon.set_icon_name(Some(icon));
                    back_btn.set_sensitive(nav.borrow().can_go_back());
                    forward_btn.set_sensitive(nav.borrow().can_go_forward());
                    stack.set_visible_child_name("drives_network");
                }
                _ => {
                    // Live file view
                    if let Some(root) = section_root(idx) {
                        nav.borrow_mut().navigate_to(root);
                        stack.set_visible_child_name("files");
                        refresh_view(&content_host, &nav, &back_btn, &forward_btn,
                            &path_bar_label, &path_bar_icon);
                    }
                }
            }
    }));

    // ── Layout ────────────────────────────────────────────────────────
    let content_container = gtk::Box::new(gtk::Orientation::Vertical, 0);
    content_container.add_css_class("content-container");
    content_container.set_hexpand(true);
    content_container.set_vexpand(true);
    content_container.append(&stack);

    let right_box = gtk::Box::new(gtk::Orientation::Vertical, 0);
    right_box.set_hexpand(true);
    right_box.set_vexpand(true);
    right_box.append(&content_header);
    right_box.append(&content_container);

    let main_content = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    main_content.append(&sidebar_box);
    main_content.append(&gtk::Separator::new(gtk::Orientation::Vertical));
    main_content.append(&right_box);

    let window = adw::ApplicationWindow::new(app);
    window.set_default_size(1100, 720);
    window.set_title(Some("NOMO"));
    window.set_content(Some(&main_content));
    window.present();
}

// ── CSS ───────────────────────────────────────────────────────────────────────

fn load_css() {
    let provider = gtk::CssProvider::new();
    provider.load_from_data(
        /* Sidebar */
        ".flat-list, .flat-list row { background-color: transparent; }\n\
.flat-list row:selected { background-color: transparent; }\n\
.sidebar-title { font-weight: 800; font-size: 14px; }\n\
/* Content container */\n\
.content-container { background-color: @view_bg_color; }\n\
/* Search row */\n\
.search-row { background-color: alpha(@accent_bg_color, 0.15); color: @accent_color; border-radius: 6px; }\n\
    /* Sidebar divider */\n\
         .sidebar-divider { opacity: 0.45; }\n\
            .sidebar-divider-row { padding: 0; min-height: 2px; }\n\
/* Path bar */\n\
.path-bar-pill { background-color: alpha(@window_fg_color, 0.06); border: 1px solid alpha(@window_fg_color, 0.08); border-radius: 8px; padding: 4px 12px; }\n\
.path-bar-text { font-weight: 600; }\n\
/* Recent view */\n\
.recent-hero { padding: 2px 2px 6px; }\n\
.recent-title { font-size: 22px; font-weight: 700; }\n\
.recent-subtitle { opacity: 0.6; }\n\
.recent-section-title { font-size: 13px; font-weight: 700; opacity: 0.75; letter-spacing: 0.2px; }\n\
.recent-section-divider { opacity: 0.2; }\n\
.recent-empty { opacity: 0.6; font-size: 12px; margin-top: 4px; }\n\
.recent-flow flowboxchild { padding: 0; }\n\
.recent-card { background-color: alpha(@window_fg_color, 0.04); border: 1px solid alpha(@window_fg_color, 0.08); border-radius: 10px; padding: 12px; }\n\
.recent-card-title { font-weight: 600; }\n\
.recent-card-meta { opacity: 0.6; font-size: 12px; }\n\
.recent-list > row { border-radius: 10px; background-color: alpha(@window_fg_color, 0.03); margin-bottom: 8px; }\n\
.recent-list > row:hover { background-color: alpha(@window_fg_color, 0.06); }\n\
.recent-row-title { font-weight: 600; }\n\
.recent-row-meta { opacity: 0.6; font-size: 12px; }\n\
.recent-row-time { opacity: 0.5; font-size: 12px; }\n\
.recent-icon { color: @accent_color; }\n\
/* File list rows */\n\
.file-list { background: transparent; }\n\
.file-list > row { border-radius: 6px; padding: 0; transition: background 120ms ease; }\n\
.file-list > row:hover { background-color: alpha(@window_fg_color, 0.05); }\n\
.file-list > row:selected { background-color: alpha(@accent_bg_color, 0.18); }\n\
.file-row { border-radius: 6px; }\n\
.file-name { font-size: 13px; }\n\
.file-name-folder { font-size: 13px; font-weight: 600; }\n\
.folder-icon { color: @accent_color; }\n\
.file-icon { opacity: 0.65; }\n\
/* Grid cards */\n\
.file-card { border-radius: 10px; padding: 4px; transition: background 120ms ease; min-width: 110px; }\n\
.file-card:hover { background-color: alpha(@window_fg_color, 0.05); }\n\
.file-card:selected { background-color: alpha(@accent_bg_color, 0.18); }\n\
/* Empty state */\n\
.empty-folder-icon { opacity: 0.3; }\n\
.empty-folder-label { opacity: 0.4; font-size: 14px; }\n\
/* Status bar */\n\
.status-bar { font-size: 11px; }",
    );

    if let Some(display) = gtk::gdk::Display::default() {
        gtk::style_context_add_provider_for_display(
            &display,
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    }
}
