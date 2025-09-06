use anyhow::Result;
use clap::{Parser, Subcommand};
use log::info;

#[derive(Parser)]
#[command(name = "jpegxs")]
#[command(about = "JPEG XS encoder/decoder CLI", long_about = None)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,
}

#[derive(Subcommand)]
enum Commands {
    Encode {
        #[arg(short, long)]
        input: String,

        #[arg(short, long)]
        output: String,

        #[arg(short = 'W', long)]
        width: u32,

        #[arg(short = 'H', long)]
        height: u32,

        #[arg(short, long, default_value = "yuv422p")]
        format: String,

        #[arg(short, long, default_value = "0.9")]
        quality: f32,
    },

    Decode {
        #[arg(short, long)]
        input: String,

        #[arg(short, long)]
        output: String,
    },

    Validate {
        #[arg(short, long)]
        input: String,

        #[arg(long, default_value = "jxs")]
        reference: String,
    },

    Info {
        #[arg(short, long)]
        input: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let log_level = match cli.verbose {
        0 => "warn",
        1 => "info",
        2 => "debug",
        _ => "trace",
    };

    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or(log_level)).init();

    match cli.command {
        Commands::Encode {
            input,
            output,
            width,
            height,
            format,
            quality,
        } => {
            info!("Encoding {} to {}", input, output);
            info!(
                "Resolution: {}x{}, Format: {}, Quality: {}",
                width, height, format, quality
            );
            println!("Encoding not yet implemented");
        }

        Commands::Decode { input, output } => {
            info!("Decoding {} to {}", input, output);
            println!("Decoding not yet implemented");
        }

        Commands::Validate { input, reference } => {
            info!("Validating {} against {}", input, reference);
            println!("Validation not yet implemented");
        }

        Commands::Info { input } => {
            info!("Getting info for {}", input);
            println!("Info not yet implemented");
        }
    }

    Ok(())
}
