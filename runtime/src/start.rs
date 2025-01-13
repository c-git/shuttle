use anyhow::Context;

use crate::{
    __internals::{Loader, Runner},
    alpha, rt,
};

#[derive(Default)]
struct Args {
    /// Enable compatibility with beta platform [env: SHUTTLE_BETA]
    beta: bool,
    /// Alpha (required): Port to open gRPC server on
    port: Option<u16>,
}

impl Args {
    // uses simple arg parsing logic instead of clap to reduce dependency weight
    fn parse() -> anyhow::Result<Self> {
        let mut args = Self::default();

        // The first argument is the path of the executable
        let mut args_iter = std::env::args().skip(1);

        while let Some(arg) = args_iter.next() {
            if arg.as_str() == "--port" {
                let port = args_iter
                    .next()
                    .context("missing port value")?
                    .parse()
                    .context("invalid port value")?;
                args.port = Some(port);
            }
        }

        args.beta = std::env::var("SHUTTLE_BETA").is_ok();

        if args.beta {
            if std::env::var("SHUTTLE_ENV").is_err() {
                return Err(anyhow::anyhow!(
                    "SHUTTLE_ENV is required to be set on shuttle.dev"
                ));
            }
        } else if args.port.is_none() {
            return Err(anyhow::anyhow!("--port is required"));
        }

        Ok(args)
    }
}

pub async fn start(loader: impl Loader + Send + 'static, runner: impl Runner + Send + 'static) {
    if true {
        rt::start(loader, runner).await
    } else {
        alpha::start(0, loader, runner).await
    }
}
