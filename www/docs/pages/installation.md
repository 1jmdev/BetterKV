# Installation

BetterKV can be installed via Docker, package managers, or compiled from source.

## Docker

The simplest way to run BetterKV:

```bash
# Latest stable
docker pull betterkv/betterkv:latest

# Specific version
docker pull betterkv/betterkv:8.1.0

# Run with persistence
docker run -d \
  --name betterkv \
  -p 6379:6379 \
  -v /local/data:/data \
  betterkv/betterkv:latest \
  betterkv-server --save 900 1 --save 300 10
```

## Package Managers

### apt (Debian / Ubuntu)

```bash
curl -fsSL https://packages.betterkv.io/gpg | sudo gpg --dearmor -o /usr/share/keyrings/betterkv.gpg

echo "deb [signed-by=/usr/share/keyrings/betterkv.gpg] https://packages.betterkv.io/apt stable main" \
  | sudo tee /etc/apt/sources.list.d/betterkv.list

sudo apt update && sudo apt install betterkv
```

### Homebrew (macOS)

```bash
brew tap betterkv/tap
brew install betterkv
```

Start as a service:

```bash
brew services start betterkv
```

### rpm (RHEL / Fedora)

```bash
sudo dnf install https://packages.betterkv.io/rpm/betterkv-release-latest.rpm
sudo dnf install betterkv
```

## Build from Source

Requires: **GCC 8+** or **Clang 10+**, `make`, `libc`.

```bash
git clone https://github.com/1jmdev/BetterKV.git
cd BetterKV
cargo build -r
```

Run tests:

```bash
cargo test
```

Start the server:

```bash
./target/release/betterkv-server
```

## Verifying the Installation

```bash
betterkv-server --version
# BetterKV server v0.1.0

betterkv-cli ping
# PONG
```

## System Requirements

| Component | Minimum | Recommended |
|-----------|---------|-------------|
| CPU       | 1 core  | 4+ cores    |
| RAM       | 512 MB  | 8+ GB       |
| OS        | Linux 3.10+, macOS 11+ | Linux 5.x+ |
| Disk (AOF)| 10 GB   | SSD 100+ GB |

:::tip
BetterKV is single-threaded for command processing, so a fast single-core CPU matters more than core count. However, I/O threads for persistence benefit from multiple cores.
:::

:::warning
On Linux, set `vm.overcommit_memory = 1` in `/etc/sysctl.conf` to avoid issues with background saves:

```bash
echo 'vm.overcommit_memory = 1' | sudo tee -a /etc/sysctl.conf
sudo sysctl -p
```
:::
