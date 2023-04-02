cp -f ../package/zagreus-linux.zip .
unzip -q zagreus-linux.zip -d ./unpack

cd unpack/ && ./zagreus-server --server-port 8080 --data-folder ../zagreus-data &
cd template && npx http-server --port 3000 &

