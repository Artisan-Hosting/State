use artisan_middleware::{
    cli::get_user_input, config::AppConfig, log, logger::LogLevel,
    state_persistence::StatePersistence,
};

fn main() {
    let state_name: dusa_collection_utils::stringy::Stringy = get_user_input("Application name : ");

    let spoofed_config = match AppConfig::new() {
        Ok(mut loaded_data) => {
            loaded_data.app_name = state_name.to_string();
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

    let state_data = match StatePersistence::load_state(&state_path) {
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

    print!("{state_data}")
}
