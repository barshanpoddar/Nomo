use gtk::prelude::*;
use std::collections::HashSet;
use std::path::PathBuf;
use std::time::SystemTime;

#[derive(Clone)]
struct StarredItem {
    name: String,
    location: String,
    path: PathBuf,
    is_folder: bool,
}

#[derive(Clone)]
struct RecentItem {
    name: String,
    location: String,
    time: String,
    path: PathBuf,
    is_folder: bool,
}

pub fn build_recent_list() -> gtk::Widget {
    let starred = load_bookmarks();
    let recents = load_recent_files();

    let content = gtk::Box::new(gtk::Orientation::Vertical, 20);
    content.set_margin_top(16);
    content.set_margin_bottom(20);
    content.set_margin_start(16);
    content.set_margin_end(16);

    content.append(&build_starred_section(&starred));
    content.append(&build_recent_section(&recents));

    let scroller = gtk::ScrolledWindow::new();
    scroller.set_vexpand(true);
    scroller.set_child(Some(&content));

    let container = gtk::Box::new(gtk::Orientation::Vertical, 0);
    container.set_hexpand(true);
    container.set_vexpand(true);
    container.append(&scroller);
    container.upcast()
}

fn build_starred_section(items: &[StarredItem]) -> gtk::Widget {
    let section = gtk::Box::new(gtk::Orientation::Vertical, 12);
    section.add_css_class("recent-section");

    let header = build_section_header("Starred");

    let flow = gtk::FlowBox::new();
    flow.add_css_class("recent-flow");
    flow.set_selection_mode(gtk::SelectionMode::None);
    flow.set_row_spacing(12);
    flow.set_column_spacing(12);
    flow.set_min_children_per_line(2);
    flow.set_max_children_per_line(4);

    for item in items {
        let card = build_starred_card(&item.name, &item.location, item.is_folder);
        let child = gtk::FlowBoxChild::new();
        child.set_child(Some(&card));
        flow.insert(&child, -1);
    }

    section.append(&header);
    if items.is_empty() {
        let empty = gtk::Label::new(Some("No starred files or folders"));
        empty.add_css_class("recent-empty");
        empty.set_xalign(0.0);
        section.append(&empty);
    } else {
        section.append(&flow);
    }
    section.upcast()
}

fn build_starred_card(name: &str, group: &str, is_folder: bool) -> gtk::Widget {
    let card = gtk::Box::new(gtk::Orientation::Vertical, 8);
    card.add_css_class("recent-card");
    card.set_margin_bottom(2);

    let header = gtk::Box::new(gtk::Orientation::Horizontal, 8);
    let icon_name = if is_folder { "folder-symbolic" } else { "text-x-generic-symbolic" };
    let icon = gtk::Image::from_icon_name(icon_name);
    icon.set_pixel_size(20);
    icon.add_css_class("recent-icon");

    let title = gtk::Label::new(Some(name));
    title.add_css_class("recent-card-title");
    title.set_xalign(0.0);

    header.append(&icon);
    header.append(&title);

    let meta = gtk::Label::new(Some(group));
    meta.add_css_class("recent-card-meta");
    meta.set_xalign(0.0);

    card.append(&header);
    card.append(&meta);
    card.upcast()
}

fn build_recent_section(items: &[RecentItem]) -> gtk::Widget {
    let section = gtk::Box::new(gtk::Orientation::Vertical, 10);
    section.add_css_class("recent-section");

    let header = build_section_header("Recents");

    let list = gtk::ListBox::new();
    list.add_css_class("recent-list");
    list.set_selection_mode(gtk::SelectionMode::None);

    for item in items {
        let row = gtk::ListBoxRow::new();
        row.add_css_class("recent-row");
        row.set_activatable(false);

        let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 12);
        hbox.set_margin_start(12);
        hbox.set_margin_end(12);
        hbox.set_margin_top(10);
        hbox.set_margin_bottom(10);

        let icon_name = if item.is_folder { "folder-symbolic" } else { "text-x-generic-symbolic" };
        let icon = gtk::Image::from_icon_name(icon_name);
        icon.set_pixel_size(22);
        icon.add_css_class("recent-icon");

        let text_box = gtk::Box::new(gtk::Orientation::Vertical, 2);
        text_box.set_hexpand(true);

        let title = gtk::Label::new(Some(&item.name));
        title.add_css_class("recent-row-title");
        title.set_xalign(0.0);

        let meta = gtk::Label::new(Some(&item.location));
        meta.add_css_class("recent-row-meta");
        meta.set_xalign(0.0);

        text_box.append(&title);
        text_box.append(&meta);

        let time = gtk::Label::new(Some(&item.time));
        time.add_css_class("recent-row-time");
        time.set_xalign(1.0);
        time.set_visible(!item.time.is_empty());

        hbox.append(&icon);
        hbox.append(&text_box);
        hbox.append(&time);
        row.set_child(Some(&hbox));
        list.append(&row);
    }

    section.append(&header);
    section.append(&list);
    section.upcast()
}

