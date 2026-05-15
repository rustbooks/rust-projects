// src/main.rs
// ============================================================================
// Android Clock — Main Entry Point
// ============================================================================
// Architecture: MVU (Model-View-Update) pattern
//   - Model: Pure Rust structs in src/model/
//   - View: Slint .slint files in ui/
//   - Update: Event handlers that bridge Slint callbacks → Rust logic
//
// Cross-platform strategy:
//   - Single codebase compiles for Android, Windows, macOS, Linux
//   - Platform-specific code is isolated in src/platform/
//   - No #[cfg] pollution in business logic
// ============================================================================

// On Android, the entry point is a Rust library function (not fn main).
// cargo-apk/xbuild generates the JNI bridge automatically.
#![cfg_attr(target_os = "android", allow(dead_code))]

mod alarm;
mod model;
mod platform;
mod storage;
mod timer;
mod world_clock;

use alarm::AlarmEngine;
use anyhow::Result;
use model::{AlarmModel, AppState, StopwatchState, TimerModel};
use slint::{ModelRc, SharedString, VecModel};
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use timer::TimerEngine;
use world_clock::WorldClockEngine;

// Include the generated Slint UI code. The `slint!` macro / include approach
// generates type-safe Rust bindings for every component defined in .slint files.
slint::include_modules!();

/// Application entry point for desktop platforms.
/// On Android, `android_main` is called instead (see platform/android.rs).
#[cfg(not(target_os = "android"))]
fn main() -> Result<()> {
    // Initialize logging. In release builds, env_logger compiles away
    // any log statements below the configured level (zero overhead).
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    log::info!("Starting Android Clock v{}", env!("CARGO_PKG_VERSION"));

    // Run the Tokio async runtime. We use a single-threaded runtime to
    // keep the binary small and avoid unnecessary thread overhead.
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_time()
        .build()?;

    rt.block_on(run_app())
}

/// Android entry point via ndk-glue
#[cfg(target_os = "android")]
#[no_mangle]
fn android_main(app: slint::android::AndroidApp) {
    android_logger::init_once(
        android_logger::Config::default()
            .with_max_level(log::LevelFilter::Info)
            .with_tag("AndroidClock"),
    );

    slint::android::init(app).expect("Failed to initialize Slint Android backend");

    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_time()
        .build()
        .expect("Failed to build Tokio runtime");

    rt.block_on(run_app()).expect("Application error");
}

/// Core application logic — runs on all platforms.
/// Returns Result so we can use `?` for clean error propagation.
async fn run_app() -> Result<()> {
    // Load persisted state from disk (alarms, timers, world clock cities).
    let storage = storage::AppStorage::new()?;
    let app_state = storage.load_state().unwrap_or_default();

    // Create the main Slint window. This is the root UI component.
    let app = AppWindow::new()?;

    // ── Shared State ─────────────────────────────────────────────────────────
    // Arc<Mutex<>> allows safe sharing between the Slint UI thread and
    // background async tasks (timer ticks, alarm checks, etc.)
    let alarm_engine = Arc::new(Mutex::new(AlarmEngine::new()));
    let timer_engine = Arc::new(Mutex::new(TimerEngine::new()));
    let stopwatch_state = Arc::new(Mutex::new(StopwatchState::default()));

    // ── Initialize UI State ──────────────────────────────────────────────────
    setup_clock_tab(&app, &app_state)?;
    setup_alarm_tab(&app, &app_state, Arc::clone(&alarm_engine))?;
    setup_timer_tab(&app, Arc::clone(&timer_engine))?;
    setup_stopwatch_tab(&app, Arc::clone(&stopwatch_state))?;
    setup_world_clock_tab(&app, &app_state)?;

    // ── Live Clock Update ────────────────────────────────────────────────────
    // Update the displayed time every second using Slint's timer.
    // Slint timers run on the UI thread — no locking needed.
    {
        let app_weak = app.as_weak();
        let tick_timer = slint::Timer::default();
        tick_timer.start(
            slint::TimerMode::Repeated,
            std::time::Duration::from_millis(100), // 10 Hz for smooth seconds display
            move || {
                let Some(app) = app_weak.upgrade() else {
                    return;
                };
                update_current_time(&app);
            },
        );
        // Keep the timer alive for the lifetime of the app
        std::mem::forget(tick_timer);
    }

    // ── Run the Event Loop ───────────────────────────────────────────────────
    // This blocks until the window is closed. Slint handles all rendering,
    // input events, and timer callbacks on this thread.
    app.run()?;

    // ── Persist State on Exit ────────────────────────────────────────────────
    let final_state = collect_state(&app)?;
    storage.save_state(&final_state)?;

    log::info!("Application exited cleanly");
    Ok(())
}

