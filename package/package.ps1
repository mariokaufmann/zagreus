$ErrorActionPreference = "Stop"

if (Test-Path .\target) {
    Remove-Item .\target -Recurse -Force
}
if (Test-Path .\zagreus-windows.zip) {
    Remove-Item .\zagreus-windows.zip -Force
}


New-Item -ItemType Directory -Path .\target | Out-Null
Copy-Item ..\zagreus-runtime\dist\zagreus-runtime.js .\target
Copy-Item ..\target\release\zagreus-generator.exe target\
Copy-Item ..\target\release\zagreus-server.exe target\
Copy-Item ..\zagreus-server\swagger-docs target\ -Recurse
Compress-Archive -Path .\target\* -DestinationPath .\zagreus-windows.zip
