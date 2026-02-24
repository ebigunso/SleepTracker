import { describe, expect, it } from 'vitest';
import { syncIntensityState, type ExerciseIntensity } from '../../src/lib/utils/intensity';

describe('syncIntensityState', () => {
  it('keeps intensity unchanged when initial value is unchanged and not dirty', () => {
    const result = syncIntensityState({
      intensity: 'none',
      initialIntensity: 'none',
      previousInitialIntensity: 'none',
      dirty: false
    });

    expect(result).toEqual({
      intensity: 'none',
      previousInitialIntensity: 'none',
      dirty: false
    });
  });

  it('marks state dirty when user selection diverges from initial value', () => {
    const result = syncIntensityState({
      intensity: 'hard',
      initialIntensity: 'none',
      previousInitialIntensity: 'none',
      dirty: false
    });

    expect(result).toEqual({
      intensity: 'hard',
      previousInitialIntensity: 'none',
      dirty: true
    });
  });

  it('syncs to a late initialIntensity update when untouched', () => {
    const result = syncIntensityState({
      intensity: 'none',
      initialIntensity: 'light',
      previousInitialIntensity: 'none',
      dirty: false
    });

    expect(result).toEqual({
      intensity: 'light',
      previousInitialIntensity: 'light',
      dirty: false
    });
  });

  it('does not overwrite user selection after dirty state, even if initial changes later', () => {
    const result = syncIntensityState({
      intensity: 'hard',
      initialIntensity: 'light',
      previousInitialIntensity: 'none',
      dirty: true
    });

    expect(result).toEqual({
      intensity: 'hard',
      previousInitialIntensity: 'light',
      dirty: true
    });
  });

  it('supports all enum values without narrowing issues', () => {
    const values: ExerciseIntensity[] = ['none', 'light', 'hard'];

    for (const value of values) {
      const result = syncIntensityState({
        intensity: value,
        initialIntensity: value,
        previousInitialIntensity: value,
        dirty: false
      });
      expect(result.intensity).toBe(value);
    }
  });
});
