import {TemplateElement} from '../websocket/types';

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
        console.error(`Element ${useElement.id} does not have link to referenced element.`);
        return;
    }

    // strip leading # if necessary
    if (referencedElementId.startsWith('#')) {
        referencedElementId = referencedElementId.substr(1, referencedElementId.length - 1);
    }

    const referencedElement = document.getElementById(referencedElementId);
    if (!referencedElement) {
        console.error(`Use element ${useElement.id} references invalid element ${referencedElementId}.`);
        return;
    }

    const clonedElement = <HTMLElement>referencedElement.cloneNode(true);
    const useParentElement = useElement.parentElement;

    // copy some attributes over
    clonedElement.setAttribute('id', useElement.id);
    const transform = useElement.getAttribute('transform');
    if (transform) {
        clonedElement.setAttribute('transform', transform);
    }

    useParentElement.replaceChild(clonedElement, useElement);
};
