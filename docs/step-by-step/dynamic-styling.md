# Dynamic styling
In the last chapter we added dynamic text to the template.
But sometimes we also want to change other visual aspects of a template. In this chapter we will add some custom styling that can be added and removed at runtime.

## Add styling to template
We want to be able to hide the game time if the clock is stopped. To do that we will create a stylesheet. All files in the _asset_ subdirectory are included in the template. Create a file _main.css_ in the _asset_ folder and build the template again. The stylesheet will automatically be picked up by the generator and included in the template.

Add the following snippet to _main.css_:
```css
.hidden {
    display: none;
}
```

This defines a CSS class named _hidden_. The class is not applied anywhere but is now present in the template. Adding or removing a CSS class is one of the ways we can influence an element's appearance dynamically.
To do that, reopen the API documentation if you have closed it (read the chapter about dynamic text if you haven't already). Search for the _data/class/add_ endpoint. Set the template name as previously, use ScoreboardTime for the id and set the value of the _class_ property to _hidden_.
When adding and removing the class you should now see the game time appear and disappear.
