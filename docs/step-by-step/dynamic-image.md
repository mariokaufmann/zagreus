# Dynamic image
Static images can just be embedded into the SVG file already in the SVG editor. However, with zagreus it is also possible to set image content in a template dynamically.
We are going to add a venue logo above the scoreboard that can be set dynamically without having to rebuild the template.

Open up your template file in the editor of your choice, and a rectangle above the main background. Name this rectangle _ScoreboardLogoBackground_. Add it to the _Scoreboard_ group.
Add a logo of as a placeholder to the background. We use [this icon of a fish](./img/fish.png). The icons used in this section were adapted from [Font Awesome](https://fontawesome.com/icons?d=gallery&m=free) and are licensed under the [Creative Commons Attribution 4.0 International license](https://fontawesome.com/license). Give the image placeholder the id _ScoreboardLogoImage_.

## Configure image element
Similar to the text element we need to configure the image so that it is aligned within its background element. Open _elements.yaml_ and add the following configuration:
```yaml
- id: ScoreboardLogoImage
  align:
    horizontal: center
    vertical: center
    with: ScoreboardLogoBackground
```
This centers the element both horizontally and vertically in the background element.

## Add asset
To be able to use our second icon as image we can add it as an asset. We are going to use [the icon of a dragon](./img/dragon.png). Add the file to the _asset_ folder in your template directory. Alternatively, you can also add it to the asset folder of the template directly on the server. Check out the documentation about assets for more information (TODO).
Rebuild the template and upload it to the server. If the zagreus generator is still running in watch mode this should happen automatically once you add the file to the _asset_ folder.

# Set image content dynamically
Go to the server API documentation (read the chapter about dynamic text if you haven't already). Search for the _data/image_ endpoint. The property asset in the payload determines, which file is set as the image source. Try setting the image source of the _ScoreboardLogoImage_ element to the dragon image (`"asset" : "dragon.png"`). You can now see that the logo was replaced with a dragon.