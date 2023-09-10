use std::fs;
use tauri::{Manager, State};
//use std::path::Path;

struct MyString(String);

// Prevents additional console window on Windows in release, DO NOT REMOVE!!
//#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[tauri::command]
fn string_command<'r>(state: State<'r, MyString>) -> String {
    return format!("state: {}", state.inner().0);
}


fn main() {
  let mut sql: String = Default::default();
  tauri::Builder::default()
    .setup(move |app| {
      match app.get_cli_matches() {
        Ok(matches) => {
          println!("{:?}", matches);
          println!("{:?}", matches.args["sql file"]);
          println!("{}", matches.args["sql file"].value.to_string().as_str());

          println!("{:?}", matches.args["sql file"].value.as_str());
          let filename: String = matches.args["sql file"].value.to_string();
          let mut chars = filename.chars();
          chars.next();
          chars.next_back();

          println!("{}", chars.as_str());
          sql = fs::read_to_string(chars.as_str()).expect("Unable to read file");
          sql = format!(" {}", sql); // temp fix for next() skipping first char
          println!("{}", sql);

          let mut tokens = Vec::<sqerli::tokenize::Token>::new();
          let mut sql_chars = sql.chars().peekable();
          sqerli::tokenize::tokenize(&mut sql_chars, &mut tokens);
//        app.manage(MyString(sql_chars.collect::<String>()));
          app.manage(MyString(sqerli::er_html::er_html(tokens.to_vec())));




          //app.manage(MyString(&sql_chars.to_string()))
          // app.manage(MyString("converted from tokens to html"));
          // app.manage(MyString(tokenize(&mut sql).iter().nth(0).expect("REASON").to_string()));
        }
        Err(_) => {}
      }
      Ok(())
    })
    .invoke_handler(tauri::generate_handler![string_command])
    //.invoke_handler(tauri::generate_handler![greet])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
