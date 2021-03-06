# Dynamic text

To have a score board that is actually useful we need to be able to set the text content dynamically. In this guide we will use the HTTP Rest API to control the template. Go to http://localhost:58179/static/swagger-docs/?url=spec.yaml. This should bring you to the API documentation for the server. From there you can execute requests agains the server with the _Try it out_ buttons. We will use that functionality for the rest of the guide.

## Set a text
In the documentation look for the _data/text_ endpoint. Click _Try it out_. This should bring up a form in which you can edit the request data before you send it. Make sure that the template name is _my-template_. Edit the request body by typing ScoreboardTimeText for the id and 12:10 for the text. Then, click _Execute_. 

In the browser tab with the open template you should be able to see that the text with the time has been updated. If you want, try setting any of the other texts too. If you want to revert the template to the initial state just reload the tab in the browser.

## Configure dynamic elements
When setting the content for the text elements in the last section you might already have noticed an issue. Depending on the length of the text the text element will not be correctly aligned with its background anymore.
You can fix this by adding an alignment configuration to the element. Open the file _elements.yaml_, delete the empty _[]_ and add your first element configuration:
```yaml
---
elements:
  - id: ScoreboardTimeText
    align:
      horizontal: center
      with: ScoreboardTimeBackground
```
With this added configuration we instruct zagreus to always center the text field in the element _ScoreboardTimeBackground_, independently of the length of the text in the text element.

To see the impact of your change, we can build the template again. You could just run the same command as in the last chapter, but that will get tedious soon. Zagreus offers a _watch_ mode that will watch your template for changes and rebuild the template automatically if necessary.
```shell
zagreus-generator build --watch --upload
```
The `--watch` flag instructs the generator to build the template and then watch the folder for any changes which will trigger another build automatically. With the `--upload` flag the generator will upload the template after every rebuild automatically.
Whenever you now make a change to your SVG file, to the config files or to any asset your template will be built and uploaded to the server on its automatically.

If you get an error when building the template make sure that the indentation of every line is correct.

If you go back to the scoreboard, reload the page and set the text again you should now see that the text is centered. Add the same configuration for the _ScoreboardScoreText_ element, this time aligning to the _ScoreboardMainBackground_.

To find out more about the configuration of individual elements you can check out the documentation here (TODO link).