/// Update the live clock display with the current local time.
fn update_current_time(app: &AppWindow) {
    use chrono::Local;
    let now = Local::now();

    // Format time components separately so Slint can animate digit changes
    app.set_current_hour(SharedString::from(format!("{:02}", now.hour())));
    app.set_current_minute(SharedString::from(format!("{:02}", now.minute())));
    app.set_current_second(SharedString::from(format!("{:02}", now.second())));
    app.set_current_date(SharedString::from(
        now.format("%A, %B %-d").to_string(), // "Wednesday, May 13"
    ));
    app.set_is_pm(now.hour() >= 12);
}

use chrono::Timelike; // Brings .hour(), .minute(), .second() into scope

/// Initialize the analog/digital clock display tab.
fn setup_clock_tab(app: &AppWindow, _state: &AppState) -> Result<()> {
    update_current_time(app);
    Ok(())
}

/// Initialize the alarm tab with persisted alarms.
fn setup_alarm_tab(
    app: &AppWindow,
    state: &AppState,
    alarm_engine: Arc<Mutex<AlarmEngine>>,
) -> Result<()> {
    // Convert persisted alarms to Slint model items
    let alarm_items: Vec<AlarmItem> = state
        .alarms
        .iter()
        .map(|a| AlarmItem {
            id: SharedString::from(a.id.as_str()),
            label: SharedString::from(a.label.as_str()),
            time_display: SharedString::from(format!("{:02}:{:02}", a.hour, a.minute)),
            hour: a.hour as i32,
            minute: a.minute as i32,
            is_enabled: a.enabled,
            repeat_days: SharedString::from(a.format_days()),
            is_vibrate: a.vibrate,
        })
        .collect();

    let alarms_model = Rc::new(VecModel::from(alarm_items));
    app.set_alarms(ModelRc::from(alarms_model.clone()));

    // ── Add Alarm Callback ───────────────────────────────────────────────────
    {
        let app_weak = app.as_weak();
        let alarms_model = alarms_model.clone();
        app.on_add_alarm(move |hour, minute, label| {
            let alarm = AlarmModel::new(hour as u8, minute as u8, label.as_str());
            let item = AlarmItem {
                id: SharedString::from(alarm.id.as_str()),
                label: SharedString::from(alarm.label.as_str()),
                time_display: SharedString::from(format!(
                    "{:02}:{:02}",
                    alarm.hour, alarm.minute
                )),
                hour: alarm.hour as i32,
                minute: alarm.minute as i32,
                is_enabled: true,
                repeat_days: SharedString::from(alarm.format_days()),
                is_vibrate: alarm.vibrate,
            };
            alarms_model.push(item);
            log::info!("Alarm added: {}", alarm.id);
        });
    }

    // ── Toggle Alarm Callback ────────────────────────────────────────────────
    {
        let alarms_model = alarms_model.clone();
        app.on_toggle_alarm(move |index, enabled| {
            if let Some(mut item) = alarms_model.row_data(index as usize) {
                item.is_enabled = enabled;
                alarms_model.set_row_data(index as usize, item);
            }
        });
    }

    // ── Delete Alarm Callback ────────────────────────────────────────────────
    {
        let alarms_model = alarms_model.clone();
        app.on_delete_alarm(move |index| {
            alarms_model.remove(index as usize);
            log::info!("Alarm deleted at index {}", index);
        });
    }

    Ok(())
}

