pub mod menu;
pub mod config;

pub mod cheat;

use eframe::egui;
use nvidia_overlay::core::Overlay;

use std::{
    thread,
    sync::{
        Arc,
        RwLock,
        atomic::AtomicBool,
    },
    time::Duration,
};
use std::sync::atomic::Ordering;
use crate::cheat::{
    features::feature_manager::FeatureManager,
    process::Process,
    sdk::{
        GameState,
        entity_list::EntityList,
        player::PlayerManager,
    }
};
use crate::menu::{
    gui::Menu,
    utils::get_random_window_name,
};
use crate::config::Config;

// Base pointer
pub const LOCAL_PLAYER: u32 = 0x0017E0A8;

fn main() -> eframe::Result {
    env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .format_target(false)
        .format_timestamp_secs()
        .init();

    let mut overlay = Overlay::new("Calibri", 12.0);
    overlay.init().expect("Failed to initialize overlay");
    overlay.startup_d2d().expect("Failed to startup D2D for overlay");

    log::info!("Successfully hijacked Nvidia overlay!");

    // Load config from file or create default
    let config = Arc::new(RwLock::new(Config::load().unwrap_or_else(|e| {
        log::error!("Failed to load config: {:?}, using default", e);
        Config::default()
    })));
    let config_overlay_thread = config.clone();

    // Gather info for game update thread
    // Process initialization
    let process = Process::new("ac_client.exe");

    // Local player initialization
    let local_player_ptr = process.read::<u32>(process.base_address + LOCAL_PLAYER).unwrap();
    let local_player = PlayerManager::new(process.clone(), local_player_ptr).unwrap();

    // Entity list initialization
    let entity_list = EntityList::new(process.clone());

    // Game context initialization
    let game_context = Arc::new(RwLock::new(
        GameState::new(process.clone(), local_player, entity_list)));
    let game_context_game_update_thread = game_context.clone();
    let game_context_overlay_thread = game_context.clone();

    // Feature manager initialization
    let mut feature_manager = FeatureManager::new(
        overlay,
        game_context_overlay_thread,
        config_overlay_thread);

    // Atomic bools allow for easy cleanup because it is simpler to determine when the app is running
    let process_running = Arc::new(AtomicBool::new(true));
    let running_game_update_thread = process_running.clone();
    let running_overlay_thread = process_running.clone();

    // TODO! Make a handler in the future in case more needs to be updated!
    // START GAME UPDATE THREAD
    thread::spawn(move || {
        while running_game_update_thread.load(Ordering::Relaxed) {
            game_context_game_update_thread.write().unwrap().update();
            thread::sleep(Duration::from_nanos(1));
        }
        // All resources automatically dropped
        log::debug!("Exited game update thread");
    });

    // START OVERLAY THREAD
    thread::spawn(move || {
        // Test FPS with and without this check in the future!
        while running_overlay_thread.load(Ordering::Relaxed) {
            feature_manager.tick().expect("Feature manager failed!");
            thread::sleep(Duration::from_nanos(1));
        }
        // Overlay needs to be explicitly dropped
        feature_manager.cleanup();
        log::debug!("Exited overlay thread");
    });
    
    // Randomize app name
    let window_name = get_random_window_name(4, "wakey wakey", 6);

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([320.0, 240.0])
            .with_title(window_name),
        vsync: true,
        run_and_return: false,
        persist_window: true,
        centered: true,
        ..Default::default()
    };

    // Fixed name for state storage
    let storage_name = "wakey wakey";


    // Instead of run_simple_native, use run_native
    eframe::run_native(
        storage_name,
        options,
        Box::new(|cc| {
            Ok(Box::new(Menu::new(config, process_running, cc)))
        }),
    )
}
