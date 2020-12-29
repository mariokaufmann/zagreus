/* Dynamic elements that have the ability to respect absolute positioning attributes (x/y and width/height) we want to remove
transforms from the SVG editor and replace them with absolute attributes. This has two benefits:
1. It is much easier to position the element with dynamic content
2. It is still possible to animate the element with a CSS animation that animates the 'transform' attribute
CAVEATS:
1) this removes any skewX(), skewY() values the element might have
2) at the moment this is only done for the 2D transformation matrix style of the transform attribute
 */
import {ElementConfig} from "../websocket/types";

export const flattenTransforms = (elementConfigs: ElementConfig[]) => {
    elementConfigs.forEach(flattenTransform);
}

const flattenTransform = (elementConfig: ElementConfig) => {
    const element = document.getElementById(elementConfig.id);
    if (!element) {
        console.error(`Could not find element ${elementConfig.id} when flattening transforms.`);
        return;
    }

    if (element.tagName !== 'image' && element.tagName !== 'use') {
        // only relevant for image elements at the moment
        return;
    }

    const transform = element.getAttribute('transform');
    const prefix = 'matrix(';
    if (transform.startsWith(prefix)) {
        // get actual values, eg. matrix(0.1, 0.2, 0.3, 0.4, 100, 200) -> 0.1, 0.2, 0.3, 0.4, 100, 200
        const values = transform.substr(prefix.length, transform.length - prefix.length - 1);
        const individualValues = values.split(',')
            .map(value => Number(value));
        if (individualValues.length !== 6) {
            // attribute form is not supported yet
            return;
        }

        const scaleX = individualValues[0];
        const scaleY = individualValues[3];
        const translateX = individualValues[4];
        const translateY = individualValues[5];

        setScaledPixelAttribute(element, 'width', scaleX);
        setScaledPixelAttribute(element, 'height', scaleY);
        element.setAttribute('x', translateX.toString());
        element.setAttribute('y', translateY.toString());

        // remove transform
        element.removeAttribute('transform');
    }
}

const setScaledPixelAttribute = (element: HTMLElement, attributeName: string, fraction: number) => {
    let attributeValue = element.getAttribute(attributeName);
    if (!attributeValue || attributeValue.length === 0) {
        console.error(`Expected to find attribute ${attributeName} on element ${element.id} but didn't find it.`);
        return;
    }
    // strip 'px' postfix
    attributeValue = attributeValue.substr(0, attributeValue.length - 2);

    const newValue = Number(attributeValue) * fraction;
    element.setAttribute(attributeName, `${newValue}px`);
}
