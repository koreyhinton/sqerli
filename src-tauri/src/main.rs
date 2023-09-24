use std::fs;
use tauri::{Manager, State};
//use std::path::Path;
use std::collections::HashMap;
use im::HashMap as ImMap;
use tauri::api::cli::ArgData;

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

fn cli_path_clean(path: String) -> String {
  let str = path;
  let mut chars = str.chars();
  { // remove surrounding quotes: ""
    chars.next();
    chars.next_back();
  }
  return chars.collect();  
}

fn cli_arg_sql_file(args: &HashMap<String, ArgData>) -> String {
  println!("{:?}", args["sql file"]);
  let path = args["sql file"].value.to_string();
  cli_path_clean(path)
}

fn cli_arg_o(args: &HashMap<String, ArgData>) -> Option<String> {
  println!("{:?}", args["output"]);
  if args["output"].occurrences == 1 {
    let path = cli_path_clean(args["output"].value.to_string());
    return Some(path)
  }
  None
}

fn opt_out_f_write(output_file: Option<String>, html: &String) {
  match output_file {
    Some(output_filename) => {
      fs::write(output_filename, html).expect("Unable to write to file");
    },
    None => ()
  }
}

fn main() {
  let mut sql: String = Default::default();
  tauri::Builder::default()
    .setup(move |app| {
      match app.get_cli_matches() {
        Ok(matches) => {

          // get cli args
          let filename = cli_arg_sql_file(&matches.args); // positional required
          let output_file = cli_arg_o(&matches.args); // named optional

          let ddl_path = filename;
          //println!("{:?}",ddl_path);
          sql = fs::read_to_string(ddl_path).expect("Unable to read file");
          // sql = format!(" {}", sql); // temp fix for next() skipping first char
          println!("{}", sql);

          let mut tokens = Vec::<sqerli::tokenize::Token>::new();
          let mut sql_chars = sql.chars().peekable();
          sqerli::tokenize::tokenize(&mut sql_chars, &mut tokens);
//        app.manage(MyString(sql_chars.collect::<String>()));

          let html = sqerli::er_html::er_html(tokens.to_vec());

          opt_out_f_write(output_file, &html);
          app.manage(MyString(html));




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
