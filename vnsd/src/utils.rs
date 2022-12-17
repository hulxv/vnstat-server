use anyhow::Result;
use log::info;
use tokio::{
    signal::unix::{signal, SignalKind},
    spawn,
};

/// Terminate vnsd process by UNIX Signals
pub async fn terminate_process() -> Result<()> {
    let signals = Vec::from([
        SignalKind::terminate(),
        SignalKind::interrupt(),
        SignalKind::quit(),
    ]);

    for sig in signals {
        spawn(async move {
            if let Some(_) = signal(sig)?.recv().await {
                info!("{:?} recevied, the process is shutdown now.", sig);
                std::process::exit(1);
            }
            Result::<()>::Ok(())
        });
    }

    Ok(())
}
