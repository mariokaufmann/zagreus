import {AnimationSequence, TextElementConfig} from "../websocket/types";

declare global {
    interface Window {
        zagreusRuntimeState: ZagreusRuntimeState,
    }
}

export interface TextAlignmentState {
    elementBoundingBox: DOMRect,
}

export interface ZagreusRuntimeState {
    animationsSequences: AnimationSequence[],
    textElementConfigs: TextElementConfig[],
    textAlignmentStates: { [key in string]: TextAlignmentState },
}

if (!window.zagreusRuntimeState) {
    window.zagreusRuntimeState = {
        animationsSequences: [],
        textElementConfigs: [],
        textAlignmentStates: undefined,
    }
}

export const getZagreusState = (): ZagreusRuntimeState => {
    return window.zagreusRuntimeState;
}


