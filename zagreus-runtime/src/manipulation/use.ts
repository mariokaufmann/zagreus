import {TemplateElement} from '../websocket/types';
import {reportErrorMessage} from '../error';

/**
 SVG documents can contain <use> elements. These reference another element. Through this mechanism, the same element
 can be present in the document without duplicating it.
 For our purposes it is more convenient to duplicate these elements so we can manipulate each copy individually. To do
 this, we clone the referenced element and give the clone the id of the use element.
 */
export const flattenUseElements = (elements: TemplateElement[]): void => {
    elements
        .map(element => document.getElementById(element.id))
        .filter(element => element !== null)
        .filter(element => element.tagName === 'use')
        .map(element => <SVGUseElement><Element>element)
        .forEach(flattenUseElement);
};

const flattenUseElement = (useElement: SVGUseElement): void => {
    let referencedElementId = useElement.getAttribute('href');
    if (!referencedElementId) {
        referencedElementId = useElement.getAttributeNS('http://www.w3.org/1999/xlink', 'href');
    }
    if (!referencedElementId) {
        reportErrorMessage(`Element ${useElement.id} does not have link to referenced element.`);
        return;
    }

    // strip leading # if necessary
    if (referencedElementId.startsWith('#')) {
        referencedElementId = referencedElementId.substr(1, referencedElementId.length - 1);
    }

    const referencedElement = document.getElementById(referencedElementId);
    if (!referencedElement) {
        reportErrorMessage(`Use element ${useElement.id} references invalid element ${referencedElementId}.`);
        return;
    }

    const clonedElement = <HTMLElement>referencedElement.cloneNode(true);
    const useParentElement = useElement.parentElement;

    // copy some attributes over
    clonedElement.setAttribute('id', useElement.id);
    copyAttribute(useElement, clonedElement, 'transform');
    copyAttribute(useElement, clonedElement, 'x');
    copyAttribute(useElement, clonedElement, 'y');
    copyAttribute(useElement, clonedElement, 'width');
    copyAttribute(useElement, clonedElement, 'height');

    useParentElement.replaceChild(clonedElement, useElement);
};

const copyAttribute = (useElement: SVGUseElement, clonedElement: HTMLElement, attributeName: string): void => {
    const value = useElement.getAttribute(attributeName);
    if (value) {
        clonedElement.setAttribute(attributeName, value);
    }
};
