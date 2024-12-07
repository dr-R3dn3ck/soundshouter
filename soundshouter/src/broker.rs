
use rumqttd::{Broker, Config};
use config;

/// run embedded 'rumqttd' mqtt broker
/// configuration is entirely done in a configuration file
pub fn run_broker(conf: String) {
    let config = config::Config::builder()
        .add_source(config::File::with_name(conf.as_str()))
        .build()
        .unwrap();

    let config: Config = config.try_deserialize().unwrap();

    let mut broker = Broker::new(config);

    broker.start().unwrap();
}