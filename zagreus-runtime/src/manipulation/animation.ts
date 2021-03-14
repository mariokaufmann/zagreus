import {AnimationDirection, AnimationSequence, AnimationStep} from '../websocket/types';
import {getZagreusState, ZagreusRuntimeState} from '../data/data';

export const applyAnimation = (sequenceName: string): void => {
    const state = getZagreusState();
    const sequence = findAnimationSequence(sequenceName, state);
    if (sequence) {
        scheduleAnimationSequence(sequence.steps);
    }
};

const scheduleAnimationSequence = (steps: AnimationStep[]) => {
    steps.forEach((step, index) => {
        const start = getStartForAnimationStep(steps, index);
        if (start > 0) {
            setTimeout(() => applyAnimationStep(step), start);
        } else {
            applyAnimationStep(step);
        }
    });
};

const applyAnimationStep = (step: AnimationStep) => {
    step.animations.forEach(element => applyAnimationToElement(element.id, element.name, element.direction, step.duration));
};

const applyAnimationToElement = (id: string, animationName: string, animationDirection: AnimationDirection, duration: number) => {
    const element = document.getElementById(id);
    if (element) {
        if (element.style.animationName === animationName) {
            // hack: remove animation and call getBoundingClientRect() to trigger reflow to reset the animation
            element.style.animation = 'none';
            element.getBoundingClientRect();
        }
        element.style.transformBox = 'fill-box';
        element.style.transformOrigin = '0 0';
        element.style.animation = `${duration}ms linear 0s 1 ${animationDirection} forwards running ${animationName}`;
    }
};

const getStartForAnimationStep = (steps: AnimationStep[], index: number): number => {
    const step = steps[index];
    if (step.start != 0) {
        return step.start;
    } else if (index > 0) {
        const previousStep = steps[index - 1];
        return getStartForAnimationStep(steps, index - 1) + previousStep.duration;
    } else {
        return 0;
    }
};

export const getMaxTimeoutFromSequences = (sequences: string[]): number => {
    const state = getZagreusState();
    let maxTimeout = 0;
    sequences.forEach(sequenceName => {
        const sequence = findAnimationSequence(sequenceName, state);
        const timeout = getMaxTimeoutFromSequence(sequence);
        maxTimeout = Math.max(maxTimeout, timeout);
    });
    return maxTimeout;
};

const findAnimationSequence = (sequenceName: string, state: ZagreusRuntimeState): AnimationSequence | undefined => {
    return state.animationsSequences.find(currentSequence => currentSequence.name.toLowerCase() === sequenceName.toLowerCase());
};

const getMaxTimeoutFromSequence = (sequence: AnimationSequence): number => {
    let maxTimeout = 0;
    sequence.steps.forEach((step, index) => {
        const start = getStartForAnimationStep(sequence.steps, index);
        const duration = step.duration;

        maxTimeout = Math.max(maxTimeout, start + duration);
    });
    return maxTimeout;
};
