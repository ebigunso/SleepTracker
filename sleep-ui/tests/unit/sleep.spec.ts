import { describe, it, expect } from 'vitest';
import {
  computeSegments,
  formatDurationHMM,
  formatDurationMin,
  formatIsoTime,
  formatMinutesAsTime,
  formatTimeHHMM
} from '../../src/lib/utils/sleep';
import {
  applyDayTypeUsualTimes,
  getDurationWarningBoundsFromMetric,
  getDurationWarningMessage,
  selectPrioritizedTrendsExplanation,
  shouldShowDayTypeUsualTimesAction,
  type ActionRecommendation,
  type DurationBaselineMetric
} from '../../src/lib/api';

describe('computeSegments', () => {
  it('returns single segment when no wrap (22:00 -> 23:00)', () => {
    const segs = computeSegments('22:00:00', '23:00:00');
    expect(segs).toHaveLength(1);
    expect(segs[0]).toEqual({ start: 22 * 60, end: 23 * 60 });
  });

  it('wraps across midnight (23:00 -> 06:00) into two segments', () => {
    const segs = computeSegments('23:00:00', '06:00:00');
    expect(segs).toHaveLength(2);
    expect(segs[0]).toEqual({ start: 0, end: 6 * 60 });
    expect(segs[1]).toEqual({ start: 23 * 60, end: 24 * 60 });
  });
});

describe('formatDurationHMM', () => {
  it('formats minutes as h:mm', () => {
    expect(formatDurationHMM(125)).toBe('2:05');
  });

  it('rounds and clamps negative values', () => {
    expect(formatDurationHMM(89.6)).toBe('1:30');
    expect(formatDurationHMM(-5)).toBe('0:00');
  });

  it('returns em dash for invalid input', () => {
    expect(formatDurationHMM(null)).toBe('—');
    expect(formatDurationHMM(undefined)).toBe('—');
    expect(formatDurationHMM(Number.NaN)).toBe('—');
  });
});

describe('formatMinutesAsTime', () => {
  it('formats minutes as HH:MM in 24h time', () => {
    expect(formatMinutesAsTime(90)).toBe('01:30');
    expect(formatMinutesAsTime(24 * 60)).toBe('00:00');
  });

  it('wraps negative minutes', () => {
    expect(formatMinutesAsTime(-30)).toBe('23:30');
  });

  it('returns em dash for invalid input', () => {
    expect(formatMinutesAsTime(null)).toBe('—');
    expect(formatMinutesAsTime(undefined)).toBe('—');
    expect(formatMinutesAsTime(Number.NaN)).toBe('—');
  });
});

describe('formatTimeHHMM', () => {
  it('formats HH:mm:ss as HH:MM', () => {
    expect(formatTimeHHMM('23:15:00')).toBe('23:15');
  });

  it('returns em dash for missing input', () => {
    expect(formatTimeHHMM(null)).toBe('—');
    expect(formatTimeHHMM(undefined)).toBe('—');
  });
});

describe('formatIsoTime', () => {
  it('pads hours and minutes for HH:mm inputs', () => {
    expect(formatIsoTime('8:5')).toBe('08:05');
    expect(formatIsoTime('9:45')).toBe('09:45');
  });

  it('returns fallback when input is empty or incomplete', () => {
    expect(formatIsoTime('')).toBe('—');
    expect(formatIsoTime('7')).toBe('7');
  });
});

describe('formatDurationMin', () => {
  it('formats minutes into human readable strings', () => {
    expect(formatDurationMin(45)).toBe('45m');
    expect(formatDurationMin(60)).toBe('1h');
    expect(formatDurationMin(75)).toBe('1h 15m');
  });

  it('handles invalid or negative values', () => {
    expect(formatDurationMin(null)).toBe('—');
    expect(formatDurationMin(Number.NaN)).toBe('—');
    expect(formatDurationMin(-5)).toBe('0m');
  });
});

