use env_logger::{Builder, Env};

fn main() {
    Builder::from_env(
        Env::new()
            .filter_or("OPENMC_LOG", "info")
            .write_style("OPENMC_LOG_STYLE"),
    )
    .init();
}
