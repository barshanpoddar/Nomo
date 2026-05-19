use gtk::prelude::*;

pub fn build_recent_list() -> gtk::Widget {
    let starred = ["Brand Assets", "Invoices", "Roadmap.md", "Design.sketch"];
    let recents = ["Projects", "Notes.txt", "Screenshot.png", "Report.pdf"];

    let content = gtk::Box::new(gtk::Orientation::Vertical, 12);
    content.set_margin_bottom(12);
    content.append(&build_section("Starred", &starred));
    content.append(&build_section("Recent", &recents));

    let scroller = gtk::ScrolledWindow::new();
    scroller.set_vexpand(true);
    scroller.set_child(Some(&content));

    let container = gtk::Box::new(gtk::Orientation::Vertical, 0);
    container.set_hexpand(true);
    container.set_vexpand(true);
    container.append(&scroller);
    container.upcast()
}

fn build_section(title: &str, items: &[&str]) -> gtk::Widget {
    let label = gtk::Label::new(Some(title));
    label.add_css_class("heading");
    label.set_xalign(0.0);

    let list = gtk::ListBox::new();
    list.add_css_class("flat-list");
    for item in items {
        let text = gtk::Label::new(Some(item));
        text.set_xalign(0.0);
        let row = gtk::ListBoxRow::new();
        row.set_child(Some(&text));
        list.append(&row);
    }

    let section = gtk::Box::new(gtk::Orientation::Vertical, 6);
    section.set_margin_start(12);
    section.set_margin_end(12);
    section.append(&label);
    section.append(&list);
    section.upcast()
}
