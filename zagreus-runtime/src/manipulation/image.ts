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
  // alignImage(imageElement, elementName);
};
//
// /**
//  * Align the image according to its alignment config. If the element is center aligned another element must be
//  * configured as the 'alignWith' element. The element will then be aligned according to the positioning of that element.
//  * When aligning a center-aligned element we read the alignment state (basically the bounding box of the 'alignWith'
//  * element at template load time.
//  * @param element the image element to align
//  * @param elementName the name (id) of the element to align
//  */
// const alignImage = (element: SVGImageElement, elementName: string) :void=> {
//     const state = getZagreusState();
//
//     const templateElement = state.elements.find(element => element.id === elementName);
//     if (!templateElement) {
//         reportErrorMessage(`Element ${elementName} is not present in template.`);
//         return;
//     }
//     const alignmentConfig = templateElement.config?.align;
//     if (alignmentConfig) {
//         // left, right, top and bottom alignment does not have any effect on images
//         const updatedImageBoundingBox = scaleBoundingBoxToViewBox(state, element.getBoundingClientRect());
//         const alignWithBoundingBox = getAlignmentBoundingBox(state, alignmentConfig);
//         if (!alignWithBoundingBox) {
//             return;
//         }
//
//         if (alignmentConfig.horizontal === 'center') {
//             const newX = alignWithBoundingBox.x + (alignWithBoundingBox.width / 2) - (updatedImageBoundingBox.width / 2);
//             element.setAttribute('x', String(newX));
//         }
//
//         if (alignmentConfig.vertical === 'center') {
//             const newY = alignWithBoundingBox.y + (alignWithBoundingBox.height / 2) - (updatedImageBoundingBox.height / 2);
//             element.setAttribute('y', String(newY));
//         }
//     }
// };

// const getAlignmentBoundingBox = (state: ZagreusRuntimeState, config: AlignmentConfig): DOMRect => {
//     const alignmentState = state.alignmentStates[config.with];
//     if (!alignmentState) {
//         reportErrorMessage(`Align with element ${config.with} could not be found.`);
//         return;
//     }
//     return alignmentState.elementBoundingBox;
// };
