use anyhow::{anyhow, Context};
use clap::Args;
use minicbor::Decoder;
use ockam_api::clean_multiaddr;
use serde_json::json;
use tracing::debug;

use ockam_api::nodes::models::forwarder::CreateForwarder;
use ockam_api::nodes::models::forwarder::ForwarderInfo;
use ockam_api::nodes::NODEMANAGER_ADDR;
use ockam_core::api::{Method, Request, Response, Status};
use ockam_core::Route;
use ockam_multiaddr::MultiAddr;

use crate::util::{connect_to, exitcode, get_final_element, stop_node, DEFAULT_CLOUD_ADDRESS};
use crate::{CommandGlobalOpts, OutputFormat};

#[derive(Clone, Debug, Args)]
pub struct CreateCommand {
    /// Node for which to create the forwarder.
    #[clap(long = "for", name = "NODE", display_order = 900)]
    for_node: String,

    /// Route to the node on which to create the forwarder.
    #[clap(long, name = "ROUTE", default_value = DEFAULT_CLOUD_ADDRESS, display_order = 900)]
    at: MultiAddr,

    /// Forwarding address.
    #[clap(long = "from", display_order = 900)]
    from: Option<String>,

    /// Forwarding address.
    #[clap(hide = true)]
    address: Option<String>,
}

impl CreateCommand {
    pub fn run(opts: CommandGlobalOpts, cmd: CreateCommand) {
        let cfg = &opts.config;
        let cmd = CreateCommand {
            at: match clean_multiaddr(&cmd.at, &cfg.get_lookup()) {
                Some(addr) => addr,
                None => {
                    eprintln!("failed to normalize MultiAddr route");
                    std::process::exit(exitcode::USAGE);
                }
            },
            from: cmd
                .from
                .as_ref()
                .map(|f| String::from(get_final_element(f))),
            ..cmd
        };

        let node = get_final_element(&cmd.for_node);
        let port = match cfg.select_node(node) {
            Some(cfg) => cfg.port,
            None => {
                eprintln!("No such node available.  Run `ockam node list` to list available nodes");
                std::process::exit(exitcode::IOERR);
            }
        };
        connect_to(port, (opts, cmd), create);
    }
}

async fn create(
    ctx: ockam::Context,
    (opts, cmd): (CommandGlobalOpts, CreateCommand),
    mut base_route: Route,
) -> anyhow::Result<()> {
    let route: Route = base_route.modify().append(NODEMANAGER_ADDR).into();
    let message = make_api_request(cmd)?;

    let response: Vec<u8> = ctx
        .send_and_receive(route, message)
        .await
        .context("Failed to process request")?;
    let mut dec = Decoder::new(&response);
    let header = dec.decode::<Response>()?;
    debug!(?header, "Received response");

    let res = match header.status() {
        Some(Status::Ok) => {
            let body = dec.decode::<ForwarderInfo>()?;
            let address = format!("/service/{}", body.remote_address());
            let output = match opts.global_args.output_format {
                OutputFormat::Plain => address,
                OutputFormat::Json => json!({ "remote_address": address }).to_string(),
            };
            Ok(output)
        }
        Some(Status::InternalServerError) => {
            let err = dec
                .decode::<String>()
                .unwrap_or_else(|_| "Unknown error".to_string());
            Err(anyhow!(
                "An error occurred while processing the request: {err}"
            ))
        }
        _ => Err(anyhow!("Unexpected response received from node")),
    };
    match res {
        Ok(o) => println!("{o}"),
        Err(err) => {
            eprintln!("{err}");
            std::process::exit(exitcode::IOERR);
        }
    };

    stop_node(ctx).await
}

/// Construct a request to create a forwarder
pub(crate) fn make_api_request(cmd: CreateCommand) -> ockam::Result<Vec<u8>> {
    let (at_rust_node, address) = match &cmd.from {
        Some(_s) => (true, &cmd.from),
        None => (false, &cmd.address),
    };

    let mut buf = vec![];
    Request::builder(Method::Post, "/node/forwarder")
        .body(CreateForwarder::new(
            &cmd.at,
            address.as_deref(),
            at_rust_node,
        ))
        .encode(&mut buf)?;
    Ok(buf)
}
