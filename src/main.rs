use adw::prelude::*;
use glib::clone;

mod ui;

fn main() {
    let app = adw::Application::new(Some("com.namo.FileManager"), Default::default());

    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &adw::Application) {
    load_css();
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

    let content_header = adw::HeaderBar::new();
    content_header.add_css_class("flat");
    content_header.set_show_start_title_buttons(false);
    
    let content_title = gtk::Label::new(Some("Recent"));
    content_title.add_css_class("content-title");
    content_header.set_title_widget(Some(&content_title));

    let split_toggle = gtk::ToggleButton::new();
    split_toggle.set_icon_name("view-split-left-right-symbolic");
    split_toggle.set_tooltip_text(Some("Toggle split view"));
    content_header.pack_end(&split_toggle);

    let (sidebar, sidebar_list, sidebar_labels, sidebar_containers, search_row) = ui::sidebar::build_sidebar();
    sidebar.set_vexpand(true);

    let sidebar_box = gtk::Box::new(gtk::Orientation::Vertical, 0);
    sidebar_box.append(&sidebar_header);
    sidebar_box.append(&sidebar);

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
        if is_expanded {
            sidebar.set_min_content_width(220);
        } else {
            sidebar.set_min_content_width(32);
        }
    }));

    let stack = gtk::Stack::new();
    stack.set_hexpand(true);
    stack.set_vexpand(true);
    stack.add_named(&ui::options::recent::build_recent_list(), Some("recent"));
    stack.add_named(&ui::options::home::build_home_view(), Some("home"));
    stack.add_named(&ui::options::downloads::build_downloads_view(), Some("downloads"));
    stack.add_named(&ui::options::documents::build_documents_view(), Some("documents"));
    stack.add_named(&ui::options::pictures::build_pictures_view(), Some("pictures"));
    stack.add_named(&build_file_list("Audio"), Some("audio"));
    stack.add_named(&build_file_list("Video"), Some("video"));
    stack.add_named(
        &ui::options::drives::build_drives_network_view(),
        Some("drives_network"),
    );
    stack.set_visible_child_name("recent");

    if let Some(row) = sidebar_list.row_at_index(1) {
        sidebar_list.select_row(Some(&row));
    }

    sidebar_list.connect_row_selected(clone!(@weak stack, @weak content_title => move |_list, row| {
        let Some(row) = row else { return; };
        let (name, title) = match row.index() {
            0 => return, // Search row
            1 => ("recent", "Recent"),
            2 => ("home", "Home"),
            3 => ("downloads", "Downloads"),
            4 => ("documents", "Documents"),
            5 => ("pictures", "Pictures"),
            6 => ("audio", "Audio"),
            7 => ("video", "Video"),
            _ => ("drives_network", "Drives & Network"),
        };
        stack.set_visible_child_name(name);
        content_title.set_text(title);
    }));

    let primary_pane = stack;
    let secondary_pane = build_file_list("Right pane");

    let paned = gtk::Paned::new(gtk::Orientation::Horizontal);
    paned.set_start_child(Some(&primary_pane));
    paned.set_end_child(Some(&secondary_pane));
    paned.set_position(520);

    split_toggle.connect_toggled(clone!(@weak secondary_pane => move |btn| {
        secondary_pane.set_visible(btn.is_active());
    }));
    secondary_pane.set_visible(false);

    let content_container = gtk::Box::new(gtk::Orientation::Vertical, 0);
    content_container.add_css_class("content-container");
    content_container.set_hexpand(true);
    content_container.set_vexpand(true);
    content_container.append(&paned);

    let status = gtk::Label::new(Some("Ready"));
    status.set_xalign(0.0);
    status.add_css_class("dim-label");
    status.set_margin_start(12);
    status.set_margin_bottom(8);
    status.set_margin_top(8);
    content_container.append(&status);

    let right_box = gtk::Box::new(gtk::Orientation::Vertical, 0);
    right_box.set_hexpand(true);
    right_box.set_vexpand(true);
    right_box.append(&content_header);
    right_box.append(&content_container);

    let content = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    content.append(&sidebar_box);
    
    let separator = gtk::Separator::new(gtk::Orientation::Vertical);
    content.append(&separator);
    
    content.append(&right_box);

    let main_box = gtk::Box::new(gtk::Orientation::Vertical, 0);
    main_box.append(&content);

    let window = adw::ApplicationWindow::new(app);
    window.set_default_size(1100, 720);
    window.set_title(Some("NAMO"));
    let root = gtk::Box::new(gtk::Orientation::Vertical, 0);
    root.append(&main_box);
    window.set_content(Some(&root));
    window.present();
}

fn load_css() {
    let provider = gtk::CssProvider::new();
    provider.load_from_data(
        ".flat-list, .flat-list row { background-color: transparent; }\n\
.flat-list row:selected { background-color: transparent; }\n\
.content-container { background-color: @view_bg_color; }\n\
.search-row { background-color: alpha(@accent_bg_color, 0.15); color: @accent_color; border-radius: 6px; }\n\
.sidebar-title, .content-title { font-weight: bold; }",
    );

    if let Some(display) = gtk::gdk::Display::default() {
        gtk::style_context_add_provider_for_display(
            &display,
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    }
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
    header_box.set_margin_start(12);
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
