use std::path::PathBuf;

use structopt::StructOpt;

pub fn get_command() -> ZagreusServerCommand {
    ZagreusServerCommand::from_args()
}

#[derive(Debug, StructOpt)]
#[structopt(
    name = "Zagreus Server",
    about = "Management and playout server for Zagreus templates."
)]
pub struct ZagreusServerCommand {
    #[structopt(short, long, help = "Enables verbose logging")]
    pub verbose: bool,
    #[structopt(long, help = "The server port Zagreus should bind to.")]
    pub server_port: Option<u16>,
    #[structopt(
        long,
        help = "The data folder where Zagreus should store the template data."
    )]
    pub data_folder: Option<PathBuf>,
}
