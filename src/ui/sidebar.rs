use gtk::prelude::*;

pub fn build_sidebar() -> (gtk::ScrolledWindow, gtk::ListBox, Vec<gtk::Label>, Vec<gtk::Box>, gtk::ListBoxRow) {
    let list = gtk::ListBox::new();
    list.add_css_class("navigation-sidebar");
    list.set_selection_mode(gtk::SelectionMode::Single);

    let search_row = gtk::ListBoxRow::new();
    search_row.set_selectable(false);
    search_row.set_visible(false); // Hidden by default
    search_row.add_css_class("search-row");
    
    let search_box = gtk::Box::new(gtk::Orientation::Horizontal, 12);
    search_box.set_margin_start(8);
    search_box.set_margin_end(8);
    search_box.set_margin_top(8);
    search_box.set_margin_bottom(8);
    
    let search_icon = gtk::Image::from_icon_name("system-search-symbolic");
    
    search_box.append(&search_icon);
    search_row.set_child(Some(&search_box));
    list.append(&search_row);

    let items = [
        ("Recent", "document-open-recent-symbolic"),
        ("Home", "user-home-symbolic"),
        ("Downloads", "folder-download-symbolic"),
        ("Documents", "folder-documents-symbolic"),
        ("Pictures", "folder-pictures-symbolic"),
        ("Audio", "folder-music-symbolic"),
        ("Video", "folder-videos-symbolic"),
        ("Drives & Network", "drive-harddisk-symbolic"),
    ];

    let mut labels = Vec::new();
    let mut containers = Vec::new();
    containers.push(search_box); // Add to containers so it gets centered

    for (label, icon_name) in items {
        let row = gtk::ListBoxRow::new();
        
        let box_container = gtk::Box::new(gtk::Orientation::Horizontal, 12);
        box_container.set_margin_start(8);
        box_container.set_margin_end(8);
        box_container.set_margin_top(8);
        box_container.set_margin_bottom(8);
        
        let icon = gtk::Image::from_icon_name(icon_name);
        // Dim the icon slightly for standard sidebar look
        icon.add_css_class("dim-label");
        
        let label_widget = gtk::Label::new(Some(label));
        label_widget.set_xalign(0.0);
        labels.push(label_widget.clone());
        containers.push(box_container.clone());
        
        box_container.append(&icon);
        box_container.append(&label_widget);
        
        row.set_child(Some(&box_container));
        list.append(&row);
    }

    let scroller = gtk::ScrolledWindow::new();
    scroller.add_css_class("sidebar-area");
    scroller.set_min_content_width(220);
    scroller.set_child(Some(&list));
    (scroller, list, labels, containers, search_row)
}
