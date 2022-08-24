use std::{ fs, io::Read, process, env, collections::HashMap };
use serde::{ Deserialize, Serialize };
use colored::Colorize;
use dialoguer::{ theme::ColorfulTheme, Select };
use serde_json::{ Value, json };
use tabled::Tabled;
extern crate dirs;

pub fn get_config_file() -> fs::File {
    let path: String = dirs::home_dir().unwrap().to_str().unwrap().to_string() + "/seekconfig.json";

    if fs::metadata(&path).is_ok() {
        return fs::File::open(&path).expect("Failed to open config file");
    } else {
        let file = fs::File::create(&path).expect("Failed to create config file");
        return file;
    }
}

pub fn check_path(path: &String) -> bool {
    fs::metadata(path).is_ok()
}

pub fn change_app(app: &String, path: &String) {

    let mut p = String::new();
    if path.starts_with("./") || path.starts_with("/") || path.starts_with(r#".\"#) || path.starts_with(r#"\"#) {
        p = env::current_dir().unwrap().to_str().unwrap().to_string();
        p.push_str(r#"\"#);
        p.push_str(path);
        
        if check_path(&p) == false {
            println!("{}", format!("Path - `{}` does not exist.", &p).red());
            process::exit(1);
        }
    } else {
        p = path.to_string();
    }
    
    let config: fs::File = get_config_file();

    let mut data = String::new();

    (&config).read_to_string(&mut data).unwrap();

    if data.len() < 1 {
        data = r#"{"paths": {}, "apps": {}}"#.to_string();
    }

    let mut v: Value = match serde_json::from_str(&data) {
        Ok(v) => v,
        Err(_) => {
            data = r#"{"paths": {}, "apps": {}}"#.to_string();
            serde_json::from_str(&data).unwrap()
        }
    };
    
    if (v["apps"])[app].is_null() == false {
        
        println!("An app with the name `{}` already exists!", app);

        let opts = vec!["Yes", "No"];
        if Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Do you want to update it?")
            .items(&opts)
            .default(1)
            .interact()
            .unwrap()
            == 0
        {
            (v["apps"])[app] = json!(p);
            let data = serde_json::to_string(&v).unwrap();
            fs::write(
                dirs::home_dir().unwrap().to_str().unwrap().to_string() + "/seekconfig.json",
                data,
            )
            .expect("Failed to write to file");
            println!("Updated `{}` with `{}`!", app, p);
        } else {
            process::exit(0);
        }
    } else {
        v["apps"][app] = json!(p);
        let data = serde_json::to_string(&v).unwrap();
        fs::write(
            dirs::home_dir().unwrap().to_str().unwrap().to_string() + "/seekconfig.json",
            data,
        )
        .expect("Failed to write to file");
        println!("Added an app named `{}` with `{}` path!", app, p);
    }
    
}

pub fn change_path(name: &String, path: &String) {

    let mut p = String::new();
    if path.starts_with("./") || path.starts_with("/") || path.starts_with(r#".\"#) || path.starts_with(r#"\"#) {
        p = env::current_dir().unwrap().to_str().unwrap().to_string();
        p.push_str(r#"\"#);
        p.push_str(path);
        
        if check_path(&p) == false {
            println!("{}", format!("Path - `{}` does not exist.", &p).red());
            process::exit(1);
        }
    } else {
        p = path.to_string();
    }

    let config: fs::File = get_config_file();

    let mut data = String::new();

    (&config).read_to_string(&mut data).unwrap();

    if data.len() < 1 {
        data = r#"{"paths": {}, "apps": {}}"#.to_string();
    }

    let mut v: Value = match serde_json::from_str(&data) {
        Ok(v) => v,
        Err(_) => {
            data = r#"{"paths": {}, "apps": {}}"#.to_string();
            serde_json::from_str(&data).unwrap()
        }
    };

    if v["paths"][name].is_null() == false {
        
        println!("A path shortcut with the name `{}` already exists!", name);

        let opts = vec!["Yes", "No"];
        if Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Do you want to update it?")
            .items(&opts)
            .default(1)
            .interact()
            .unwrap()
            == 0
        {
            v["paths"][name] = json!(p);
            let data = serde_json::to_string(&v).unwrap();
            fs::write(
                dirs::home_dir().unwrap().to_str().unwrap().to_string() + "/seekconfig.json",
                data,
            )
            .expect("Failed to write to file");
            println!("Updated `{}` path shortcut with `{}`!", name, p);
        } else {
            process::exit(0);
        }
    } else {
        v["paths"][name] = json!(p);
        let data = serde_json::to_string(&v).unwrap();
        fs::write(
            dirs::home_dir().unwrap().to_str().unwrap().to_string() + "/seekconfig.json",
            data,
        )
        .expect("Failed to write to file");
        println!("Added a path shortcut named `{}` with `{}` path!", name, p);
    }
    
}

pub fn help_msg() {
    println!(r#"
{} {}

{}
    seek {}

{}
    {}
    {}

    seek --app <APP> <PATH>
    seek --config <NAME> <FULLPATH>

{}
    seek --app  discord ../path/to/discord/executable
    seek --app  code    code-insiders.cmd
    seek --path seek    ../path/to/seek/code/directory
    
    seek code seek
    {}

    seek code --custom ./a/different/path/which/is/not/saved

Visit the repo for more help - {}

{} {}
    "#, 
        "seek".green(), 
        env!("CARGO_PKG_VERSION"), 
        "USAGE:".yellow(),
        "<APP> [PATH]".dimmed(),
        "CONFIGURATION:".yellow(),
        "# Adding/Updating an app shortcut".bright_black(),
        "# Adding/Updating an path shortcut".bright_black(),
        "EXAMPLES:".yellow(),
        "# Opens the app `code` with path `seek`".bright_black(),
        "https://github.com/yxshv/seek".cyan().underline(),
        "<REQUIRED>".bold(), "[OPTIONAL]".dimmed(),
    )
}

pub fn get_app(app: &String) -> Result<String, String> {
    let config: fs::File = get_config_file();

    let mut data = String::new();

    (&config).read_to_string(&mut data).unwrap();

    if data.len() < 1 {
        data = r#"{"paths": {}, "apps": {}}"#.to_string();
    }
    let v: Value = match serde_json::from_str(&data) {
        Ok(v) => v,
        Err(_) => {
            data = r#"{"paths": {}, "apps": {}}"#.to_string();
            serde_json::from_str(&data).unwrap()
        }
    };
    if v["apps"][app].is_null() == false {
        Ok(v["apps"][app].as_str().unwrap().to_string())
    } else {
        Err(format!("App `{}` does not exist!", app))
    }
}

pub fn get_path(path: &String) -> Result<String, String> {
    let config: fs::File = get_config_file();

    let mut data = String::new();

    (&config).read_to_string(&mut data).unwrap();

    if data.len() < 1 {
        data = r#"{"paths": {}, "apps": {}}"#.to_string();
    }
    let v: Value = match serde_json::from_str(&data) {
        Ok(v) => v,
        Err(_) => {
            data = r#"{"paths": {}, "apps": {}}"#.to_string();
            serde_json::from_str(&data).unwrap()
        }
    };
    if v["paths"][path].is_null() == false {
        Ok(v["paths"][path].as_str().unwrap().to_string())
    } else {
        Err(format!("No path short exists with the name - `{}`!", path))
    }
}

#[derive(Tabled)]
pub struct Apps {
    #[tabled(rename = "Name")]
    names: String,
    #[tabled(rename = "Path")]
    paths: String
}

#[derive(Tabled)]
pub struct Path {
    #[tabled(rename = "Name")]
    names: String,
    #[tabled(rename = "Shortcut")]
    paths: String
}

#[derive(Serialize, Deserialize)]
pub struct Object {
    apps:  HashMap<String, String>,
    paths: HashMap<String, String>
}

pub fn apps_list() -> Vec<Apps> {

    let mut apps: Vec<Apps> = Vec::new();
    
    let config: fs::File = get_config_file();
    let mut content: String = String::new();

    (&config).read_to_string(&mut content).unwrap();

    if content.len() < 1 {
        content = r#"{"paths": {}, "apps": {}}"#.to_string();
    }
    let v: Object = match serde_json::from_str(&content) {
        Ok(v) => v,
        Err(_) => {
            content = r#"{"paths": {}, "apps": {}}"#.to_string();
            serde_json::from_str(&content).unwrap()
        }
    };

    for (k, v) in v.apps {
        apps.push(Apps {
            names: k,
            paths: v
        });
    }

    apps

}

pub fn paths_list() -> Vec<Apps> {

    let mut apps: Vec<Apps> = Vec::new();
    
    let config: fs::File = get_config_file();
    let mut content: String = String::new();

    (&config).read_to_string(&mut content).unwrap();

    if content.len() < 1 {
        content = r#"{"paths": {}, "apps": {}}"#.to_string();
    }
    let v: Object = match serde_json::from_str(&content) {
        Ok(v) => v,
        Err(_) => {
            content = r#"{"paths": {}, "apps": {}}"#.to_string();
            serde_json::from_str(&content).unwrap()
        }
    };

    for (k, v) in v.paths {
        apps.push(Apps {
            names: k,
            paths: v
        });
    }

    apps

}