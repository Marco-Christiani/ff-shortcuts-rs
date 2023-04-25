extern crate serde_json;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Command {
    #[serde(rename = "precedenceList")]
    precedence_list: Vec<Extension>,
}

#[derive(Serialize, Deserialize)]
struct Extension {
    id: String,
    #[serde(rename = "installDate")]
    install_date: u64,
    value: Value,
    enabled: bool,
}

#[derive(Serialize, Deserialize)]
struct Value {
    shortcut: String,
}

fn main() -> std::io::Result<()> {
    println!("Reading file");
    let home_dir = match env::var("HOME") {
        Ok(val) => val,
        Err(_) => panic!("Unable to read HOME environment variable"),
    };
    let settings_file_path = format!("{}/.mozilla/firefox/xv2q0hk1.default-release/extension-settings.json", home_dir);
    let mut file = File::open(settings_file_path)?;
    println!("Processing");
    let mut output = String::from("<html><head><style>");
    // inject styles to save eyes
    output.push_str("
        body {
            background-color: #1E1E1E;
            color: #F8F8F8;
            font-family: sans-serif;
        }
        div {
            width: 100%;
            display: flex;
            justify-content: center;
            padding-top: 3em;
        }
        table {
            border-collapse: collapse;
        }
        th, td {
            padding: 8px;
            text-align: left;
            border-bottom: 1px solid #ddd;
        }
        th {
            background-color: #333;
            color: #F8F8F8;
        }
    ");
    output.push_str("</style></head><body><div>");
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let commands: serde_json::Value = serde_json::from_str(&contents)?;

    let mut table = String::new();
    table.push_str("<table>");
    table.push_str("<tr><th>Command Name</th><th>Shortcut</th><th>Extension</th><th>Enabled</th></tr>");
    for (key, value) in commands["commands"].as_object().unwrap() {
        let command = serde_json::from_value::<Command>(value.clone()).unwrap();
        table.push_str("<tr>");
        table.push_str(&format!("<td>{}</td>", key));
        table.push_str(&format!("<td>{}</td>", command.precedence_list[0].value.shortcut));
        table.push_str(&format!("<td>{}</td>", command.precedence_list[0].id));
        table.push_str(&format!("<td>{}</td>", command.precedence_list[0].enabled));
        table.push_str("</tr>");
    }
    table.push_str("</table>");
    output.push_str(&table);
    output.push_str("</div></body></html>");

    let mut output_file = File::create("output.html")?;
    output_file.write_all(output.as_bytes())?;
    println!("Done");

    Ok(())
}
