cp -f ../package/zagreus-linux.zip .
unzip zagreus-linux.zip -d ./unpack -q

./unpack/zagreus-generator new e2e-template

# copy fixtures
cp -f ./fixtures/template.svg e2e-template/
cp -f ./fixtures/elements.yaml e2e-template/
cp -f ./fixtures/animations.yaml e2e-template/

cp -f ./fixtures/main.css e2e-template/assets/
cp -f ./fixtures/dragon.png e2e-template/assets/

cd unpack/ && ./zagreus-server &

cd e2e-template/
../unpack/zagreus-generator build --upload

