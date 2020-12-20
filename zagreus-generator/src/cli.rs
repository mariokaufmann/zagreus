use std::path::PathBuf;
use structopt::StructOpt;

pub fn get_command() -> ZagreusCommand {
    ZagreusCommand::from_args()
}

#[derive(Debug, StructOpt)]
#[structopt(
name = "Zagreus Template Generator",
about = "CLI application for generating, building and uploading Zagreus templates."
)]
pub struct ZagreusCommand {
    #[structopt(short, long, help = "Enables debug and trace logging")]
    debug: bool,

    #[structopt(subcommand)]
    subcommand: ZagreusSubcommand,
}

impl ZagreusCommand {
    pub fn debug_enabled(&self) -> bool {
        self.debug
    }

    pub fn subcommand(self) -> ZagreusSubcommand {
        self.subcommand
    }
}

#[derive(Debug, StructOpt)]
pub enum ZagreusSubcommand {
    #[structopt(name = "new", about = "Generates a new boilerplate template.")]
    New {
        #[structopt(help = "Name of the new template")]
        name: String,
    },

    #[structopt(name = "build", about = "Builds a template.")]
    Build {
        #[structopt(parse(from_os_str), help = "Path to the template to be built")]
        path: PathBuf,

        #[structopt(short, long, help = "Keep running and rebuild on file changes")]
        watch: bool,

        #[structopt(
        short,
        long,
        help = "Upload template to the configured Zagreus server after every build"
        )]
        upload: bool,
    },

    #[structopt(
    name = "upload",
    about = "Uploads a template to the Zagreus server configured in the template."
    )]
    Upload {
        #[structopt(parse(from_os_str), help = "Path to the template to be uploaded")]
        path: PathBuf,
    },
}