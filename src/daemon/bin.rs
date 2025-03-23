mod daemon;
use daemon::GavDaemon;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    //env_logger::init();
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("trace")).init();


    let mut daemon: GavDaemon = GavDaemon::new(7878).await?;

    daemon.run_daemon().await?;

    Ok(())
}
