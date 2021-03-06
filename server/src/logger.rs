use log4rs::append::console::ConsoleAppender;
use log4rs::append::rolling_file::RollingFileAppender;
use log4rs::append::rolling_file::policy::compound::CompoundPolicy;
use log4rs::append::rolling_file::policy::compound::trigger::size::SizeTrigger;
use log4rs::append::rolling_file::policy::compound::roll::fixed_window::FixedWindowRoller;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::config::{Appender, Config, Root};
use log::LevelFilter;

const LOG_PATTERN: &str = "[{d(%H:%M:%S)}] [{thread}/{h({level})}]: {m}{n}";

pub fn init() {
    let stdout = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new(LOG_PATTERN)))
        .build();

    let file = RollingFileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(LOG_PATTERN)))
        .build("logs/server.log", Box::new(
            CompoundPolicy::new(
                Box::new(SizeTrigger::new(10_000_000)),
                Box::new(FixedWindowRoller::builder().build("logs/server-{}.log.gz", 10).unwrap()),
            ),
        ))
        .unwrap();

    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .appender(Appender::builder().build("file", Box::new(file)))
        .build(
            Root::builder()
                .appender("stdout")
                .appender("file")
                .build(LevelFilter::Info)
        )
        .unwrap();

    log4rs::init_config(config).unwrap();

    info!("init -> Logger initialized.");
}
