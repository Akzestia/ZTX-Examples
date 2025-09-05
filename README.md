# Examples

# Getting started

Clone repo.

```sh
git clone https://github.com/Akzestia/ZTX-Examples.git
```

Build the project.

```sh
cd ZTX-Examples
cargo build --release
```

Generate Certs

```sh
cd target/release
openssl genrsa -out cert.key 2048
openssl req -new -x509 -days 36500 -key cert.key -out cert.crt -subj "/CN=example.org"
```

Run server

```sh
./server
```

Run Client

```sh
./client
```

Expected Output

```sh
# Server
./server
[2025-09-05T13:33:12Z INFO  zetax_types::server] starting 16 workers on 0.0.0.0:4433
PING!
[2025-09-05T13:33:23Z INFO  tquic::connection] SERVER-941d45db016a35bd idle timeout
[2025-09-05T13:33:23Z INFO  tquic::connection] SERVER-285035ef5cb868d6 idle timeout
[2025-09-05T13:33:23Z INFO  tquic::connection] SERVER-f34333e382743654 idle timeout

# Client
./client
echo => OK
hello world
hello => OK
hello: from-bianry
```
