pub fn emit_tracing() {
    let ccc = 333;
    const CCC: &str = "ccc";
    let ddd = 444;

    let myspan = tracing::span!(
        target: "span-target",
        tracing::Level::INFO,
        "my-span",
        aaa = 111,
        "bbb" = 222,
        { CCC } = ?ccc,
        ddd = tracing::field::Empty
    );

    myspan.record("ddd", 444);

    tracing::event!(
        parent: &myspan,
        tracing::Level::INFO,
        aaa = 111,
        "bbb" = 222,
        { CCC } = ?ccc,
        "ddd = {}", ddd
    );
}
