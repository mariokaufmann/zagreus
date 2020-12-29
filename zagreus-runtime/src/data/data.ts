import {AnimationSequence, ElementConfig} from "../websocket/types";

declare global {
    interface Window {
        zagreusRuntimeState: ZagreusRuntimeState,
    }
}

export interface AlignmentState {
    elementBoundingBox: DOMRect,
}

export interface ZagreusRuntimeState {
    animationsSequences: AnimationSequence[],
    elementConfigs: ElementConfig[],
    alignmentStates: { [key in string]: AlignmentState },
}

if (!window.zagreusRuntimeState) {
    window.zagreusRuntimeState = {
        animationsSequences: [],
        elementConfigs: [],
        alignmentStates: undefined,
    }
}

export const getZagreusState = (): ZagreusRuntimeState => {
    return window.zagreusRuntimeState;
}


