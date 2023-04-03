# Zagreus - Web graphic overlays

Zagreus is an application which allows you to control graphic overlays built with web technologies over a convenient HTTP API.

Graphics can be designed in any technologies that can run in a browser. This can be a complex application with frameworks such as React, Svelte, Angular etc. but it can also be a simple HTML template with some basic styling. By running the zagreus server and then embedding a small script into your graphics the server will take care of the communication between the running graphics and any other software which desires to control the graphics. This can for example be used to create a graphics overlay and then play it out in any playout/streaming software that supports web browser overlays (such as OBS, vMix, CasparCG, etc.).
Zagreus also offers the possibility to upload and embed dynamic assets (such as images) during runtime. This can be useful if some of the used assets (e.g. team logos) are not available yet when the graphics are created.

## Installation
Download the newest release version from the [releases page](https://github.com/mariokaufmann/zagreus/releases/latest). Zagreus does not need to be installed. The zip archive can be unpacked and zagreus can be executed. To make work with the zagreus server easier it might make sense to add the folder to the `PATH` variable on your system.