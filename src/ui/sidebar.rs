use gtk::prelude::*;

pub fn build_sidebar() -> (gtk::Widget, gtk::ListBox) {
    let list = gtk::ListBox::new();
    list.add_css_class("sidebar");
    list.set_selection_mode(gtk::SelectionMode::Single);

    for label in [
        "Recent",
        "Home",
        "Downloads",
        "Documents",
        "Pictures",
        "Drives",
        "Network",
    ] {
        let row = gtk::ListBoxRow::new();
        row.set_child(Some(&gtk::Label::new(Some(label))));
        list.append(&row);
    }

    let scroller = gtk::ScrolledWindow::new();
    scroller.set_min_content_width(220);
    scroller.set_child(Some(&list));
    (scroller.upcast(), list)
}