describe('personalization warning message', () => {
  const recommendedMedium: ActionRecommendation = {
    action_key: 'personal_duration_warning_tuning',
    status: 'recommended',
    confidence: 'medium',
    rationale: 'Use personalized duration warning range.',
    suppression_reasons: []
  };

  it('uses personalized unusual-duration warning when duration baseline is eligible', () => {
    const metric: DurationBaselineMetric = {
      eligible: true,
      sample_days: 28,
      p10_min: 390,
      p50_min: 450,
      p90_min: 510,
      iqr_min: 50,
      recent_out_of_range_incidence_pct: 10
    };

    const bounds = getDurationWarningBoundsFromMetric(recommendedMedium, metric);
    expect(bounds).toEqual({ min: 390, max: 510 });
    expect(getDurationWarningMessage(bounds, formatDurationMin)).toBe(
      'Your recent personal range is 6h 30m to 8h 30m. Proceed anyway?'
    );
  });

  it('falls back to static warning when personalization is unavailable or ineligible', () => {
    const unavailable = getDurationWarningBoundsFromMetric(recommendedMedium, undefined);
    const ineligible = getDurationWarningBoundsFromMetric(recommendedMedium, {
      eligible: false,
      sample_days: 0,
      p10_min: null,
      p50_min: null,
      p90_min: null,
      iqr_min: null,
      recent_out_of_range_incidence_pct: null
    });

    expect(unavailable).toBeNull();
    expect(ineligible).toBeNull();
    expect(getDurationWarningMessage(unavailable)).toBe('The sleep duration is < 2h or > 14h. Proceed anyway?');
    expect(getDurationWarningMessage(ineligible)).toBe('The sleep duration is < 2h or > 14h. Proceed anyway?');
  });

  it('falls back to static warning when duration recommendation is suppressed even if metric is eligible', () => {
    const suppressed: ActionRecommendation = {
      action_key: 'personal_duration_warning_tuning',
      status: 'suppressed',
      confidence: 'low',
      rationale: 'Not enough out-of-range incidence.',
      suppression_reasons: ['recent out-of-range incidence is below 5% trigger']
    };
    const metric: DurationBaselineMetric = {
      eligible: true,
      sample_days: 90,
      p10_min: 390,
      p50_min: 450,
      p90_min: 510,
      iqr_min: 50,
      recent_out_of_range_incidence_pct: 2
    };

    const bounds = getDurationWarningBoundsFromMetric(suppressed, metric);
    expect(bounds).toBeNull();
    expect(getDurationWarningMessage(bounds, formatDurationMin)).toBe(
      'The sleep duration is < 2h or > 14h. Proceed anyway?'
    );
  });
});

describe('day-type usual times action', () => {
  const recommendedMedium: ActionRecommendation = {
    action_key: 'day_type_default_prefill',
    status: 'recommended',
    confidence: 'medium',
    rationale: 'Apply by day type.',
    suppression_reasons: []
  };

  it('shows action only when recommendation is recommended with sufficient confidence and defaults exist', () => {
    expect(shouldShowDayTypeUsualTimesAction(recommendedMedium, true)).toBe(true);
    expect(
      shouldShowDayTypeUsualTimesAction({ ...recommendedMedium, confidence: 'low' }, true)
    ).toBe(false);
    expect(
      shouldShowDayTypeUsualTimesAction({ ...recommendedMedium, status: 'suppressed' }, true)
    ).toBe(false);
    expect(shouldShowDayTypeUsualTimesAction(recommendedMedium, false)).toBe(false);
    expect(shouldShowDayTypeUsualTimesAction(null, true)).toBe(false);
  });

  it('applies day-type default times only when action is enabled', () => {
    expect(
      applyDayTypeUsualTimes('22:00:00', '06:00:00', { bed: '23:30:00', wake: '07:30:00' }, true)
    ).toEqual({ bed: '23:30:00', wake: '07:30:00' });

    expect(
      applyDayTypeUsualTimes('22:00:00', '06:00:00', { bed: '23:30:00', wake: '07:30:00' }, false)
    ).toEqual({ bed: '22:00:00', wake: '06:00:00' });
  });
});

describe('trends prioritized explanation', () => {
  it('prefers regularity recommendation over quality explanation recommendation', () => {
    const regularity: ActionRecommendation = {
      action_key: 'regularity_insight_priority',
      status: 'recommended',
      confidence: 'high',
      rationale: 'Regularity is the strongest driver this week.',
      suppression_reasons: []
    };
    const quality: ActionRecommendation = {
      action_key: 'quality_aligned_factor_explanation',
      status: 'recommended',
      confidence: 'high',
      rationale: 'Quality is influenced by bedtime consistency.',
      suppression_reasons: []
    };

    expect(selectPrioritizedTrendsExplanation(regularity, quality, 'Total sleep time')).toBe(
      'Regularity is the strongest driver this week.'
    );
  });

  it('falls back to quality recommendation, then metric helper when unavailable/ineligible', () => {
    const quality: ActionRecommendation = {
      action_key: 'quality_aligned_factor_explanation',
      status: 'recommended',
      confidence: 'medium',
      rationale: 'Quality recommendation rationale.',
      suppression_reasons: []
    };

    expect(
      selectPrioritizedTrendsExplanation(
        {
          action_key: 'regularity_insight_priority',
          status: 'suppressed',
          confidence: 'medium',
          rationale: 'Suppressed regularity rationale.',
          suppression_reasons: ['insufficient_signal']
        },
        quality,
        'Total sleep time'
      )
    ).toBe('Quality recommendation rationale.');

    expect(selectPrioritizedTrendsExplanation(null, null, 'Total sleep time')).toBe('Total sleep time');
  });
});
