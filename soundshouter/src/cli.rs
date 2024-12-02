use std::path::PathBuf;
use std::thread;
use std::thread::JoinHandle;
use clap::builder::Str;
use crate::audio_player::{play_test_audio, poll_queue};
use crate::broker::run_broker;
use crate::api::run_api;

pub struct AppState {
    api_thread: Option<JoinHandle<()>>,
    broker_thread: Option<JoinHandle<()>>,
    player_thread: Option<JoinHandle<()>>
}

impl AppState {
    fn start(&mut self, db_uri: String, broker_conf: String) -> &mut AppState {
        if self.api_thread.is_none() {
            self.api_thread = Some(thread::spawn(|| { run_api();}));
        }
        if self.broker_thread.is_none() {
            self.broker_thread = Some(thread::spawn(|| { run_broker(broker_conf); }));
        }
        if self.player_thread.is_none() {
            self.player_thread = Some(thread::spawn(||{ poll_queue(db_uri);}));
        }
        self
    }

    fn new_sound_thread(&mut self) {
        self.player_thread = Some(thread::spawn(||{ play_test_audio();}));
    }

    fn wait(&mut self) {
        if let Some(pthread) = self.api_thread.take() {
            pthread.join().unwrap();
        } else {
            println!("No player thread to wait for");
        }
    }
}

pub fn run(db_uri: String, broker_conf: String) {
    let mut appstate = AppState { api_thread: None, broker_thread: None, player_thread: None };
    appstate.start(db_uri, broker_conf);
    appstate.wait();
}