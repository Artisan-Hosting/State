use artisan_middleware::{
    cli::get_user_input,
    config::AppConfig,
    dusa_collection_utils::{core::{logger::LogLevel, types::{pathtype::PathType, stringy::Stringy}}, log},
    state_persistence::StatePersistence,
};

#[tokio::main]
async fn main() {
    let state_name: Stringy = get_user_input("Application name : ");

    let spoofed_config = match AppConfig::new() {
        Ok(mut loaded_data) => {
            loaded_data.app_name = state_name;
            loaded_data
        }
        Err(e) => {
            log!(
                LogLevel::Error,
                "Failed to load the config: {}",
                e.to_string()
            );
            std::process::exit(0)
        }
    };

    let state_path: artisan_middleware::dusa_collection_utils::core::types::pathtype::PathType =
        PathType::Content(format!(
            "/opt/artisan/tmp/.{}.state",
            spoofed_config.app_name
        ));

    let state_data = match StatePersistence::load_state(&state_path).await {
        Ok(loaded_data) => loaded_data,
        Err(e) => {
            log!(
                LogLevel::Error,
                "Failed to load AppState data: {}",
                e.to_string()
            );
            std::process::exit(0)
        }
    };

    println!("{state_data}")
}
