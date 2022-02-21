use std::io::Write;
use std::sync::Once;

static INIT: Once = Once::new();

pub fn logger_init() {
    INIT.call_once(|| {
        env_logger::builder()
            .format(|buf, record| {
                writeln!(
                    buf,
                    "[{} {}:{}] - {}",
                    buf.default_styled_level(record.level()),
                    record.file().unwrap(),
                    record.line().unwrap(),
                    record.args()
                )
            })
            .init();
    });
}
