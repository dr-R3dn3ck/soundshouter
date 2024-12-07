use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::{ExpressionMethods, QueryDsl, SqliteConnection};
use diesel::prelude::*;
use rodio;
use rumqttc::{Client, Event, Incoming, MqttOptions, QoS};
use std::fs::File;
use std::io::BufReader;
use std::time::Duration;
use std::{thread};
use log::{debug, error};
use crate::db::models::{PSound};
use rodio::OutputStreamHandle;

fn increment_play_count(sound_id: u32, con: &mut PooledConnection<ConnectionManager<SqliteConnection>>) {
    use crate::db::schema::sound::dsl::*;

    let res = diesel::update(sound)
        .filter(id.eq(sound_id as i32))
        .set(play_count.eq(play_count + 1))
        .execute(con);

    match res {
        Err(e) => {
            error!("error updating play count: {}", e);
        }
        _ => {}
    }
}

pub fn load_audio(sound_id: u32, con: &mut PooledConnection<ConnectionManager<SqliteConnection>>) -> Option<File> {
    use crate::db::schema::sound;

    let snd_lst: Vec<PSound> = if let Ok(snd) = sound::dsl::sound
        .filter(sound::dsl::id.eq(sound_id as i32))
        .select(PSound::as_select())
        .load(con) {
        snd
    }
    else {
        error!("couldn't load sound {}", sound_id);
        return None;
    };


    debug!("{:?}", &snd_lst);
    match snd_lst.first() {
        Some(_sound) => {
            if let Ok(file) = File::open(&_sound.path) {
                Some(file)
            }
            else {
                error!("couldn't open file of sound {}", sound_id);
                None
            }
        },
        None => {
            error!("sound {} not in database", sound_id);
            None
        }
    }
}

pub fn play_audio(file: File, stream_handle: &OutputStreamHandle) {
    let sink = rodio::Sink::try_new(&stream_handle).unwrap();
    sink.append(rodio::Decoder::new(BufReader::new(file)).unwrap());
    sink.detach();
}

pub fn play(id: u32, con: &mut PooledConnection<ConnectionManager<SqliteConnection>>, stream_handle: &OutputStreamHandle) {
    let pth = load_audio(id, con);
    // increment play count
    // submit_to_play_queue(id);
    match pth {
        Some(file) => {
            play_audio(file, stream_handle);
            increment_play_count(id, con);
        },
        None => {}
    };
}

pub fn poll_queue(db_url: String) {
    // wait for mqtt queue
    thread::sleep(Duration::from_millis(50));

    let mut mqttoptions = MqttOptions::new(
        "soundshouter-player", "127.0.0.1", 1883);
    mqttoptions.set_keep_alive(Duration::from_secs(10));

    let (client, mut connection) = Client::new(mqttoptions, 10);
    client
        .subscribe("soundshouter/queue", QoS::AtMostOnce)
        .unwrap();

    let manager = ConnectionManager::<SqliteConnection>::new(&db_url);
    let pool = Pool::builder()
        .build(manager)
        .expect("Failed to create the pool.");

    let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();

    // Iterate to poll the event loop for connection progress
    let mut con_iter = connection.iter();
    let mut i = 0;
    loop {
        i = if i < usize::MAX { i + 1 } else { 1 };

        let notification = con_iter.next()
            .expect("audio player connection error");

        debug!("Notification = {:?}", &notification);
        match notification {
            Ok(Event::Incoming(Incoming::Publish(p))) => {
                let payload_str = std::str::from_utf8(&p.payload).unwrap();
                debug!("Payload: {}", &payload_str);

                if let Ok(id) = payload_str.parse::<u32>() {
                    match pool.get() {
                        Ok(mut con) => {
                            play(id, &mut con, &stream_handle);
                        },
                        Err(_e) => {}
                    };
                };
            }
            Ok(_) => {}
            Err(_e) => {
                // error!("Error: {:?}", &e);
            }
        }
    }
}
