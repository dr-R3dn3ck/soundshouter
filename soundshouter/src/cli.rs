use std::thread;
use std::thread::JoinHandle;
use crate::audio_player::{AudioPlayer};
use crate::broker::run_broker;
use crate::api::run_api;
use crate::config::Config;

pub struct AppState {
    api_thread: Option<JoinHandle<()>>,
    broker_thread: Option<JoinHandle<()>>,
    player_thread: Option<JoinHandle<()>>
}

impl AppState {
    fn start(&mut self, db_uri: String, broker_conf: String, config: &Config) -> &mut AppState {
        if self.api_thread.is_none() {
            self.api_thread = Some(thread::spawn(|| { let _ = run_api();}));
        }
        if self.broker_thread.is_none() {
            if config.queue.active {
                self.broker_thread = Some(thread::spawn(|| { run_broker(broker_conf); }));
            }
        }

        if self.player_thread.is_none() {
            let topic = config.queue.topic.to_string();
            let ip = config.queue.ip.to_string();
            let port = config.queue.port;

            self.player_thread = Some(thread::spawn(move || {
                AudioPlayer {
                    db_url: db_uri,
                    queue_topic: topic,
                    queue_ip: ip,
                    queue_port: port,
                    ..AudioPlayer::default()
                }
                    .poll_queue();
            }));
        }
        self
    }

    fn wait(&mut self) {
        if let Some(pthread) = self.api_thread.take() {
            pthread.join().unwrap();
        } else {
            println!("No player thread to wait for");
        }
    }
}

pub fn run(db_uri: String, broker_conf: String, config: &Config) {
    let mut appstate = AppState { api_thread: None, broker_thread: None, player_thread: None };
    appstate.start(db_uri, broker_conf, config);
    appstate.wait();
}