# Dynamic styling
In the last chapter we added dynamic text to the template.
But sometimes we also want to change other visual aspects of a template. In this chapter we will add some custom styling that can be added and removed at runtime.

## Add styling to template
We want to be able to change the background color of the game time rectangle if the clock is stopped. To do that we will create a stylesheet. All files in the _asset_ subdirectory are included in the template. Create a file _main.css_ in the _asset_ folder and build the template again. The stylesheet will automatically be picked up by the generator and included in the template.

Add the following snippet to _main.css_:
```css
.darker-background {
    background-color: #acacac;
}
```

This defines a CSS class named _darker-background_. The class is not applied anywhere but is now present in the template. Adding or removing a CSS class is one of the ways we can influence an element's appearance dynamically. 
(TODO navigate in UI and add class to element)
When adding and removing the class you should now see the background of the element change.