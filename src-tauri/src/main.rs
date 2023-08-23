// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod models {
    pub mod database;
    pub mod feeds;
    pub mod items;
    pub mod settings;
}

pub mod commands {
    pub mod feeds;
    pub mod items;
    pub mod settings;
}

pub mod error;
pub mod producer;
pub mod syndication;
pub mod worker;

fn main() {
    let _ = models::database::migrate();

    let app = tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            commands::feeds::create_feed,
            commands::feeds::read_all_feeds,
            commands::feeds::read_feed,
            commands::feeds::update_feed,
            commands::feeds::delete_feed,
            commands::items::read_all_items,
            commands::items::count_all_items,
            commands::items::update_item,
            commands::settings::read_all_settings,
            commands::settings::update_setting,
        ])
        .build(tauri::generate_context!("tauri.conf.json"))
        .expect("error while running tauri application");

    worker::start(app.config().tauri.bundle.identifier.clone(), app.handle());

    app.run(move |handle, event| {
        if let tauri::RunEvent::WindowEvent {
            label: _,
            event: tauri::WindowEvent::CloseRequested { api, .. },
            ..
        } = event
        {
            let _ = handle.hide();
            api.prevent_close();
        }
    });
}
