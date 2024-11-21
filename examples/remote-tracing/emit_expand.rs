const CCC: &str = "ccc";

fn emit_span() -> tracing::Span {
    let ccc = 333;

    use tracing::__macro_support::Callsite as _;
    static __CALLSITE: tracing::callsite::DefaultCallsite = {
        static META: tracing::Metadata<'static> = {
            tracing_core::metadata::Metadata::new(
                "my-span",
                "span-target",
                tracing::Level::INFO,
                Some("examples\\remote-tracing\\emit_macro.rs"),
                Some(6u32),
                Some("remote_tracing::emit_macro"),
                tracing_core::field::FieldSet::new(
                    &["aaa", "bbb", CCC, "ddd"],
                    tracing_core::callsite::Identifier(&__CALLSITE),
                ),
                tracing::metadata::Kind::SPAN,
            )
        };

        tracing::callsite::DefaultCallsite::new(&META)
    };
    let mut interest = tracing::subscriber::Interest::never();

    if tracing::Level::INFO <= tracing::level_filters::STATIC_MAX_LEVEL
        && tracing::Level::INFO <= tracing::level_filters::LevelFilter::current()
        && {
            interest = __CALLSITE.interest();
            !interest.is_never()
        }
        && tracing::__macro_support::__is_enabled(__CALLSITE.metadata(), interest)
    {
        let meta = __CALLSITE.metadata();

        tracing::Span::new(meta, &{
            let mut iter = meta.fields().iter();

            meta.fields().value_set(&[
                (
                    &Iterator::next(&mut iter).expect("FieldSet corrupted (this is a bug)"),
                    Some(&111 as &dyn tracing::field::Value),
                ),
                (
                    &Iterator::next(&mut iter).expect("FieldSet corrupted (this is a bug)"),
                    Some(&222 as &dyn tracing::field::Value),
                ),
                (
                    &Iterator::next(&mut iter).expect("FieldSet corrupted (this is a bug)"),
                    Some(&tracing::field::debug(&ccc) as &dyn tracing::field::Value),
                ),
                (
                    &Iterator::next(&mut iter).expect("FieldSet corrupted (this is a bug)"),
                    Some(&tracing::field::Empty as &dyn tracing::field::Value),
                ),
            ])
        })
    } else {
        tracing::__macro_support::__disabled_span(__CALLSITE.metadata())
    }
}

fn emit_event(myspan: tracing::Span) {
    let ccc = 333;
    let ddd = 444;

    use tracing::__macro_support::Callsite as _;
    static __CALLSITE: tracing::callsite::DefaultCallsite = {
        static META: tracing::Metadata<'static> = {
            tracing_core::metadata::Metadata::new(
                "event examples\\remote-tracing\\emit_macro.rs:18",
                "remote_tracing::emit_macro",
                tracing::Level::INFO,
                Some("examples\\remote-tracing\\emit_macro.rs"),
                Some(18u32),
                Some("remote_tracing::emit_macro"),
                tracing_core::field::FieldSet::new(
                    &["message", "aaa", "bbb", CCC],
                    tracing_core::callsite::Identifier(&__CALLSITE),
                ),
                tracing::metadata::Kind::EVENT,
            )
        };

        tracing::callsite::DefaultCallsite::new(&META)
    };
    let enabled = tracing::Level::INFO <= tracing::level_filters::STATIC_MAX_LEVEL
        && tracing::Level::INFO <= tracing::level_filters::LevelFilter::current()
        && {
            let interest = __CALLSITE.interest();
            !interest.is_never()
                && tracing::__macro_support::__is_enabled(__CALLSITE.metadata(), interest)
        };

    if enabled {
        (|value_set: tracing::field::ValueSet| {
            let meta = __CALLSITE.metadata();
            tracing::Event::child_of(&myspan, meta, &value_set);
        })({
            let mut iter = __CALLSITE.metadata().fields().iter();

            __CALLSITE.metadata().fields().value_set(&[
                (
                    &Iterator::next(&mut iter).expect("FieldSet corrupted (this is a bug)"),
                    Some(&format_args!("ddd = {0}", ddd) as &dyn tracing::field::Value),
                ),
                (
                    &Iterator::next(&mut iter).expect("FieldSet corrupted (this is a bug)"),
                    Some(&111 as &dyn tracing::field::Value),
                ),
                (
                    &Iterator::next(&mut iter).expect("FieldSet corrupted (this is a bug)"),
                    Some(&222 as &dyn tracing::field::Value),
                ),
                (
                    &Iterator::next(&mut iter).expect("FieldSet corrupted (this is a bug)"),
                    Some(&tracing::field::debug(&ccc) as &dyn tracing::field::Value),
                ),
            ])
        });
    }
}

pub fn emit_tracing() {
    let myspan = emit_span();
    myspan.record("ddd", 444);
    emit_event(myspan);
}
