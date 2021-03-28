import {getZagreusState} from '../data/data';
import {HorizontalAlignment} from '../websocket/types';
import {scaleBoundingBoxToViewBox} from './manipulation';
import {reportErrorMessage} from '../error';

export const setTextOnElement = (elementName: string, text: string):void => {
    let element = document.getElementById(elementName);
    if (element) {
        setTextOnFoundElement(element, text, elementName);
        return;
    }

    element = document.querySelector(`#${elementName} text`);
    if (element) {
        setTextOnFoundElement(element, text, elementName);
    }
};

export const setTextOnFoundElement = (element: HTMLElement, text: string, elementName: string):void => {
    const state = getZagreusState();

    const templateElement = state.elements.find(element => element.id === elementName);
    if (!templateElement) {
        reportErrorMessage(`Element ${elementName} is not present in template.`);
        return;
    }
    const alignmentConfig = templateElement.config?.align;

    let align: HorizontalAlignment = 'left';
    if (alignmentConfig) {
        align = alignmentConfig.horizontal;
    }

    if (align === 'center') {
        setTextAndAlignCenter(element, text, alignmentConfig.with);
    } else if (align === 'right') {
        setTextAndAlignRight(element, text);
    } else if (align === 'left') {
        setTextAndAlignLeft(element, text);
    }
};

export const setTextAndAlignCenter = (element: HTMLElement, text: string, alignWithElementName: string):void => {
    const state = getZagreusState();
    element.textContent = text;

    const alignmentState = state.alignmentStates[alignWithElementName];
    if (!alignmentState) {
        reportErrorMessage(`Align with element ${alignWithElementName} could not be found.`);
        return;
    }
    const boundingRect = alignmentState.elementBoundingBox;
    element.style.textAnchor = 'middle';
    element.setAttribute('x', String(boundingRect.x + (boundingRect.width / 2)));
};

export const setTextAndAlignLeft = (element: HTMLElement, text: string):void => {
    element.textContent = text;
};

export const setTextAndAlignRight = (element: HTMLElement, text: string):void => {
    const state = getZagreusState();
    const boundingBox = scaleBoundingBoxToViewBox(state, element.getBoundingClientRect());
    element.textContent = text;

    if (element.style.textAnchor !== 'end') {
        element.style.textAnchor = 'end';
        const lowerRightCorner = boundingBox.x + boundingBox.width;
        element.setAttribute('x', String(lowerRightCorner));
    }
};
