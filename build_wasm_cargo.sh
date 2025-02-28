#!/bin/sh

rm -rf target

export WASI_SDK_PATH=/opt/wasi-sdk/
rustup target add wasm32-wasip1

# Config in Cargo.toml
#rustflags = [
#  "-C", "link-arg=--initial-memory=65536",
#  "-C", "link-arg=-zstack-size=8192",
#  "-C", "link-arg=--export=__heap_base",
#  "-C", "link-arg=--export=__data_end",
#  "-C", "link-arg=--strip-all",
#]
#

echo "Build wasm app from cargo build .."
echo "Generate luminance.wasm .."
cargo build --release --target wasm32-wasip1


echo "Generate wasm_image_processor.h .."
/home/zzhou21/wasm-micro-runtime/test-tools/binarydump-tool/build/binarydump \
        -o wasm_image_processor.h \
        -n wasm_image_processor \
           target/wasm32-wasip1/release/image_processor.wasm

echo "Done"
