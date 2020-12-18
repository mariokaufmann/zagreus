import {getZagreusState, TextAlignmentState} from "../data/data";
import {TextAlignment, TextElementConfig} from "../websocket/types";

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
    const config = state.textElementConfigs.find(textConfig => textConfig.id === elementName);
    let align: TextAlignment = "left";
    if (config) {
        align = config.align;
    }

    if (align === "center") {
        setTextAndAlignCenter(element, text, config.alignWith);
    } else if (align === "right") {
        setTextAndAlignRight(element, text);
    } else if (align === "left") {
        setTextAndAlignLeft(element, text);
    }
}

export const setTextAndAlignCenter = (element: HTMLElement, text: string, alignWithElementName: string) => {
    const state = getZagreusState();
    element.textContent = text;

    const alignmentState = state.textAlignmentStates[alignWithElementName];
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

export const saveInitialAlignmentStates = (textConfigs: TextElementConfig[]) => {
    const state = getZagreusState();

    if (state.textAlignmentStates) {
        // only save them on initial load
        return;
    }

    state.textAlignmentStates = {};
    textConfigs
        .filter(textConfig => textConfig.alignWith && textConfig.alignWith.length > 0)
        .forEach(textConfig => state.textAlignmentStates[textConfig.alignWith] = getInitialAlignmentStateForElement(textConfig.alignWith));
};

const getInitialAlignmentStateForElement = (elementName: string): TextAlignmentState => {
    const element = document.getElementById(elementName);
    if (!element) {
        console.error(`Could not find alignment element ${elementName}.`);
        return undefined;
    }

    return {
        elementBoundingBox: element.getBoundingClientRect(),
    }
}
