# Zagreus - Web graphic overlays

Zagreus is an application which allows you to control graphic overlays built with web technologies over a convenient
HTTP API.

Graphics can be designed in any technologies that can run in a browser. This can be a complex application with
frameworks such as React, Svelte, Angular etc. but it can also be a simple HTML template with some basic styling. By
running the zagreus server and then embedding a small script into your graphics the server will take care of the
communication between the running graphics and any other software which desires to control the graphics. This can for
example be used to create a graphics overlay and then play it out in any playout/streaming software that supports web
browser overlays (such as OBS, vMix, CasparCG, etc.).
Zagreus also offers the possibility to upload and embed dynamic assets (such as images) during runtime. This can be
useful if some of the used assets (e.g. team logos) are not available yet when the graphics are created.

![Demo](./docs/step-by-step/scoreboard.gif)
## Installation

Download the newest release version from the [releases page](https://github.com/mariokaufmann/zagreus/releases/latest).
Zagreus does not need to be installed. The zip archive can be unpacked and zagreus can be executed. To make work with
the zagreus server easier it might make sense to add the folder to the `PATH` variable on your system.

## Quick start

1. Create a folder `my-template`
2. Create an HTML file `index.html` with the following contents:

```html
<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="utf-8" />
  <title>zagreus template</title>
  <style>
      * {
          font-family: sans-serif;
          font-size: 25px;
      }
      .background {
          width: 200px;
          height: 100px;
          display: flex;
          justify-content: center;
          align-items: center;
          background-color: #0E1428;
      }
      .background p {
          color: white;
      }
      @keyframes fadeOut {
          from {
              opacity: 1;
          }
          to {
              opacity: 0;
          }
      }
  </style>
</head>
<body>
<div id="root" data-zag="zagreus-container" class="zagreus-hidden">
  <div class="background" data-zag="Background">
    <p data-zag="Text">Example text</p>
  </div>
</div>
<script src="http://localhost:58180/static/zagreus-runtime.js"></script>
<script>
  window.zagreus.setup({
    host: "localhost",
    port: 58180,
    instance: 'my-template',
    container: {
      name: 'zagreus-container',
      width: 1920,
      height: 1080,
    },
    animationSequences: [
      {
        name: "Hide",
        steps: [
          {
            duration: 500,
            animations: [
              {
                id: "Background",
                name: "fadeOut"
              }
            ]
          }
        ]
      }
    ]
  });
</script>
</body>
</html>
```
3. Start the zagreus server (see _Installation_ above)
```shell
zagreus-server
```
4. Load the template file (`index.html`) in your playout software. Example: if you use OBS, add a browser source with a local file, set width to 1920 pixels, height to 1080 pixels
5. Navigate to the [Swagger UI of zagreus](http://localhost:58180/static/swagger-docs/?url=spec.yaml) and manipulate the template. To set the text dynamically, use the `data/text` endpoint. To trigger an animation, use the `data/animation` endpoint. Pass in `my-template` as instance name.

## Additional documentation

- For a more involved and thorough introduction on how to use zagreus check out the [step-by-step guide](docs/step-by-step/setup.md)
- For more information about how to actually overlay the graphics over video check out the [playout](docs/step-by-step/playout.md) section