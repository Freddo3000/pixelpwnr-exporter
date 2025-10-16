#[macro_use]
extern crate rocket;

use lazy_static::lazy_static;
use prometheus::{IntCounter, IntGauge, Registry};
use rocket::State;
use rocket::fairing::AdHoc;
use rocket::http::ContentType;
use rocket::serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::process::exit;
use std::str::FromStr;

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
struct Stats {
    pixels: i64,
    bytes_read: i64,
}

#[derive(Debug, Deserialize, Default)]
#[serde(crate = "rocket::serde")]
struct ExporterConfig {
    #[serde(default)]
    stats_prefix: Option<String>,
    #[serde(default)]
    stats_labels: Option<HashMap<String, String>>,
    stats_file: PathBuf,
}

lazy_static! {
    static ref PIXELS_GAUGE: IntGauge =
        IntGauge::new("pixels", "Number of pixels received").unwrap();
    static ref BYTES_GAUGE: IntGauge =
        IntGauge::new("bytes_read", "Number of bytes received").unwrap();
    static ref EXPORTER_COUNTER: IntCounter =
        IntCounter::new("requests", "Number of requests sent to metrics endpoint").unwrap();
}

#[get("/metrics")]
fn metrics(
    cfg: &State<ExporterConfig>,
    registry: &State<Registry>,
) -> rocket_anyhow::Result<(ContentType, String)> {
    let data = fs::read_to_string(cfg.stats_file.clone())?;
    let res: Stats = serde_yaml::from_str(&data)?;

    PIXELS_GAUGE.set(res.pixels);
    BYTES_GAUGE.set(res.bytes_read);
    EXPORTER_COUNTER.inc();

    let encoder = prometheus::TextEncoder::new();
    let res = encoder.encode_to_string(&registry.gather())?;

    Ok((ContentType::from_str(prometheus::TEXT_FORMAT).unwrap(), res))
}

#[launch]
fn rocket() -> _ {
    let r = rocket::build().attach(AdHoc::config::<ExporterConfig>());

    let cfg = match r.figment().extract::<ExporterConfig>() {
        Ok(cfg) => cfg,
        Err(e) => {
            error!("{}", e);
            exit(1);
        }
    };

    let reg = Registry::new_custom(cfg.stats_prefix, cfg.stats_labels).unwrap_or_else(|e| {
        error!("{}", e);
        exit(1);
    });
    reg.register(Box::new(PIXELS_GAUGE.clone())).unwrap();
    reg.register(Box::new(BYTES_GAUGE.clone())).unwrap();
    reg.register(Box::new(EXPORTER_COUNTER.clone())).unwrap();

    r.manage(reg).mount("/", routes![metrics])
}
