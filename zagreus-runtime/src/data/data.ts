import {AnimationSequence, TemplateElement} from '../websocket/types';

declare global {
    interface Window {
        zagreusRuntimeState: ZagreusRuntimeState,
    }
}

export interface AlignmentState {
    elementBoundingBox: DOMRect,
}

export type ErrorReporter = (error: Error) => void;

export interface ZagreusRuntimeState {
    animationsSequences: AnimationSequence[],
    elements: TemplateElement[],
    alignmentStates: { [key in string]: AlignmentState },
    viewBoxScaling: number,
    errorReporter: ErrorReporter,
}

if (!window.zagreusRuntimeState) {
    window.zagreusRuntimeState = {
        animationsSequences: [],
        elements: [],
        alignmentStates: undefined,
        viewBoxScaling: 1,
        errorReporter: undefined,
    };
}

export const getZagreusState = (): ZagreusRuntimeState => {
    return window.zagreusRuntimeState;
};


