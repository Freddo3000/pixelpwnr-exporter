# pixelpwnr-exporter
![Crates.io Version](https://img.shields.io/crates/v/pixelpwnr-exporter)
![Crates.io Total Downloads](https://img.shields.io/crates/d/pixelpwnr-exporter)
![GitLab Tag](https://img.shields.io/gitlab/v/tag/freddo%2Fpixelpwnr-exporter?gitlab_url=https%3A%2F%2Fgit.ludd.ltu.se)


This is a simply pixelpwnr [prometheus](https://prometheus.io) exporter for the
[pixelpwnr](https://github.com/timvisee/pixelpwnr-server) PixelFlut server. It
reads the produced stats file and exports it in a format digestible by
Prometheus.

## Downloads

The latest binary can be downloaded
[here](https://git.ludd.ltu.se/api/v4/projects/862/jobs/artifacts/master/raw/target/release/pixelpwnr-exporter?job=build)
or be fetched as a docker image here: `registry.git.ludd.ltu.se/freddo/pixelpwnr-exporter`.

## Sample config

```toml
# Rocket.toml
[release]
# Supports rocket.rs configuration: https://rocket.rs/guide/v0.5/configuration/
address = "0.0.0.0"
port = 8000

# Extended with the following options:
stats_file = "pixelpwnr.yaml" # Path to PixelPwnr stats file
stats_prefix = "pixelpwnr"    # Prefix for all metrics

[[stats_labels]]
# Labels applied to all metrics
instance = "bigscreen"
location = "entry_hall"
```
Or use environment variables...
```env
ROCKET_address=0.0.0.0
ROCKET_PORT=8000
ROCKET_stats_file=pixelpwnr.yaml
ROCKET_stats_prefix=pixelpwnr
ROCKET_stats_labels={instance="bigscreen",location="entry_hall"}
```


---

[![](http://www.wtfpl.net/wp-content/uploads/2012/12/wtfpl-badge-4.png)](http://www.wtfpl.net/)