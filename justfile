_default:
	@just --list

# Checks with clippy
clippy:
	cargo clippy --locked --all-targets -- -Dwarnings

# Checks the formatting
checkfmt:
	cargo fmt --all -- --check

# Checks clippy and formatting
lint: clippy checkfmt

# Installs binary
install:
	cargo install --path .

# Runs tests
test:
	cargo test --locked --all-targets

# Searches for things which need to be improved
todos:
	rg "(TODO|print(!|ln!)|unwrap\()"

# Cleans build artifacts
clean:
	cargo clean

# Crate compile timings
timings: clean
	cargo build --timings --release

# Runs everything important
all: lint test
