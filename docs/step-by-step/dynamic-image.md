# Dynamic image
Sometimes we also want to add and remove images to our template dynamically.
Images can come from two sources:
1. They are already included in the template. This is the case for our example `image.jpg` that we used in our template. This source is appropriate whenever the images are known beforehand and can therefore be added to template before using the overlay
2. They are uploaded to zagreus at runtime and then provided by zagreus. This source is appropriate whenever the images are dynamic and can change quickly (e.g. team logos at a sports event).

We have already used source option 1 for our `image.jpg` image. Let us explore how to use option 2 (dynamic images).

### Adding the asset at runtime
Sometimes it can also be useful to add an asset dynamically at runtime when using the overlay. This can be achieved through the server API. Go to the server API documentation (reference the chapter about dynamic text for more information about it). Search for the _/asset_ endpoints. With the POST request one can upload an asset dynamically. Enter `image2.jpg` as asset name and select another image from your computer. Then click execute and the asset should be uploaded. Check the response for your request. zagreus will return the name of the newly uploaded asset. Example:
```json
{
  "name": "c6492260f4240be4f8c03699608c51644828f1fb5257251687eff29de44c3d09.jpg"
}
```
Note down this name as we will use it again in the next section.

# Set image content dynamically
Go to the server API documentation. Search for the _data/image_ endpoint. The property asset in the payload determines, which file is set as the image source. To indicate to zagreus that the image was upload dynamically we will set the asset source to `zagreus` (otherwise we would use `template`). Try setting the image source of the _ScoreboardLogoImage_ element to your new image by using the name from the previous section.

```json
{
  "id": "ScoreboardLogoImage",
  "asset": "c6492260f4240be4f8c03699608c51644828f1fb5257251687eff29de44c3d09.jpg",
  "assetSource": "zagreus"
}
```

You should now see that the image above the score board has changed.

Next step: [Animation](animations.md)
