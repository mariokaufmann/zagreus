# Dynamic text

To have a score board that is actually useful we need to be able to set the text content dynamically. In this guide we will use the HTTP Rest API to control the template. Go to http://localhost:58180/static/swagger-docs/?url=spec.yaml. This should bring you to the API documentation for the server. From there you can execute requests against the server with the _Try it out_ buttons. We will use that functionality for the rest of the guide.

## Set a text
In the documentation look for the _data/text_ endpoint. Click _Try it out_. This should bring up a form in which you can edit the request data before you send it. Make sure that the instance name is _test-template_. Edit the request body by typing ScoreboardTimeText for the id and 12:10 for the text. Then, click _Execute_. 

In the browser tab with the open template you should be able to see that the text with the time has been updated. If you want, try setting any of the other texts too. If you want to revert the template to the initial state just reload the tab in the browser.

Next step: [Dynamic styling](dynamic-styling.md)
