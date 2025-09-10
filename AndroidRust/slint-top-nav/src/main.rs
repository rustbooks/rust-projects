use slint::{Model, ModelRc, VecModel};
use std::rc::Rc;
use rusqlite::{Connection, Result};
use log::info;

slint::include_modules!();

/* mod theme;
mod strings;
mod colors;
mod components;
*/
use theme::AppTheme;
use strings::Strings;
use colors::Colors;
use components::{MenuItem, FormData};

fn init_database() -> Result<Connection> {
    let conn = Connection::open_in_memory()?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS form_data (
            id INTEGER PRIMARY KEY,
            menu TEXT NOT NULL,
            input1 TEXT NOT NULL,
            input2 TEXT NOT NULL
        )",
        [],
    )?;
    Ok(conn)
}

fn main() -> Result<(), slint::PlatformError> {
    let app = AppWindow::new()?;
    
    let theme = AppTheme::new();
    let strings = Strings::new();
    
    // Initialize database
    let conn = init_database().expect("Failed to initialize database");
    
    // Menu items with full icon paths
    let menu_items = Rc::new(VecModel::from(vec![
        MenuItem {
            name: "Home".into(),
            icon: "assets/icons/home.svg".into(),
        },
        MenuItem {
            name: "Profile".into(),
            icon: "assets/icons/profile.svg".into(),
        },
        MenuItem {
            name: "Settings".into(),
            icon: "assets/icons/settings.svg".into(),
        },
    ]));
    
    app.global::<AppLogic>().set_menu_items(ModelRc::from(menu_items.clone()));
    app.global::<AppLogic>().set_current_menu(0);
    
    // Theme properties
    app.global::<Theme>().set_primary_color(Colors::PRIMARY.into());
    app.global::<Theme>().set_accent_color(Colors::ACCENT.into());
    app.global::<Theme>().set_background_color(Colors::BACKGROUND.into());
    
    // Strings
    app.global::<Strings>().set_app_name(strings.app_name.into());
    app.global::<Strings>().set_submit_button(strings.submit_button.into());
    
    // Handle form submission
    let app_weak = app.as_weak();
    app.global::<AppLogic>().on_submit(move |menu, input1, input2| {
        let app = app_weak.unwrap();
        match conn.execute(
            "INSERT INTO form_data (menu, input1, input2) VALUES (?1, ?2, ?3)",
            [&menu, &input1, &input2],
        ) {
            Ok(_) => {
                app.global::<AppLogic>().set_popup_message("Data saved successfully!".into());
                app.global::<AppLogic>().set_popup_visible(true);
            }
            Err(e) => {
                app.global::<AppLogic>().set_popup_message(format!("Error: {}", e).into());
                app.global::<AppLogic>().set_popup_visible(true);
            }
        }
    });
    
    // Handle menu selection
    let app_weak = app.as_weak();
    app.global::<AppLogic>().on_menu_selected(move |index| {
        app_weak.unwrap().global::<AppLogic>().set_current_menu(index);
    });
    
    // Handle popup close
    let app_weak = app.as_weak();
    app.global::<AppLogic>().on_close_popup(move || {
        app_weak.unwrap().global::<AppLogic>().set_popup_visible(false);
    });
    
    app.run()
}