use clap::{Parser, Subcommand};

use crate::data::theme::ThemeType;

#[derive(Parser, Clone)]
#[clap(author, version, about)]
pub struct OpenMCArgs {
    #[clap(short, long, env = "OPENMC_THEME")]
    pub theme: Option<ThemeType>,
    #[clap(long, default_value = "false")]
    pub no_gui: bool,
    #[clap(subcommand)]
    pub command: Option<OpenMCommands>,
}

#[derive(Subcommand, Debug, Clone)]
pub enum OpenMCommands {
    Launch {
        #[arg(short, long)]
        instance: String,
    },
}
