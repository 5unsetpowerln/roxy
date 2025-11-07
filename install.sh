SHARED_PATH="$HOME/.local/share/roxy"

rm -rf $SHARED_PATH
cp -r ./roxy $SHARED_PATH

cargo build --release
cp target/release/roxy /usr/local/bin
