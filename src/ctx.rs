use clap::ArgMatches;
use config::types::Config;
use std::path::Path;
use xdg::BaseDirectories;

pub struct Context<'a> {
    pub args: ArgMatches<'a>,
    pub dirs: BaseDirectories,
    pub config: Config,
}

impl<'a> Context<'a> {
    pub fn open(args: ArgMatches<'a>) -> Context<'a> {
        let dirs = BaseDirectories::with_prefix("rusty-term").unwrap();

        let config_path = dirs.place_config_file("config.cfg")
                              .expect("Application data not present");
        if !config_path.exists() {
            create_default_config(&config_path);
        }
        let config = ::config::reader::from_file(&config_path).unwrap();

        Context {
            args: args,
            dirs: dirs,
            config: config,
        }
    }
}

fn create_default_config(path: &Path) {
    use std::fs::File;
    use std::io::prelude::*;

    static DEFAULT_CONFIG: &'static str = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/data/config.cfg"
    ));

    let mut file = File::create(path).unwrap();
    file.write_all(DEFAULT_CONFIG.as_bytes()).unwrap();
}
