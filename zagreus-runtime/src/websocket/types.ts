export interface TaggedEnumType<T extends string, P = unknown> {
    tag: T;
    payload?: P;
}

export type EnumTypeHandler<T extends string, D> = {
    // eslint-disable-next-line
    [name in T]: (payload: any, data: D) => void;
};

export type TemplateMessage =
    "SetText"
    | "LogError"
    | "OnLoad"
    | "AddClass"
    | "RemoveClass"
    | "LoadAnimations"
    | "LoadTexts"
    | "ExecuteAnimation"
    | "SetImageSource";

export type SetTextPayload = { id: string, text: string };
export type OnLoadPayload = { animationSequences: string[] };
export type LoadAnimationsPayload = { animations: AnimationSequence[] };
export type LoadTextsPayload = { textElements: TextElementConfig[] };
export type ExecuteAnimationPayload = { animationSequence: string };
export type ManipulateClassPayload = { id: string, class: string };
export type SetImageSourcePayload = { id: string, asset: string };
export type LogErrorPayload = { message: string, stack: string };

export interface AnimationSequence {
    name: string,
    steps: AnimationStep[],
}

export interface AnimationStep {
    start: number,
    duration: number,
    animations: Animation[],
}

export interface Animation {
    id: string,
    name: string,
    direction: AnimationDirection;
}

export type AnimationDirection = "normal" | "reverse";

export interface TextElementConfig {
    id: string,
    align: TextAlignment,
    alignWith: string,
}

export type TextAlignment = "center" | "left" | "right";
