use std::{fs, io};
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use directories::ProjectDirs;

#[derive(Debug)]
/// Soundshouter base dir configuration
pub struct DirConfig {
    // pub(crate) rocket_conf: PathBuf,
    pub(crate) queue_conf: PathBuf,
    // pub(crate) config_dir: PathBuf,
    // pub(crate) data_dir: PathBuf,
    pub(crate) log_conf: PathBuf,
}

#[derive(Deserialize, Serialize, Debug)]
/// Soundshouter configuration
pub struct Config {
    pub(crate) general: General,
    api: Option<Api>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct General {
    pub(crate) sound_file_path: PathBuf,
    pub(crate) db_uri: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Api {
    active: bool,
}

#[cfg(not(target_os = "macos"))]
fn qualifier() -> &'static str { "de" }
#[cfg(target_os = "macos")]
fn qualifier() -> &'static str { "" }

/// Checks if configuration dir is available. If not available, the directory and default config
/// files will be created.
/// If directory and configurations exist, loads returns configuration.
pub fn init_app() -> Result<(DirConfig, Config), io::Error> {
    let proj_dirs = ProjectDirs::from(
        qualifier(), "Megalaessig",  "Soundshouter")
        .expect("could not find project directories");

    let config_dir = proj_dirs.config_dir().to_path_buf();
    if !config_dir.try_exists()? {
        fs::create_dir_all(proj_dirs.config_dir())?;
    }

    let data_path = proj_dirs.data_dir().to_path_buf();
    if !data_path.try_exists()? {
        fs::create_dir_all(&data_path)?;
    }

    let config_path = proj_dirs.config_dir().join("config.toml");
    let config = if config_path.exists() {
        load_config(&config_path)?
    }
    else {
        create_default_config(&config_path, &data_path)?
    };
    let queue_conf = proj_dirs.config_dir().join("rumqttd.toml");
    if !queue_conf.exists() {
        create_broker_config(&queue_conf)?;
    }
    let rocket_conf = proj_dirs.config_dir().join("Rocket.toml");
    if !rocket_conf.exists() {
        create_api_config(&rocket_conf, &config.general.db_uri)?;
    }

    let log_conf = proj_dirs.config_dir().join("log4rs.yml");
    if !log_conf.exists() {
        create_log_config(&log_conf)?;
    };
    std::env::set_var("ROCKET_CONFIG", &rocket_conf);

    Ok((DirConfig {
        // rocket_conf,
        queue_conf,
        // config_dir,
        // data_dir: data_path,
        log_conf
    }, config))
}

fn create_log_config(pth: &PathBuf) -> Result<(), io::Error> {
    let config = r#"
appenders:
  stdout:
    kind: console
root:
  level: warn
  appenders:
    - stdout
loggers:
  rumqttd::router::routing: { level: off }
  rumqttd::server::broker: { level: off }
  rocket: { level: info }
  soundshouter: { level: info }
"#;
    let mut file = File::create(pth)?;
    file.write_all(config.as_bytes())?;

    Ok(())
}

fn create_default_config(pth: &PathBuf, data_pth: &PathBuf) -> Result<Config, io::Error> {
    let config = Config {
        general: General {
            sound_file_path: data_pth.join("sounds"),
            db_uri: data_pth.join("soundshouter.sqlite3").to_str().unwrap().to_string(),
        },
        api: Some(Api { active: true }),
    };

    let mut file = File::create(pth)?;
    let toml = toml::to_string(&config).expect(
        "could not serialize config to toml");

    let commented_conf = r#"# soundshouter configuration file
#

"#.to_string() + toml.as_str();

    file.write_all(commented_conf.as_bytes())?;

    Ok(config)
}

fn create_broker_config(pth: &PathBuf) -> Result<(), io::Error> {
    let config = r#"id = 0
[router]
id = 0
max_connections = 10010
max_outgoing_packet_count = 200
max_segment_size = 104857600
max_segment_count = 10

[v4.1]
name = "v4-1"
listen = "0.0.0.0:1883"
next_connection_delay_ms = 1

[v4.1.connections]
connection_timeout_ms = 60000
max_payload_size = 20480
max_inflight_count = 100
dynamic_filters = true

[console]
listen = "0.0.0.0:3030"
"#;

    let mut file = File::create(pth)?;
    file.write_all(config.as_bytes())?;

    Ok(())
}

fn create_api_config(pth: &PathBuf, dburi: &String) -> Result<(), io::Error> {
    let config = format!(r#"[default.databases.soundshouter]
url = "{}"
timeout = 10"#, dburi);

    let mut file = File::create(pth)?;
    file.write_all(config.as_bytes())?;

    Ok(())
}

fn load_config(pth: &PathBuf) -> Result<Config, io::Error> {
    let mut file = File::open(pth)?;
    let mut cnf_string = String::new();
    file.read_to_string(&mut cnf_string)?;
    let config: Config = toml::from_str(&*cnf_string).unwrap();

    Ok(config)
}