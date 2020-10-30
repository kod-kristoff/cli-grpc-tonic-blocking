pub mod remotecli;

use structopt::StructOpt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = ApplicationArguments::from_args();

    match args.subcommand {
        SubCommand::StartServer(opts) => {
            println!("Start the server on: {:?}", opts.server_listen_addr);
            remotecli::server::start_server(opts).await?;
        }
        SubCommand::Run(rc_opts) => {
            println!("Run command: '{:?}' on {:?}", rc_opts.command, rc_opts.server_addr);
            remotecli::client::client_run(rc_opts).await?;
        }
    }
    Ok(())
}


// This is the main arguments structure
#[derive(StructOpt, Debug)]
#[structopt(name = "remotecli")]
struct ApplicationArguments {
    #[structopt(flatten)]
    pub subcommand: SubCommand,
}


#[derive(StructOpt, Debug)]
pub enum SubCommand {
    /// Start the remote command gRPC server
    #[structopt(name = "server")]
    StartServer(ServerOptions),
    /// Send a remote command to the gRPC server
    #[structopt(setting = structopt::clap::AppSettings::TrailingVarArg)]
    Run(RemoteCommandOptions),
}


#[derive(StructOpt, Debug)]
pub struct ServerOptions {
    /// The address of the server that will run commands.
    #[structopt(long, default_value = "127.0.0.1:50051")]
    pub server_listen_addr: String,
}


#[derive(StructOpt, Debug)]
pub struct RemoteCommandOptions {
    /// The address of the server to connect to.
    #[structopt(long = "server", default_value = "http://127.0.0.1:50051")]
    pub server_addr: String,
    /// The command to run
    pub command: Vec<String>,
}