fn load_bookmarks() -> Vec<StarredItem> {
    let Some(home) = home_dir() else { return Vec::new(); };
    let bookmarks_path = home.join(".config/gtk-3.0/bookmarks");
    let Ok(content) = std::fs::read_to_string(bookmarks_path) else { return Vec::new(); };

    let mut items = Vec::new();
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let mut parts = line.splitn(2, ' ');
        let uri = parts.next().unwrap_or_default();
        let label = parts.next().unwrap_or("").trim();

        let Some(path) = uri_to_path(uri) else { continue; };
        if !path.exists() {
            continue;
        }

        let name = if !label.is_empty() {
            label.to_string()
        } else {
            path.file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_else(|| path.to_string_lossy().to_string())
        };

        let location = path.parent()
            .and_then(|p| p.file_name())
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| "/".to_string());

        items.push(StarredItem {
            name,
            location,
            is_folder: path.is_dir(),
            path,
        });
    }

    items
}

fn load_recent_files() -> Vec<RecentItem> {
    let Some(home) = home_dir() else { return Vec::new(); };
    let recent_path = home.join(".local/share/recently-used.xbel");
    let Ok(content) = std::fs::read_to_string(recent_path) else { return Vec::new(); };

    let mut seen = HashSet::new();
    let mut items = Vec::new();

    for chunk in content.split("href=\"").skip(1) {
        let Some(rest) = chunk.split_once('"') else { continue; };
        let uri = rest.0;
        let Some(path) = uri_to_path(uri) else { continue; };
        if !path.exists() {
            continue;
        }

        if !seen.insert(path.clone()) {
            continue;
        }

        let name = path.file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| path.to_string_lossy().to_string());

        let location = path.parent()
            .and_then(|p| p.file_name())
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| "/".to_string());

        let time = std::fs::metadata(&path)
            .and_then(|m| m.modified())
            .ok()
            .map(format_relative_time)
            .unwrap_or_default();

        items.push(RecentItem {
            name,
            location,
            time,
            is_folder: path.is_dir(),
            path,
        });

        if items.len() >= 20 {
            break;
        }
    }

    items
}

fn format_relative_time(t: SystemTime) -> String {
    use std::time::UNIX_EPOCH;
    let Ok(d) = t.duration_since(UNIX_EPOCH) else { return String::new(); };
    let Ok(now) = SystemTime::now().duration_since(UNIX_EPOCH) else { return String::new(); };
    let diff = now.as_secs().saturating_sub(d.as_secs());
    if diff < 60 {
        "Just now".to_string()
    } else if diff < 3600 {
        format!("{} min", diff / 60)
    } else if diff < 86400 {
        format!("{} hr", diff / 3600)
    } else if diff < 86400 * 2 {
        "Yesterday".to_string()
    } else {
        format!("{} days", diff / 86400)
    }
}

fn uri_to_path(uri: &str) -> Option<PathBuf> {
    let path = uri.strip_prefix("file://").unwrap_or(uri);
    let decoded = path.replace("%20", " ");
    if decoded.is_empty() {
        None
    } else {
        Some(PathBuf::from(decoded))
    }
}

fn home_dir() -> Option<PathBuf> {
    std::env::var("HOME").ok().map(PathBuf::from)
}

fn build_section_header(title: &str) -> gtk::Widget {
    let header = gtk::Box::new(gtk::Orientation::Vertical, 6);

    let label = gtk::Label::new(Some(title));
    label.add_css_class("recent-section-title");
    label.set_xalign(0.0);

    let divider = gtk::Separator::new(gtk::Orientation::Horizontal);
    divider.add_css_class("recent-section-divider");

    header.append(&label);
    header.append(&divider);
    header.upcast()
}
