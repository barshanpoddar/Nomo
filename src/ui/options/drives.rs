use gtk::prelude::*;

pub fn build_drives_network_view() -> gtk::Widget {
    let header = gtk::Label::new(Some("Drives & Network"));
    header.add_css_class("heading");
    header.set_xalign(0.0);

    let header_box = gtk::Box::new(gtk::Orientation::Vertical, 6);
    header_box.set_margin_top(12);
    header_box.set_margin_bottom(6);
    header_box.set_margin_start(12);
    header_box.set_margin_end(12);
    header_box.append(&header);

    let content = gtk::Box::new(gtk::Orientation::Vertical, 16);
    content.set_margin_top(6);
    content.set_margin_bottom(12);
    content.set_margin_start(12);
    content.set_margin_end(12);
    content.append(&build_section("Drives", &["System", "Backup"]));
    content.append(&build_section("Network", &["Office NAS", "Team Share"]));

    let scroller = gtk::ScrolledWindow::new();
    scroller.set_vexpand(true);
    scroller.set_child(Some(&content));

    let container = gtk::Box::new(gtk::Orientation::Vertical, 0);
    container.set_hexpand(true);
    container.set_vexpand(true);
    container.append(&header_box);
    container.append(&scroller);
    container.upcast()
}

fn build_section(title: &str, items: &[&str]) -> gtk::Widget {
    let title_label = gtk::Label::new(Some(title));
    title_label.add_css_class("title-4");
    title_label.set_xalign(0.0);

    let list = gtk::ListBox::new();
    for item in items {
        let row = gtk::ListBoxRow::new();
        let label = gtk::Label::new(Some(item));
        label.set_xalign(0.0);
        row.set_child(Some(&label));
        list.append(&row);
    }

    let container = gtk::Box::new(gtk::Orientation::Vertical, 8);
    container.append(&title_label);
    container.append(&list);
    container.upcast()
}
