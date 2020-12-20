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

    #[structopt(name = "build", about = "Builds the template in this directory.")]
    Build {
        #[structopt(short, long, help = "Keep running and rebuild on file changes")]
        watch: bool,

        #[structopt(
            short,
            long,
            help = "Upload template to the configured Zagreus server after building"
        )]
        upload: bool,
    },

    #[structopt(
        name = "upload",
        about = "Uploads the template in this directory to the Zagreus server configured in the template."
    )]
    Upload,
}