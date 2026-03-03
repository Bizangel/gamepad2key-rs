set -e
cd "$(dirname "$0")"
cd ..

cargo build --release
sudo mv target/release/gamepad2key-rs /usr/local/bin/gamepad2key
sudo chown root:root /usr/local/bin/gamepad2key
sudo chmod 755 /usr/local/bin/gamepad2key
