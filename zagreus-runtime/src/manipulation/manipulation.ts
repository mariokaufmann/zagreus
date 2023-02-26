// export const scaleBoundingBoxToViewBox = (state: ZagreusRuntimeState, boundingBox: DOMRect): DOMRect => {
//     const viewBoxScaling = state.viewBoxScaling;
//     return <DOMRect>{
//         x: boundingBox.x * viewBoxScaling,
//         y: boundingBox.y * viewBoxScaling,
//         width: boundingBox.width * viewBoxScaling,
//         height: boundingBox.height * viewBoxScaling,
//     };
// };

// export const saveInitialAlignmentStates = (elements: TemplateElement[]): void => {
//     const state = getZagreusState();
//
//     if (state.alignmentStates) {
//         // only save alignment states on initial load
//         return;
//     }
//
//     state.viewBoxScaling = getViewBoxScaling();
//     state.alignmentStates = {};
//     elements
//         .filter(element => element.config !== null)
//         .map(element => element.config)
//         .map(config => config.align)
//         .filter(config => config.with && config.with.length > 0)
//         .filter(config => config.horizontal === 'center' || config.vertical === 'center')
//         .forEach(config => saveAlignmentStateForElement(state, config.with));
// };

// const saveAlignmentStateForElement = (state: ZagreusRuntimeState, elementName: string): void => {
//     const alignmentState = state.alignmentStates[elementName];
//     if (alignmentState) {
//         // only measure once
//         return;
//     }
//
//     state.alignmentStates[elementName] = getInitialAlignmentStateForElement(state, elementName,);
// };

// const getInitialAlignmentStateForElement = (state: ZagreusRuntimeState, elementName: string): AlignmentState => {
//     const element = document.getElementById(elementName);
//     if (!element) {
//         reportErrorMessage(`Could not find alignment element ${elementName}.`);
//         return undefined;
//     }
//
//     const boundingBox = element.getBoundingClientRect();
//
//     return {
//         elementBoundingBox: scaleBoundingBoxToViewBox(state, boundingBox),
//     };
// };

// /**
//  * The view box of the SVG element does not necessarily need to have the same pixel width and height as the SVG element
//  * itself has. Therefore we need to know the scaling factor.
//  */
// const getViewBoxScaling = (): number => {
//     const svgElement = getZagreusSvgElement();
//     return svgElement.viewBox.baseVal.width / svgElement.width.baseVal.value;
// };

// const getZagreusSvgElement = (): SVGSVGElement => {
//     const elements = document.querySelectorAll('#zagreus-svg-container svg');
//     if (elements.length !== 1) {
//         reportErrorMessage('Expected exactly one SVG container element, found ' + elements.length);
//     }
//     return <SVGSVGElement>elements[0];
// };
