use artisan_middleware::{
    cli::get_user_input, config::AppConfig, dusa_collection_utils::{log, core::logger::LogLevel, core::types::stringy::Stringy}, state_persistence::StatePersistence
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

    let state_path = StatePersistence::get_state_path(&spoofed_config);

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
