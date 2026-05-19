use adw::prelude::*;
use glib::clone;

fn main() {
    let app = adw::Application::new(Some("com.namo.FileManager"), Default::default());

    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &adw::Application) {
    let header = adw::HeaderBar::new();
    header.set_title_widget(Some(&gtk::Label::new(Some("NAMO"))));

    let split_toggle = gtk::ToggleButton::new();
    split_toggle.set_icon_name("view-split-left-right-symbolic");
    split_toggle.set_tooltip_text(Some("Toggle split view"));
    header.pack_end(&split_toggle);

    let sidebar = build_sidebar();
    let primary_pane = build_file_list("Left pane");
    let secondary_pane = build_file_list("Right pane");

    let paned = gtk::Paned::new(gtk::Orientation::Horizontal);
    paned.set_start_child(Some(&primary_pane));
    paned.set_end_child(Some(&secondary_pane));
    paned.set_position(520);

    split_toggle.connect_toggled(clone!(@weak secondary_pane => move |btn| {
        secondary_pane.set_visible(btn.is_active());
    }));
    secondary_pane.set_visible(false);

    let content = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    content.append(&sidebar);
    content.append(&paned);

    let status = gtk::Label::new(Some("Ready"));
    status.set_xalign(0.0);
    status.add_css_class("dim-label");

    let main_box = gtk::Box::new(gtk::Orientation::Vertical, 0);
    main_box.append(&content);
    main_box.append(&status);

    let window = adw::ApplicationWindow::new(app);
    window.set_default_size(1100, 720);
    window.set_title(Some("NAMO"));
    let root = gtk::Box::new(gtk::Orientation::Vertical, 0);
    root.append(&header);
    root.append(&main_box);
    window.set_content(Some(&root));
    window.present();
}

fn build_sidebar() -> gtk::Widget {
    let list = gtk::ListBox::new();
    list.add_css_class("sidebar");

    for label in ["Home", "Downloads", "Documents", "Pictures", "Drives", "Network"] {
        let row = gtk::ListBoxRow::new();
        row.set_child(Some(&gtk::Label::new(Some(label))));
        list.append(&row);
    }

    let scroller = gtk::ScrolledWindow::new();
    scroller.set_min_content_width(220);
    scroller.set_child(Some(&list));
    scroller.upcast()
}

fn build_file_list(title: &str) -> gtk::Widget {
    let list = gtk::ListBox::new();

    for label in ["Folder A", "Folder B", "File.txt", "Image.png"] {
        let row = gtk::ListBoxRow::new();
        row.set_child(Some(&gtk::Label::new(Some(label))));
        list.append(&row);
    }

    let header = gtk::Label::new(Some(title));
    header.add_css_class("heading");

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
