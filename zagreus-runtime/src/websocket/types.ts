export interface TaggedEnumType<T extends string, P = unknown> {
    tag: T;
    payload?: P;
}

export type EnumTypeHandler<T extends string, D> = {
    // eslint-disable-next-line
    [name in T]: (payload: any, data: D) => void;
};

export type TemplateMessage =
    'SetText'
    | 'LogError'
    | 'OnLoad'
    | 'AddClass'
    | 'RemoveClass'
    | 'LoadAnimations'
    | 'LoadElements'
    | 'ExecuteAnimation'
    | 'SetImageSource';

export type SetTextPayload = { id: string, text: string };
export type OnLoadPayload = { animationSequences: string[] };
export type LoadAnimationsPayload = { animations: AnimationSequence[] };
export type LoadElementsPayload = { elements: TemplateElement[] };
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
    iterations: AnimationIterationCount;
}

export type AnimationDirection = 'normal' | 'reverse' | 'alternate' | 'alternate-reverse';

export type AnimationIterationCount = 'infinite' | number;

export interface TemplateElement {
    id: string,
    config: ElementConfig | null,
}

export interface ElementConfig {
    id: string,
    align: AlignmentConfig,
}

export interface AlignmentConfig {
    horizontal: HorizontalAlignment,
    vertical: VerticalAlignment,
    with: string,
}

export type HorizontalAlignment = 'center' | 'left' | 'right';
export type VerticalAlignment = 'center' | 'top' | 'bottom';
