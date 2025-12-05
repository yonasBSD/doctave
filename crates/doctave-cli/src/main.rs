use doctave_lib as doctave;

use bunt::termcolor::{ColorChoice, StandardStream};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "Doctave",
    version = env!("CARGO_PKG_VERSION"),
    about = "An opinionated static site generator designed specifically for technical documentation."
)]
struct Cli {
    /// Disable terminal color output
    #[arg(long, global = true)]
    no_color: bool,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new project (start here!)
    Init {
        /// Optional custom root directory for documentation (defaults to docs/)
        #[arg(long)]
        docs_dir: Option<String>,
    },

    /// Builds your site from the project's Markdown files
    Build {
        /// Build the site in release mode
        #[arg(long)]
        release: bool,

        /// Don't return an error if there are failed checks
        #[arg(long)]
        allow_failed_checks: bool,
    },

    /// Starts a live reloading development server to serve your documentation site
    Serve {
        /// Port used to serve the documentation site. Must be a positive integer.
        #[arg(short, long, value_parser = clap::value_parser!(u32))]
        port: Option<u32>,
    },
}

fn main() {
    let cli = Cli::parse();

    let result = match &cli.command {
        Some(Commands::Init { docs_dir }) => init(&cli, docs_dir.clone()),
        Some(Commands::Build {
            release,
            allow_failed_checks,
        }) => build(&cli, *release, *allow_failed_checks),
        Some(Commands::Serve { port }) => serve(&cli, *port),
        None => Ok(()),
    };

    let mut out = if cli.no_color {
        StandardStream::stdout(ColorChoice::Never)
    } else {
        StandardStream::stdout(ColorChoice::Auto)
    };

    if let Err(e) = result {
        bunt::writeln!(out, "{$red}ERROR:{/$} {}", e).unwrap();
        std::process::exit(1);
    }
}

fn init(cli: &Cli, docs_dir: Option<String>) -> doctave::Result<()> {
    let root_dir = std::env::current_dir().expect("Unable to determine current directory");
    doctave::InitCommand::run(root_dir, !cli.no_color, docs_dir)
}

fn build(cli: &Cli, release: bool, allow_failed_checks: bool) -> doctave::Result<()> {
    let project_dir = doctave::config::project_root().unwrap_or_else(|| {
        eprintln!("Could not find a doctave project in this directory, or its parents.");
        std::process::exit(1);
    });
    let mut config = doctave::Config::load(&project_dir)?;
    if release {
        config.set_build_mode(doctave::BuildMode::Release);
    }
    if cli.no_color {
        config.disable_colors();
    }
    if allow_failed_checks {
        config.set_allow_failed_checks();
    }
    doctave::BuildCommand::run(config)
}

fn serve(cli: &Cli, port: Option<u32>) -> doctave::Result<()> {
    let project_dir = doctave::config::project_root().unwrap_or_else(|| {
        eprintln!("Could not find a doctave project in this directory, or its parents.");
        std::process::exit(1);
    });
    let mut options = doctave::ServeOptions::default();
    let mut config = doctave::Config::load(&project_dir)?;
    if let Some(p) = port {
        options.port = Some(p);
    }
    if cli.no_color {
        config.disable_colors();
    }
    doctave::ServeCommand::run(options, config)
}
