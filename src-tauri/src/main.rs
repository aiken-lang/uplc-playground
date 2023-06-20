// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use uplc::{
    ast::{Name, NamedDeBruijn, Program},
    machine::cost_model::ExBudget,
};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn evaluate(code: &str) -> String {
    let program_str = format!("(program 1.0.0 {code})");
    let result = uplc::parser::program(&program_str)
        .map_err(|e| e.to_string())
        .and_then(|program| {
            let program: Program<NamedDeBruijn> = program.try_into().unwrap();

            program
                .eval(ExBudget::default())
                .result()
                .map_err(|e| e.to_string())
        })
        .and_then(|term| {
            let program: Result<Program<Name>, _> = Program {
                version: (1, 0, 0),
                term,
            }
            .try_into();

            program.map_err(|e| e.to_string())
        });

    match result {
        Ok(program) => program.term.to_pretty(),
        Err(err) => err,
    }
}

#[tauri::command]
fn format(code: &str) -> Result<String, String> {
    uplc::parser::term(code)
        .map(|term| term.to_pretty())
        .map_err(|e| e.to_string())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![evaluate, format])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
