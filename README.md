# RUST

error: one of the features ['runtime-actix-native-tls', 'runtime-async-std-native-tls', 'runtime-tokio-native-tls', 'runtime-actix-rustls', 'runtime-async-std-rustls', 'runtime-tokio-rustls'] must be enabled
  --> C:\Users\user\.cargo\registry\src\index.crates.io-6f17d22bba15001f\sqlx-rt-0.5.13\src\lib.rs:9:1
   |
9  | / compile_error!(
10 | |     "one of the features ['runtime-actix-native-tls', 'runtime-async-std-native-tls', \
11 | |      'runtime-tokio-native-tls', 'runtime-actix-rustls', 'runtime-async-std-rustls', \
12 | |      'runtime-tokio-rustls'] must be enabled"
13 | | );
   | |_^

error: could not compile `sqlx-rt` (lib) due to 1 previous error