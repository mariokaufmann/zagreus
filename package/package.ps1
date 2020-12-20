$ErrorActionPreference = "Stop"

if (Test-Path .\target) {
    Remove-Item .\target -Recurse -Force
}
if (Test-Path .\zagreus.zip) {
    Remove-Item .\zagreus.zip -Force
}


New-Item -ItemType Directory -Path .\target | Out-Null
Copy-Item ..\zagreus-runtime\dist\zagreus-runtime.js .\target
Copy-Item ..\zagreus-generator\target\release\zagreus-generator.exe target\
Copy-Item ..\zagreus-server\target\release\zagreus-server.exe target\
Compress-Archive -Path .\target\* -DestinationPath .\zagreus.zip