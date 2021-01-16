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
    #[structopt(short, long, help = "Enables verbose logging")]
    verbose: bool,

    #[structopt(subcommand)]
    subcommand: ZagreusSubcommand,
}

impl ZagreusCommand {
    pub fn is_verbose(&self) -> bool {
        self.verbose
    }

    pub fn subcommand(self) -> ZagreusSubcommand {
        self.subcommand
    }
}

#[derive(Debug, StructOpt)]
pub enum ZagreusSubcommand {
    #[structopt(name = "new", about = "Generates a new boilerplate template.")]
    New {
        #[structopt(help = "Name of the new template, allowed characters: [a-zA-Z0-9-]")]
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
