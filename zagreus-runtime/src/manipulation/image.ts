export const setImageSource = (elementName: string, asset: string) => {
    const url = `assets/${asset}`;
    const element = document.getElementById(elementName);
    if (element) {
        if (element.tagName === 'use') {
            replaceUseElement(<SVGUseElement><Element>element, url);
        } else if (element.tagName === 'image') {
            const imageElement = <SVGImageElement><Element>element;
            imageElement.setAttribute('href', url);
        }
    }
}

const replaceUseElement = (element: SVGUseElement, url: string) => {
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
}
