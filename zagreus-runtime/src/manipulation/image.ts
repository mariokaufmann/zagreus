import { AssetSource } from "../websocket/types";
import { getUrlOnServer } from "../runtime";
import { getZagreusElement } from "../utils";

const getAssetUrl = (asset: string, assetSource: AssetSource): string => {
  if (assetSource === "zagreus") {
    return getUrlOnServer(`/assets/${asset}`);
  }
  return asset;
};

export const setImageSource = (
  elementName: string,
  asset: string,
  assetSource: AssetSource
): void => {
  const url = getAssetUrl(asset, assetSource);
  const element = getZagreusElement<HTMLImageElement>(elementName);
  if (element.tagName !== "img") {
    throw new Error(
      `Cannot set image source on element ${elementName} since it its not an img element.`
    );
  }
  element.setAttribute("src", url);
};
