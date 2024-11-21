use std::fmt::Debug;
use tracing::{
    span::{Attributes, Id, Record},
    Event, Subscriber,
};
use tracing_subscriber::{
    filter::LevelFilter,
    fmt::{self, format::FmtSpan, time::ChronoLocal},
    layer::{Context, SubscriberExt},
    registry::LookupSpan,
    util::SubscriberInitExt,
    Layer,
};

mod emit_macro;
use emit_macro::emit_tracing;

struct DebugLayer;

impl<S> Layer<S> for DebugLayer
where
    S: Subscriber + for<'a> LookupSpan<'a> + Debug,
{
    fn on_new_span(&self, attrs: &Attributes<'_>, id: &Id, _ctx: Context<'_, S>) {
        println!("on_new_span: {{\r\nattrs: {:#?},\r\nid: {:#?}}}", attrs, id);
    }

    fn on_record(&self, span: &Id, values: &Record<'_>, _ctx: Context<'_, S>) {
        println!(
            "on_record: {{\r\nspan: {:#?},\r\nvalues: {:#?}}}",
            span, values
        );
    }

    fn on_follows_from(&self, span: &Id, follows: &Id, _ctx: Context<'_, S>) {
        println!(
            "on_follows_from: {{\r\nspan: {:#?},\r\nfollows: {:#?}}}",
            span, follows
        );
    }

    fn on_event(&self, event: &Event<'_>, _ctx: Context<'_, S>) {
        println!("on_event: {{\r\nevent: {:#?}}}", event);
    }

    fn on_enter(&self, id: &Id, _ctx: Context<'_, S>) {
        println!("on_enter: {{\r\nid: {:#?}}}", id);
    }

    fn on_exit(&self, id: &Id, _ctx: Context<'_, S>) {
        println!("on_exit: {{\r\nid: {:#?}}}", id);
    }

    fn on_close(&self, id: Id, _ctx: Context<'_, S>) {
        println!("on_close: {{\r\nid: {:#?}}}", id);
    }

    fn on_id_change(&self, old: &Id, new: &Id, _ctx: Context<'_, S>) {
        println!("on_id_change: {{\r\nold: {:#?},\r\nnew: {:#?}}}", old, new);
    }
}

fn init_subscriber() {
    // See: https://github.com/tokio-rs/tracing/issues/3068
    #[cfg(windows)]
    nu_ansi_term::enable_ansi_support().ok();

    tracing_subscriber::registry()
        .with(
            fmt::layer()
                .with_timer(ChronoLocal::new("%m-%d %H:%M:%S".into()))
                .with_span_events(FmtSpan::FULL)
                .with_thread_names(true)
                .with_filter(LevelFilter::INFO),
        )
        .with(DebugLayer)
        .init();
}

#[tokio::main]
async fn main() {
    init_subscriber();
    emit_tracing();
}
