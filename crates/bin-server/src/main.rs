use domain::guess;
use tracing::{debug, error, info, warn};
use tracing_log::LogTracer;

#[tracing::instrument]
fn function(x: u8) {
    debug!("starting...");
    let result = guess(x);

    match result {
        Ok(_) => info!("correct guess with {}!", x),
        Err(err) => error!("wrong guess with {}, {}", x, err),
    }
    warn!("...finished!");
}

fn main() {
    // transmitting logs (from log) to tracing
    LogTracer::init().expect("log tracer initialized");

    // construct a subscriber that prints formatted traces to stdout
    let subscriber = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .finish();
    // use that subscriber to process traces emitted after this point
    tracing::subscriber::set_global_default(subscriber).expect("tracing sub initialized");

    function(4);
    function(5);
    function(6);
}
