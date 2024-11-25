#[cfg(feature= "gui")]
mod gui;
mod broker;
mod db;
mod audio_player;
mod api;
mod connection_pool;
mod error;
mod config;
mod import;
mod lib;
mod cli;

use std::io::IsTerminal;
use std::path::PathBuf;
use clap::{arg, Parser, Subcommand};
use crate::db::load_all_sounds;
use crate::Commands::Import;
use crate::config::{init_app, DirConfig};
// use crate::db::run_migration;
use crate::import::import_sounds;
use crate::cli::run;

#[derive(Parser, Debug)]
#[clap(name = "soundshouter", version = "0.1.0", about=r#"
| ================================================================================ |
|   ____                            _       _                    _                 |
|  / ___|   ___   _   _  _ __    __| | ___ | |__    ___   _   _ | |_   ___  _ __   |
|  \___ \  / _ \ | | | || '_ \  / _` |/ __|| '_ \  / _ \ | | | || __| / _ \| '__|  |
|   ___) || (_) || |_| || | | || (_| |\__ \| | | || (_) || |_| || |_ |  __/| |     |   
|  |____/  \___/  \__,_||_| |_| \__,_||___/|_| |_| \___/  \__,_| \__| \___||_|     |
| ================================== v0.1.0 ====================================== |
"#)]
struct Args {
    #[command(subcommand)]
    cmd: Option<Commands>,
}

#[derive(Subcommand, Debug)]
#[clap(name = "import")]
enum Commands {
    /// import sounds
    Import {
        #[arg(help = "directory with sound files to import")]
        path: PathBuf
    },
    /// list all available sounds
    List,
    /// run gui
    #[cfg(feature = "gui")]
    Gui,
    Serve,
}

fn main() {
    let args = Args::parse();
    println!("{args:?}");

    let (dirs, conf) = init_app().expect("failed to init app");
    println!("{:?}", conf);
    println!("{:?}", dirs);

    // create database if it doesn't exist,
    db::run_migration(&mut db::establish_connection(Some(&conf.general.db_uri)));

    if let Some(cmd) = args.cmd {
        match cmd {
            Import { path } => {
                // read path, create database if it doesn't exist
                import_sounds(&path, &conf.general.sound_file_path, &conf.general.db_uri);

            },
            Commands::List => {
                match load_all_sounds(&conf.general.db_uri) {
                    Ok(soundlist) => {
                        soundlist.iter()
                            .for_each(|s| {
                                println!("{}",
                                         serde_json::to_string(s)
                                             .expect("failed to serialize sound")
                                );
                            });
                    }
                    Err(e) => {
                        println!("No sounds available ({:?})", e);
                    }
                }
            },
            #[cfg(feature = "gui")]
            Commands::Gui => {
                let _result = gui::main_window(&conf.general.db_uri);
            }
            Commands::Serve => {
                let _result = run(conf.general.db_uri,
                                  dirs.queue_conf.to_str()
                                      .expect("could not create string")
                                      .to_string());
            }
        }
        return;
    }
}
