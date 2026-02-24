export type ExerciseIntensity = 'none' | 'light' | 'hard';

export type IntensityStateInput = {
  intensity: ExerciseIntensity;
  initialIntensity: ExerciseIntensity;
  previousInitialIntensity: ExerciseIntensity;
  dirty: boolean;
};

export type IntensityStateOutput = {
  intensity: ExerciseIntensity;
  previousInitialIntensity: ExerciseIntensity;
  dirty: boolean;
};

export function syncIntensityState(input: IntensityStateInput): IntensityStateOutput {
  let { intensity, initialIntensity, previousInitialIntensity, dirty } = input;

  if (initialIntensity !== previousInitialIntensity) {
    if (!dirty) {
      intensity = initialIntensity;
    }
    previousInitialIntensity = initialIntensity;
  }

  if (!dirty && intensity !== initialIntensity) {
    dirty = true;
  }

  return { intensity, previousInitialIntensity, dirty };
}
