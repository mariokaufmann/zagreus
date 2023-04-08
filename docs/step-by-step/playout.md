# Playout
Once we have a finished graphics template we want to use, we can integrate it into the playout software we are using.
Theoretically, any playout software that can overlay a browser source over some other content (e.g. a video) should work for this. Examples for this are [OBS](https://obsproject.com/), [vMix](https://www.vmix.com/) or [casparCG](http://casparcg.com/).

The steps below assume that you have a running zagreus server and a zagreus template ready to be played out. If you haven't done this already, you can check out the [step-by-step guide](./step-by-step.md).

## OBS
If you are using [OBS](https://obsproject.com/) you can add a new _Browser_ source for the templates. For a local template file _Local file_ can be checked and the `index.html` file of the template can be selected. For a tepmlate that is served by a webserver, enter the URL of that server.
Don't forget to set the width and height of the template to the right dimensions (e.g. 1920 by 1080 pixels).

## vMix
If you are using [vMix](https://www.vmix.com/) you can use _Add Input -> Browser_ to add the templates to the project.
Enter the URL of the server that hosts the templates and and set the dimensions of the source (e.g. 1920 by 1080 pixels).
Then, enable one of the overlays for the source to overlay the templates over your current video input.

## casparCG
If you are using [casparCG](http://casparcg.com/) you can add the HTML source to the server with the corresponding AMCP command or the CasparCG Client. Refer to the [documentation](https://github.com/CasparCG/help/wiki/HTML-Producer) for how to do that.