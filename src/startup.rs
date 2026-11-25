pub struct Startup {
    exit: bool,
    config: Config,
}

impl Startup {
    pub fn new(config: Config) -> Self {
        Self {
            exit: false,
            config,
        }
    }
}
