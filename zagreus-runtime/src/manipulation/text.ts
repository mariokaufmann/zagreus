import {getZagreusState} from "../data/data";
import {Alignment} from "../websocket/types";

export const setTextOnElement = (elementName: string, text: string) => {
    let element = document.getElementById(elementName);
    if (element) {
        setTextOnFoundElement(element, text, elementName);
        return;
    }

    element = document.querySelector(`#${elementName} text`);
    if (element) {
        setTextOnFoundElement(element, text, elementName);
    }
}

export const setTextOnFoundElement = (element: HTMLElement, text: string, elementName: string) => {
    const state = getZagreusState();
    const config = state.elementConfigs.find(config => config.id === elementName)?.align;
    let align: Alignment = "left";
    if (config) {
        align = config.horizontal;
    }

    if (align === "center") {
        setTextAndAlignCenter(element, text, config.with);
    } else if (align === "right") {
        setTextAndAlignRight(element, text);
    } else if (align === "left") {
        setTextAndAlignLeft(element, text);
    }
}

export const setTextAndAlignCenter = (element: HTMLElement, text: string, alignWithElementName: string) => {
    const state = getZagreusState();
    element.textContent = text;

    const alignmentState = state.alignmentStates[alignWithElementName];
    if (!alignmentState) {
        console.error(`Align with element ${alignWithElementName} could not be found.`);
        return;
    }
    const boundingRect = alignmentState.elementBoundingBox;
    element.style.textAnchor = "middle";
    element.setAttribute('x', String(boundingRect.x + (boundingRect.width / 2)));
};

export const setTextAndAlignLeft = (element: HTMLElement, text: string) => {
    element.textContent = text;
};

export const setTextAndAlignRight = (element: HTMLElement, text: string) => {
    const boundingBox = element.getBoundingClientRect();
    element.textContent = text;

    if (element.style.textAnchor !== 'end') {
        element.style.textAnchor = 'end';
        const lowerRightCorner = boundingBox.x + boundingBox.width;
        element.setAttribute('x', String(lowerRightCorner));
    }
}
