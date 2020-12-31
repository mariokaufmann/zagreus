import {getZagreusState, ZagreusRuntimeState} from "../data/data";
import {AlignmentConfig, HorizontalAlignment, VerticalAlignment} from "../websocket/types";

export const setImageSource = (elementName: string, asset: string) => {
    const url = `assets/${asset}`;
    const element = document.getElementById(elementName);
    if (element && element.tagName === 'image') {
        const imageElement = <SVGImageElement><Element>element;
        const boundingBox = imageElement.getBoundingClientRect();
        imageElement.setAttribute('href', url);
        alignImage(imageElement, elementName, boundingBox);
    }
}

/**
 * Align the image according to its alignment config. If the element is center aligned another element must be
 * configured as the 'alignWith' element. The element will then be aligned according to the positioning of that element.
 * When aligning a center-aligned element we read the alignment state (basically the bounding box of the 'alignWith'
 * element at template load time.
 * @param element the image element to align
 * @param elementName the name (id) of the element to align
 * @param originalImageBoundingBox the bounding box of the image element _before_ aligning it
 */
const alignImage = (element: SVGImageElement, elementName: string, originalImageBoundingBox: DOMRect) => {
    const state = getZagreusState();

    const templateElement = state.elements.find(element => element.id === elementName);
    if (!templateElement) {
        console.error(`Element ${elementName} is not configured in template.`);
        return;
    }
    const alignmentConfig = templateElement.config?.align;
    let horizontalAlign: HorizontalAlignment = "left";
    let verticalAlign: VerticalAlignment = "top";
    if (alignmentConfig) {
        horizontalAlign = alignmentConfig.horizontal;
        verticalAlign = alignmentConfig.vertical;
    }

    let newX;
    // TODO reactivate
    // if (horizontalAlign === 'left') {
    //     newX = originalImageBoundingBox.x;
    // } else if (horizontalAlign === 'right') {
    //     const updatedImageBoundingBox = element.getBoundingClientRect();
    //     newX = originalImageBoundingBox.x + originalImageBoundingBox.width - updatedImageBoundingBox.width;
    // } else if (horizontalAlign === 'center') {
    if (horizontalAlign === 'center') {
        const updatedImageBoundingBox = element.getBoundingClientRect();
        const alignWithBoundingBox = getAlignmentBoundingBox(state, alignmentConfig);
        if (alignWithBoundingBox) {
            newX = alignWithBoundingBox.x + (alignWithBoundingBox.width / 2) - (updatedImageBoundingBox.width / 2);
        }
    }

    let newY;
    // TODO reactivate
    // if (verticalAlign === 'top') {
    //     newY = originalImageBoundingBox.y;
    // } else if (verticalAlign === 'bottom') {
    //     const updatedImageBoundingBox = element.getBoundingClientRect();
    //     newY = originalImageBoundingBox.y + originalImageBoundingBox.height - updatedImageBoundingBox.height;
    // } else if (verticalAlign === 'center') {
    if (verticalAlign === 'center') {
        const updatedImageBoundingBox = element.getBoundingClientRect();
        const alignWithBoundingBox = getAlignmentBoundingBox(state, alignmentConfig);
        if (alignWithBoundingBox) {
            newY = alignWithBoundingBox.y + (alignWithBoundingBox.height / 2) - (updatedImageBoundingBox.height / 2);
        }
    }

    if (newX && newY) {
        element.setAttribute('x', String(newX));
        element.setAttribute('y', String(newY));
    }
}

const getAlignmentBoundingBox = (state: ZagreusRuntimeState, config: AlignmentConfig): DOMRect => {
    const alignmentState = state.alignmentStates[config.with];
    if (!alignmentState) {
        console.error(`Align with element ${config.with} could not be found.`);
        return;
    }
    return alignmentState.elementBoundingBox;
}
