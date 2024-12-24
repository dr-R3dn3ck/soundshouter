
mod broker;
mod db;
mod audio_player;
mod api;
mod connection_pool;
mod error;
mod config;
mod import;
mod cli;

use std::io::Write;
use std::path::PathBuf;
use std::str::FromStr;
use clap::{arg, Parser, Subcommand};
use log::info;
use crate::db::load_all_psounds;
use crate::config::{init_app};
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

// #[option(short = 'i',  help = "delete without confirmation", default_value = "false")]


#[derive(Subcommand, Debug)]
#[clap(name = "import")]
enum Commands {
    /// import sounds
    Import {
        #[arg(help = "directory with sound files to import")]
        path: PathBuf
    },
    /// list all available sounds
    List {
        #[arg(help = format!("output format [{}, {}]", Format::Jsonl.to_string(), Format::Paths.to_string()))]
        format: Format
    },
    Reset {
        #[arg(short = 'y', help = "delete without confirmation", default_value = "false")]
        yes: bool,
        #[arg(long = "include-config", help = "delete config files", default_value = "false")]
        include_config: bool,
    },
    Config {
        #[arg(long = "data-dir", help = "print data directory")]
        data_dir: bool,
    },
    Serve,
}

#[derive(Debug, Clone)]
enum Format {
    Jsonl,
    Paths
}

impl FromStr for Format {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "jsonl" => Ok(Format::Jsonl),
            "paths" => Ok(Format::Paths),
            _ => Err(format!("invalid format: {}", s))
        }
    }
}

impl ToString for Format {
    fn to_string(&self) -> String {
        match self {
            Format::Jsonl => "jsonl".to_string(),
            Format::Paths => "paths".to_string(),
        }
    }
}

fn main() {
    let args = Args::parse();

    let (dirs, conf) = init_app().expect("failed to init app");
    log4rs::init_file(&dirs.log_conf, Default::default()).unwrap();

    // create database if it doesn't exist,
    db::run_migration(&mut db::establish_connection(Some(&conf.general.db_uri)));

    if let Some(cmd) = args.cmd {
        match cmd {
            Commands::Reset { yes, include_config } => {
                let input  = if yes {
                    "Y".to_string()
                } else {
                    let st = if include_config { "and configuration " } else { "" };
                    print!("Should all files {}be deleted? [Y]: ", st);

                    std::io::stdout().flush().unwrap();
                    let mut input = String::new();
                    std::io::stdin()
                        .read_line(&mut input)
                        .expect("Failed to read input");
                    input
                };

                match input.trim() {
                    "Y" => {
                        std::fs::remove_dir_all(&dirs.data_dir).expect("failed to reset data dir");
                        std::fs::remove_dir_all(&dirs.config_dir).expect("failed to reset config dir");
                    }
                    _ => {
                        println!("aborted");
                    }
                }
            },
            Commands::Import { path } => {
                // read path, create database if it doesn't exist
                import_sounds(&path, &conf.general.sound_file_path, &conf.general.db_uri);

            },
            Commands::List { format} => {
                match load_all_psounds(&conf.general.db_uri) {
                    Ok(soundlist) => {
                        soundlist.iter()
                            .for_each(|s| {
                                match format {
                                    Format::Jsonl => {
                                        if let Ok(snd) = serde_json::to_string(s) {
                                            println!("{}", snd);
                                        }
                                    }
                                    Format::Paths => {
                                        let pth = conf.general.sound_file_path.join(&s.path);
                                        if let Some(str) = pth.to_str(){
                                            println!("{}", str);
                                        }
                                    }
                                }
                            })
                    }
                    Err(e) => {
                        println!("No sounds available ({:?})", e);
                    }
                }
            },
            Commands::Config { data_dir } => {
                if data_dir {
                    println!("{}", dirs.data_dir.to_str().unwrap_or(""));
                }
            },
            Commands::Serve => {
                let _result = run(conf.general.db_uri.clone(),
                                  dirs.queue_conf.to_str()
                                      .expect("could not create string")
                                      .to_string(),
                                    &conf);

                info!("config: {:?}", dirs.config_dir);
                info!("data:   {:?}", dirs.data_dir)
            }
        }
        return;
    }
}
