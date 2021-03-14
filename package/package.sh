#!/bin/bash

rm -rf ./target
rm -f "zagreus-$1.zip"

mkdir ./target
cp ../zagreus-runtime/dist/zagreus-runtime.js target
cp ../zagreus-generator/target/release/zagreus-generator target/
cp ../zagreus-server/target/release/zagreus-server target/
cp -r ../zagreus-server/swagger-docs target/

cd target/ && zip -r "../zagreus-$1.zip" *
