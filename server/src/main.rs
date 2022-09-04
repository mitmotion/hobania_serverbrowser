mod gameservers;
mod metrics;
mod server;

use tracing_subscriber::{prelude::__tracing_subscriber_SubscriberExt, EnvFilter};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup tracing layers

    #[cfg(feature = "telemetry")]
    let telemetry = tracing_opentelemetry::layer().with_tracer(telemetry::init_tracer().await);
    let logger = tracing_subscriber::fmt::layer();
    let env_filter = EnvFilter::try_from_default_env()?
        .add_directive("hyper=info".parse()?)
        .add_directive("tower=info".parse()?)
        .add_directive("tokio_util=info".parse()?)
        .add_directive("mio=info".parse()?);

    let collector = tracing_subscriber::Registry::default()
        .with(logger)
        .with(env_filter);

    // Initialize tracing
    tracing::subscriber::set_global_default(collector).unwrap();

    let rt = tokio::runtime::Runtime::new()?;

    tracing::info!("Starting veloren serverbrowser");
    tracing::info!("running...");
    rt.block_on(async {
        tokio::select! {
            _ = server::server() => tracing::info!("server exited"),
        }
    });
    tracing::info!("stopped");

    Ok(())
}
