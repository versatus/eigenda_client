#[macro_export]
macro_rules! grpcurl_command {
    ($bin_path:expr, $($arg:expr),*) => {{
        let mut command = std::process::Command::new($bin_path.unwrap_or(&"grpcurl".to_string()));
        $(command.arg($arg);)*
        match command.output() {
            Ok(o) => Ok(o),
            Err(e) => {
                use std::fmt::Write;
                let args = [$($arg,)*];
                let mut args_str = String::with_capacity(args.len());
                args.iter()
                    .for_each(|arg| write!(args_str, "{arg} ").expect("failed to write grpcurl args into string buffer."));
                Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "grpcurl command failed: grpcurl {}\nError: {e:?}",
                        args_str
                    ),
                ))
            }
        }
    }};
}
