use std::sync::Mutex;

use tauri::State;
use tnoodle_rs::Scramble;

use crate::{models::Solve, AppState};

#[tauri::command]
pub(crate) fn get_scramble(state: State<'_, Mutex<AppState>>) -> Scramble {
    let state = state.lock().unwrap();
    state.scrambler.generate_wca_scramble().unwrap()
}

#[tauri::command]
pub(crate) fn record_solve(solve: Solve, state: State<'_, Mutex<AppState>>) {
    todo!()
}
