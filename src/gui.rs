use anyhow::Result;
use fltk::{prelude::*, *};
use log::info;

pub fn run_app() -> Result<()> {
    let app = app::App::default();
    let mut wind = window::Window::default()
        .with_size(1000, 700)
        .with_label("Rusty Permission Changer");

    // Create main vertical layout
    let mut col = group::Flex::default()
        .with_size(1000, 700)
        .column();

    // Button row
    let mut btn_row = group::Flex::default().with_type(group::FlexType::Row);
    btn_row.set_size(&wind, 60);
    
    let _file_btn = button::Button::default().with_label("Select Files 📁");
    let _folder_btn = button::Button::default().with_label("Select Folders 📂");
    let _check_btn = button::Button::default().with_label("Check Ownership 🔍");
    let _change_btn = button::Button::default().with_label("Change Permissions 🔧");
    let _revert_btn = button::Button::default().with_label("Revert Changes ↩️");
    
    btn_row.end();

    // Header label
    let _header = text::TextDisplay::default().with_label("Ownership Status:");

    // List widget for ownership display
    let _list = browser::FileBrowser::default();

    // Log display label
    let _log_label = text::TextDisplay::default().with_label("Log Output:");

    // Log display
    let _log_display = text::TextEditor::default();

    // Counters row
    let mut counter_row = group::Flex::default().with_type(group::FlexType::Row);
    counter_row.set_size(&wind, 30);
    let _changed_label = text::TextDisplay::default().with_label("Changed: 0");
    let _unchanged_label = text::TextDisplay::default().with_label("Unchanged: 0");
    let _errors_label = text::TextDisplay::default().with_label("Errors: 0");
    counter_row.end();

    // Progress bar
    let _progress = misc::ProgressBar::default();

    col.end();
    wind.end();
    wind.show();

    info!("GUI initialized successfully");

    Ok(app.run()
        .map_err(|e| anyhow::anyhow!("Application error: {}", e))?)
}
