import {
  AnimationDirection,
  AnimationIterationCount,
  AnimationSequence,
  AnimationStep,
} from "../websocket/types";
import { getInternalZagreusState, InternalZagreusState } from "../runtime";
import { getZagreusElement } from "../utils";
import { WebsocketSender } from "../websocket/websocket-sender";

export const applyAnimation = (
  sequenceName: string,
  queueId: string | undefined,
): void => {
  const state = getInternalZagreusState();
  const sequence = findAnimationSequence(sequenceName, state);
  if (sequence) {
    if (queueId) {
      let queue = state.animationQueues[queueId];
      if (!queue) {
        queue = new AnimationQueue(queueId, state.websocketSender);
        state.animationQueues[queueId] = queue;
      }
      queue.enqueueAnimationSequence(sequence);
    } else {
      scheduleAnimationSequence(sequence.steps);
    }
  }
};

const scheduleAnimationSequence = (steps: AnimationStep[]): void => {
  steps.forEach((step, index) => {
    const start = getStartForAnimationStep(steps, index);
    if (start > 0) {
      setTimeout(() => applyAnimationStep(step), start);
    } else {
      applyAnimationStep(step);
    }
  });
};

const applyAnimationStep = (step: AnimationStep): void => {
  step.animations.forEach((element) =>
    applyAnimationToElement(
      element.id,
      element.name,
      element.iterations,
      element.direction,
      step.duration,
    ),
  );
};

const applyAnimationToElement = (
  id: string,
  animationName: string,
  animationIterationCount: AnimationIterationCount,
  animationDirection: AnimationDirection,
  duration: number,
): void => {
  const element = getZagreusElement(id);
  if (element.style.animationName === animationName) {
    // hack: remove animation and call getBoundingClientRect() to trigger reflow to reset the animation
    element.style.animation = "none";
    element.getBoundingClientRect();
  }
  element.style.transformBox = "fill-box";
  element.style.transformOrigin = "0 0";
  element.style.animation = `${duration}ms linear 0s ${animationIterationCount} ${animationDirection} forwards running ${animationName}`;
};

const getStartForAnimationStep = (
  steps: AnimationStep[],
  index: number,
): number => {
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
  const state = getInternalZagreusState();
  let maxTimeout = 0;
  sequences.forEach((sequenceName) => {
    const sequence = findAnimationSequence(sequenceName, state);
    const timeout = getMaxTimeoutFromSequence(sequence);
    maxTimeout = Math.max(maxTimeout, timeout);
  });
  return maxTimeout;
};

const findAnimationSequence = (
  sequenceName: string,
  state: InternalZagreusState,
): AnimationSequence | undefined => state.animationSequences[sequenceName];

const getMaxTimeoutFromSequence = (sequence: AnimationSequence): number => {
  let maxTimeout = 0;
  sequence.steps.forEach((step, index) => {
    const start = getStartForAnimationStep(sequence.steps, index);
    const duration = step.duration;

    maxTimeout = Math.max(maxTimeout, start + duration);
  });
  return maxTimeout;
};

export class AnimationQueue {
  readonly queue: AnimationSequence[] = [];
  currentlyExecutingSequence: AnimationSequence | undefined = undefined;

  constructor(
    private readonly queueName: string,
    private readonly websocketSender: WebsocketSender,
  ) {}

  enqueueAnimationSequence(sequence: AnimationSequence) {
    if (!this.currentlyExecutingSequence) {
      this.executeSequence(sequence);
    } else {
      this.queue.push(sequence);
    }
  }

  private executeNextSequence() {
    const nextSequence = this.queue.shift();
    if (nextSequence) {
      this.executeSequence(nextSequence);
    } else {
      this.currentlyExecutingSequence = undefined;
    }
  }

  private executeSequence(sequence: AnimationSequence) {
    this.currentlyExecutingSequence = sequence;
    const duration = getMaxTimeoutFromSequence(sequence);
    scheduleAnimationSequence(sequence.steps);
    setTimeout(() => {
      this.websocketSender.sendAnimationCompletedMessage(
        this.queueName,
        sequence.name,
      );
      this.executeNextSequence();
    }, duration);
  }
}
