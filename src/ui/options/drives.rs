use gtk::prelude::*;

pub fn build_drives_view() -> gtk::Widget {
    build_simple_list("Drives", &["System", "Backup", "Shared"])
}

fn build_simple_list(title: &str, items: &[&str]) -> gtk::Widget {
    let header = gtk::Label::new(Some(title));
    header.add_css_class("heading");

    let list = gtk::ListBox::new();
    for item in items {
        let row = gtk::ListBoxRow::new();
        row.set_child(Some(&gtk::Label::new(Some(item))));
        list.append(&row);
    }

    let header_box = gtk::Box::new(gtk::Orientation::Vertical, 6);
    header_box.set_margin_top(12);
    header_box.set_margin_bottom(12);
    header_box.set_margin_start(12);
    header_box.set_margin_end(12);
    header_box.append(&header);

    let scroller = gtk::ScrolledWindow::new();
    scroller.set_vexpand(true);
    scroller.set_child(Some(&list));

    let container = gtk::Box::new(gtk::Orientation::Vertical, 0);
    container.set_hexpand(true);
    container.set_vexpand(true);
    container.append(&header_box);
    container.append(&scroller);
    container.upcast()
}
