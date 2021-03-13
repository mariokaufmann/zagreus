#!/bin/bash

rm -rf ./target
rm -f zagreus-linux.zip

mkdir ./target
cp ../zagreus-runtime/dist/zagreus-runtime.js ./target
cp ../zagreus-generator/target/release/zagreus-generator target/
cp ../zagreus-server/target/release/zagreus-server target/

cd target/ && zip -r ../zagreus-linux.zip *