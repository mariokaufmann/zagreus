import {getZagreusState} from "../data/data";
import {Alignment} from "../websocket/types";

export const setImageSource = (elementName: string, asset: string) => {
    const url = `assets/${asset}`;
    const element = document.getElementById(elementName);
    if (element) {
        if (element.tagName === 'use') {
            replaceUseElement(<SVGUseElement><Element>element, elementName, url);
        } else if (element.tagName === 'image') {
            const imageElement = <SVGImageElement><Element>element;
            const boundingBox = imageElement.getBoundingClientRect();
            imageElement.setAttribute('href', url);
            alignImage(imageElement, elementName, boundingBox);
        }
    }
}

// TODO remove this as soon as all use elements are flattened (see use.ts)
const replaceUseElement = (element: SVGUseElement, elementName: string, url: string) => {
    const boundingBox = element.getBoundingClientRect();
    const parentElement = element.parentElement;

    const imageElement = document.createElementNS('http://www.w3.org/2000/svg', 'image');
    const attributes = element.attributes;
    for (let i = 0; i < attributes.length; i++) {
        const attribute = attributes[i];
        imageElement.setAttribute(attribute.name, attribute.value);
    }
    imageElement.setAttribute('href', url);
    imageElement.setAttributeNS('http://www.w3.org/1999/xlink', 'href', url);
    parentElement.replaceChild(imageElement, element);

    alignImage(imageElement, elementName, boundingBox);
}

const alignImage = (element: SVGImageElement, elementName: string, originalImageBoundingBox: DOMRect) => {
    const state = getZagreusState();

    const config = state.elementConfigs.find(config => config.id === elementName)?.align;
    let horizontalAlign: Alignment = "left";
    let verticalAlign: Alignment = "top";
    if (config) {
        horizontalAlign = config.horizontal;
    }
    if (config) {
        verticalAlign = config.vertical;
    }

    let newX = originalImageBoundingBox.x;
    if (horizontalAlign === 'right') {
        newX += originalImageBoundingBox.width;
    } else if (horizontalAlign === 'center') {
        const alignmentState = state.alignmentStates[config.with];
        if (!alignmentState) {
            console.error(`Align with element ${config.with} could not be found.`);
            return;
        }
        const updatedImageBoundingBox = element.getBoundingClientRect();
        const alignWithBoundingBox = alignmentState.elementBoundingBox;
        newX = alignWithBoundingBox.x + (alignWithBoundingBox.width / 2) - (updatedImageBoundingBox.width / 2);
    } else {
        console.error(`Invalid value ${horizontalAlign} for horizontal alignment on element ${elementName} provided.`);
        return;
    }

    let newY = originalImageBoundingBox.y;
    if (verticalAlign === 'bottom') {
        newY += originalImageBoundingBox.height;
    } else if (verticalAlign === 'center') {
        const alignmentState = state.alignmentStates[config.with];
        if (!alignmentState) {
            console.error(`Align with element ${config.with} could not be found.`);
            return;
        }
        const updatedImageBoundingBox = element.getBoundingClientRect();
        const alignWithBoundingBox = alignmentState.elementBoundingBox;
        newY = alignWithBoundingBox.y + (alignWithBoundingBox.height / 2) - (updatedImageBoundingBox.height / 2);
    } else {
        console.error(`Invalid value ${verticalAlign} for vertical alignment on element ${elementName} provided.`);
        return;
    }

    element.setAttribute('x', String(newX));
    element.setAttribute('y', String(newY));
}
