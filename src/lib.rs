use std::{
    env,
    error::Error,
    fs::{self, File, OpenOptions},
    path::Path, io::{Write, self, Read},
};

pub struct Config {
    name: String,
    username: String,
    password: String,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();
        let name = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a name"),
        };
        let username = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a username"),
        };
        let password = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a password"),
        };
        Ok(Config {
            name,
            username,
            password,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!(
        "Saving {}, {}, {}",
        config.name, config.username, config.password
    );

    let file = get_config_file()?;

    let contents = read(&file);

    save(config, &file);

    Ok(())
}

fn get_config_file() -> io::Result<File> {
    let path = String::from("/Users/jing/Downloads/.kpw/");
    let dir = Path::new(&path);
    let exists = dir.exists();
    if !exists {
        match fs::create_dir(dir) {
            Ok(()) => true,
            Err(e) => panic!("Coludn't create directory: {}", e),
        };
    }
    let path = path + "pass";
    let file = match OpenOptions::new()
        .create(true)
        .read(true)
        .append(true)
        .open(path) {
        Ok(file) => file,
        Err(e) => panic!("Error: {}", e),
    };
    Ok(file)
}

fn save(config: Config, mut file: &File) {
    let contents = read(file);
    let line = (config.name + " " + &config.username + " " + &config.password).to_lowercase();

    for item in contents.lines() {
        println!("item: {}, line: {}", item, line.to_lowercase());
        if line.contains(&item.to_lowercase()) {
            return
        }
    }
    file.write((line + "\n").as_bytes());
}

fn read(mut file: &File) -> String {
    let mut contents = String::from("");
    file.read_to_string(&mut contents);
    contents
}

fn copy_to_clipboard() {}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_save() {
        let result = get_config_file();
        let file = result.unwrap();
        save(Config {
            name: String::from("dilibili"),
            username: String::from("jing"),
            password: String::from("123456"),
        }, &file);
    }

    #[test]
    fn test_get_config_file() {
        get_config_file();
    }
}
