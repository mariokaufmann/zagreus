# Zagreus - SVG graphics generation & playout

Zagreus is a framework to create and playout web graphics templates for livestreams or TV productions. 

Templates can be designed in any editor that supports exporting to SVG (e.g Inkscape, Affinity Designer). With the zagreus generator a HTML template is created from the exported SVG, together with any assets such as images or stylesheets. Finally the template can be played out with the zagreus server in any playout/streaming software that supports web browser overlays (such as OBS, vMix, CasparCG, etc.).

## Installation
Download the newest release version from the releases page. Zagreus does not need to be installed. The zip archive can be unpacked and zagreus can be executed. To make work with the zagreus generator easier it might make sense to add the folder to the `PATH` variable on your system.

## Quick start
The basic workflow for creating and playing out a graphics template with zagreus is described below. For a more detailed step-by-step guide on how to create a template check out our step-by-step guide on how to create a more involved zagreus template (TODO actually create it)
1. Design graphics with an editor that can export to SVG (for example [Inkscape](https://inkscape.org/))
2. Assign ids to all elements that should later be dynamic in the template (text, animations, styling). How an id can be added to an element depends on the editor in use. In some editors it is enough to assign a name to the respective layer.
3. Create a new zagreus template with the help of the zagreus-generator:
    ```
    zagreus-generator new my-template
    ```
    This will create a subfolder named _my-template_ and prepare a zagreus template project named _my-template_.
4. Export the template into the _my-template_ folder as _template.svg_
5. Build the template using the generator:
   ```
    zagreus-generator build
   ```
6. Open a second shell or terminal and start the zagreus-server:
   ```
    zagreus-server
   ```
   This will start up the zagreus server. By default it can be reached at <http://localhost:4300/>.
7. While leaving the server running go back to the other shell/terminal and upload the template to the server:
   ```
    zagreus-generator upload
   ```
8. TODO go to server UI, go to template, copy URL
9. Add to OBS
10. Control template from server UI
11. Profit