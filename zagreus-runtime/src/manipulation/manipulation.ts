import {ElementConfig, TemplateElement} from "../websocket/types";
import {AlignmentState, getZagreusState, ZagreusRuntimeState} from "../data/data";

export const saveInitialAlignmentStates = (elements: TemplateElement[]) => {
    const state = getZagreusState();

    if (state.alignmentStates) {
        // only save alignment states on initial load
        return;
    }

    state.alignmentStates = {};
    elements
        .filter(element => element.config !== null)
        .map(element => element.config)
        .map(config => config.align)
        .filter(config => config.with && config.with.length > 0)
        .filter(config => config.horizontal === 'center' || config.vertical === 'center')
        .forEach(config => saveAlignmentStateForElement(state, config.with));
};

const saveAlignmentStateForElement = (state: ZagreusRuntimeState, elementName: string) => {
    const alignmentState = state.alignmentStates[elementName];
    if (alignmentState) {
        // only measure once
        return;
    }

    state.alignmentStates[elementName] = getInitialAlignmentStateForElement(elementName);
}

const getInitialAlignmentStateForElement = (elementName: string): AlignmentState => {
    const element = document.getElementById(elementName);
    if (!element) {
        console.error(`Could not find alignment element ${elementName}.`);
        return undefined;
    }

    return {
        elementBoundingBox: element.getBoundingClientRect(),
    }
}
