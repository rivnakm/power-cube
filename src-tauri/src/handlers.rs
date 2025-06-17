use tauri::State;
use tnoodle_rs::Scramble;

use tokio::sync::Mutex;

use crate::{
    AppState,
    db::{Repository, solves::SolveRepository},
    models::Solve,
};

#[tauri::command]
pub(crate) async fn get_scramble(state: State<'_, Mutex<AppState>>) -> Result<Scramble, ()> {
    let state = state.lock().await;
    Ok(state.scrambler.generate_wca_scramble().unwrap())
}

#[tauri::command]
pub(crate) async fn record_solve(
    solve: Solve,
    state: State<'_, Mutex<AppState>>,
) -> Result<i64, ()> {
    let state = state.lock().await;
    let mut conn = state.db_pool.acquire().await.unwrap();

    let mut repo = SolveRepository::default();
    repo.insert(&mut *conn, &solve).await.map_err(|_| ())
}

#[tauri::command]
pub(crate) async fn get_all_solves(state: State<'_, Mutex<AppState>>) -> Result<Vec<Solve>, ()> {
    let state = state.lock().await;
    let mut conn = state.db_pool.acquire().await.unwrap();

    let repo = SolveRepository::default();
    repo.get_all(&mut *conn).await.map_err(|_| ())
}

#[tauri::command]
pub(crate) async fn delete_solve(id: i64, state: State<'_, Mutex<AppState>>) -> Result<(), ()> {
    let state = state.lock().await;
    let mut conn = state.db_pool.acquire().await.unwrap();

    let mut repo = SolveRepository::default();
    repo.delete(&mut *conn, id).await.map_err(|_| ())
}

// TODO: is there a way to use serde_as instead of returning an i64 here?
#[tauri::command]
pub(crate) async fn get_avg_of_n(n: u32, state: State<'_, Mutex<AppState>>) -> Result<i64, ()> {
    let state = state.lock().await;
    let mut conn = state.db_pool.acquire().await.unwrap();

    let repo = SolveRepository::default();
    repo.get_avg_of_n(&mut *conn, n)
        .await
        .map_err(|_| ())
        .map(|td| td.num_milliseconds())
}

#[tauri::command]
pub(crate) async fn get_best_time(state: State<'_, Mutex<AppState>>) -> Result<Option<i64>, ()> {
    let state = state.lock().await;
    let mut conn = state.db_pool.acquire().await.unwrap();

    let repo = SolveRepository::default();
    repo.get_best_time(&mut *conn)
        .await
        .map_err(|_| ())
        .map(|td_opt| td_opt.map(|td| td.num_milliseconds()))
}

#[tauri::command]
pub(crate) async fn get_best_avg_of_n(
    n: u32,
    state: State<'_, Mutex<AppState>>,
) -> Result<i64, ()> {
    let state = state.lock().await;
    let mut conn = state.db_pool.acquire().await.unwrap();

    let repo = SolveRepository::default();
    repo.get_best_avg_of_n(&mut *conn, n)
        .await
        .map_err(|_| ())
        .map(|td| td.num_milliseconds())
}