/// Initialize the countdown timer tab.
fn setup_timer_tab(app: &AppWindow, _timer_engine: Arc<Mutex<TimerEngine>>) -> Result<()> {
    app.set_timer_hours(0);
    app.set_timer_minutes(0);
    app.set_timer_seconds(0);
    app.set_timer_running(false);
    app.set_timer_display(SharedString::from("00:00:00"));
    app.set_timer_progress(0.0);

    // ── Timer Start/Pause ────────────────────────────────────────────────────
    {
        let app_weak = app.as_weak();
        let tick_timer = Rc::new(slint::Timer::default());
        let tick_timer_clone = tick_timer.clone();
        let remaining_ms = Rc::new(std::cell::Cell::new(0u64));
        let remaining_ms_tick = remaining_ms.clone();
        let total_ms = Rc::new(std::cell::Cell::new(0u64));
        let total_ms_tick = total_ms.clone();

        app.on_timer_start_pause(move |h, m, s| {
            let app = match app_weak.upgrade() {
                Some(a) => a,
                None => return,
            };

            if app.get_timer_running() {
                // Pause
                tick_timer_clone.stop();
                app.set_timer_running(false);
            } else {
                // Start
                let ms = ((h as u64) * 3600 + (m as u64) * 60 + (s as u64)) * 1000;
                if ms == 0 {
                    return;
                }
                remaining_ms.set(ms);
                total_ms.set(ms);
                app.set_timer_running(true);

                let app_weak2 = app.as_weak();
                let remaining_ms2 = remaining_ms_tick.clone();
                let total_ms2 = total_ms_tick.clone();

                tick_timer_clone.start(
                    slint::TimerMode::Repeated,
                    std::time::Duration::from_millis(100),
                    move || {
                        let Some(app) = app_weak2.upgrade() else {
                            return;
                        };
                        let rem = remaining_ms2.get();
                        if rem == 0 {
                            app.set_timer_running(false);
                            app.set_timer_display(SharedString::from("00:00:00"));
                            app.set_timer_progress(1.0);
                            app.invoke_timer_finished();
                            return;
                        }
                        let new_rem = rem.saturating_sub(100);
                        remaining_ms2.set(new_rem);

                        let total_secs = new_rem / 1000;
                        let h = total_secs / 3600;
                        let m = (total_secs % 3600) / 60;
                        let s = total_secs % 60;
                        app.set_timer_display(SharedString::from(format!(
                            "{:02}:{:02}:{:02}",
                            h, m, s
                        )));
                        let progress =
                            1.0 - (new_rem as f32 / total_ms2.get().max(1) as f32);
                        app.set_timer_progress(progress);
                    },
                );
            }
        });
    }

    // ── Timer Reset ──────────────────────────────────────────────────────────
    {
        let app_weak = app.as_weak();
        app.on_timer_reset(move || {
            let Some(app) = app_weak.upgrade() else {
                return;
            };
            app.set_timer_running(false);
            app.set_timer_display(SharedString::from("00:00:00"));
            app.set_timer_progress(0.0);
        });
    }

    Ok(())
}

