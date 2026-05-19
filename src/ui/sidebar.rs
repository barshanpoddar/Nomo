use gtk::prelude::*;

pub fn build_sidebar() -> (gtk::Widget, gtk::ListBox) {
    let list = gtk::ListBox::new();
    list.add_css_class("navigation-sidebar");
    list.set_selection_mode(gtk::SelectionMode::Single);

    let items = [
        ("Recent", "document-open-recent-symbolic"),
        ("Home", "user-home-symbolic"),
        ("Downloads", "folder-download-symbolic"),
        ("Documents", "folder-documents-symbolic"),
        ("Pictures", "folder-pictures-symbolic"),
        ("Drives & Network", "drive-harddisk-symbolic"),
    ];

    for (label, icon_name) in items {
        let row = gtk::ListBoxRow::new();
        
        let box_container = gtk::Box::new(gtk::Orientation::Horizontal, 12);
        box_container.set_margin_start(12);
        box_container.set_margin_end(12);
        box_container.set_margin_top(8);
        box_container.set_margin_bottom(8);
        
        let icon = gtk::Image::from_icon_name(icon_name);
        // Dim the icon slightly for standard sidebar look
        icon.add_css_class("dim-label");
        
        let label_widget = gtk::Label::new(Some(label));
        label_widget.set_xalign(0.0);
        
        box_container.append(&icon);
        box_container.append(&label_widget);
        
        row.set_child(Some(&box_container));
        list.append(&row);
    }

    let scroller = gtk::ScrolledWindow::new();
    scroller.add_css_class("sidebar-area");
    scroller.set_min_content_width(220);
    scroller.set_child(Some(&list));
    (scroller.upcast(), list)
}
