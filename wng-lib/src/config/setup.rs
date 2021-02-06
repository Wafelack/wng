use std::path::Path;
use std::{fs, fs::File};
use std::{io, io::Write};

/// Warning: this function needs user input
pub fn setup(path: Option<&str>, version: &str) -> crate::Result<()> {
    println!("Welcome on wanager (v{}) setup", version);

    let mut cc = String::new();
    let mut name = String::new();
    let mut email = String::new();

    print!("Write the compiler that will be used by wanager: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut cc).unwrap();

    print!("Write your name: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut name).unwrap();

    print!("Write your email: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut email).unwrap();

    let home_dir = dirs::home_dir().unwrap();
    let fpath = format!("{}/wng.config", home_dir.to_str().unwrap());

    let config_file = path.map(|x| x.to_owned()).unwrap_or(fpath);

    cc = cc.trim().to_owned();
    name = name.trim().to_owned();
    email = email.trim().to_owned();

    if Path::new(&config_file).exists() {
        let mut res = String::new();
        print!("Your config file will be overwritten, do you want to continue [y/N] ? ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut res).unwrap();

        if res.to_uppercase().trim() != "Y" {
            return Ok(());
        }

        fs::remove_file(&config_file)?;
    }

    let mut file = File::create(&config_file)?;

    file.write_all(
        format!(
            "cc = \"{}\"\nname = \"{}\"\nemail = \"{}\"",
            cc, name, email
        )
        .as_bytes(),
    )?;

    println!("[+] Successfully written new configuration.");

    Ok(())
}