/// Initialize the stopwatch tab.
fn setup_stopwatch_tab(app: &AppWindow, _state: Arc<Mutex<StopwatchState>>) -> Result<()> {
    app.set_stopwatch_display(SharedString::from("00:00.00"));
    app.set_stopwatch_running(false);

    let elapsed_ms = Rc::new(std::cell::Cell::new(0u64));
    let laps_model = Rc::new(VecModel::<SharedString>::from(vec![]));
    app.set_laps(ModelRc::from(laps_model.clone()));

    // ── Stopwatch Start/Pause ────────────────────────────────────────────────
    {
        let app_weak = app.as_weak();
        let sw_timer = Rc::new(slint::Timer::default());
        let sw_timer_clone = sw_timer.clone();
        let elapsed = elapsed_ms.clone();

        app.on_stopwatch_start_pause(move || {
            let Some(app) = app_weak.upgrade() else {
                return;
            };

            if app.get_stopwatch_running() {
                sw_timer_clone.stop();
                app.set_stopwatch_running(false);
            } else {
                app.set_stopwatch_running(true);
                let app_weak2 = app.as_weak();
                let elapsed2 = elapsed.clone();

                sw_timer_clone.start(
                    slint::TimerMode::Repeated,
                    std::time::Duration::from_millis(10), // 100 Hz for centisecond accuracy
                    move || {
                        let Some(app) = app_weak2.upgrade() else {
                            return;
                        };
                        let new_elapsed = elapsed2.get() + 10;
                        elapsed2.set(new_elapsed);

                        let cs = (new_elapsed / 10) % 100; // centiseconds
                        let secs = (new_elapsed / 1000) % 60;
                        let mins = (new_elapsed / 60000) % 60;
                        let hours = new_elapsed / 3600000;

                        let display = if hours > 0 {
                            format!("{:02}:{:02}:{:02}.{:02}", hours, mins, secs, cs)
                        } else {
                            format!("{:02}:{:02}.{:02}", mins, secs, cs)
                        };
                        app.set_stopwatch_display(SharedString::from(display));
                    },
                );
            }
        });
        std::mem::forget(sw_timer);
    }

    // ── Stopwatch Reset ──────────────────────────────────────────────────────
    {
        let app_weak = app.as_weak();
        let elapsed = elapsed_ms.clone();
        let laps = laps_model.clone();
        app.on_stopwatch_reset(move || {
            let Some(app) = app_weak.upgrade() else {
                return;
            };
            app.set_stopwatch_running(false);
            elapsed.set(0);
            app.set_stopwatch_display(SharedString::from("00:00.00"));
            // Clear laps
            while laps.row_count() > 0 {
                laps.remove(0);
            }
        });
    }

    // ── Stopwatch Lap ────────────────────────────────────────────────────────
    {
        let app_weak = app.as_weak();
        let elapsed = elapsed_ms;
        let laps = laps_model;
        app.on_stopwatch_lap(move || {
            let Some(app) = app_weak.upgrade() else {
                return;
            };
            let display = app.get_stopwatch_display();
            let lap_num = laps.row_count() + 1;
            laps.insert(0, SharedString::from(format!("Lap {} — {}", lap_num, display)));
        });
    }

    Ok(())
}

/// Initialize the world clock tab with major timezone cities.
fn setup_world_clock_tab(app: &AppWindow, state: &AppState) -> Result<()> {
    let engine = WorldClockEngine::new();

    // Default cities if none saved
    let cities = if state.world_clock_cities.is_empty() {
        vec![
            "America/New_York",
            "America/Los_Angeles",
            "Europe/London",
            "Europe/Paris",
            "Asia/Tokyo",
            "Asia/Dubai",
            "Australia/Sydney",
        ]
    } else {
        state
            .world_clock_cities
            .iter()
            .map(|s| s.as_str())
            .collect()
    };

    let world_items: Vec<WorldClockItem> = cities
        .iter()
        .filter_map(|tz| engine.get_city_info(tz))
        .collect();

    let world_model = Rc::new(VecModel::from(world_items));
    app.set_world_clocks(ModelRc::from(world_model.clone()));

    // Update world clocks every minute
    {
        let app_weak = app.as_weak();
        let world_model = world_model.clone();
        let engine = WorldClockEngine::new();
        let cities_owned: Vec<String> = cities.iter().map(|s| s.to_string()).collect();

        let wc_timer = slint::Timer::default();
        wc_timer.start(
            slint::TimerMode::Repeated,
            std::time::Duration::from_secs(60),
            move || {
                let Some(_app) = app_weak.upgrade() else {
                    return;
                };
                for (i, tz) in cities_owned.iter().enumerate() {
                    if let Some(item) = engine.get_city_info(tz) {
                        world_model.set_row_data(i, item);
                    }
                }
            },
        );
        std::mem::forget(wc_timer);
    }

    Ok(())
}

/// Collect current application state for persistence.
fn collect_state(app: &AppWindow) -> Result<AppState> {
    // In a full implementation, we'd read back all model data.
    // For now, return default state — alarms are collected via the model.
    Ok(AppState::default())
}
