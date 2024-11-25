use std::path::PathBuf;
use clap::builder::Str;
use rumqttd::{Broker, Config};
use config;

pub fn run_broker(conf: String) {
    // As examples are compiled as seperate binary so this config is current path dependent. Run it
    // from root of this crate
    let config = config::Config::builder()
        .add_source(config::File::with_name(conf.as_str()))
        .build()
        .unwrap();

    let config: Config = config.try_deserialize().unwrap();

    let mut broker = Broker::new(config);
    // let (mut link_tx, mut link_rx) = broker.link("singlenode").unwrap();
    broker.start().unwrap();
}