#![allow(missing_docs)]
error_chain::error_chain! {
    foreign_links {
        Fmt(::std::fmt::Error);
        Io(::std::io::Error) #[cfg(unix)];
        ParseInt(::std::num::ParseIntError);
    }

    errors {
        InvalidSteamId(id: String) {
            description("Invalid Steam ID.")
            display("Invalid Steam ID: \"{}\".", id)
        }
    }
}
