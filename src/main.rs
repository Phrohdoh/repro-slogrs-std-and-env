use slog::Drain;

fn main() {
    let decorator = slog_term::TermDecorator::new().stdout().build();
    let fmt = slog_term::CompactFormat::new(decorator).build().fuse();
    let drain = slog_envlogger::new(fmt);
    let (drain, _) = slog_async::Async::new(drain).build_with_guard(); // changing this to `build` changes nothing about the output

    let logger = slog::Logger::root(
        drain.fuse(),
        slog::o!(
            "some" => "thing",
        )
    );

    let _ = slog_stdlog::init().unwrap(); // changing this to `slog_envlogger::init` changes nothing about the output
    slog_scope::scope(&logger, slog_std_env::run);
}