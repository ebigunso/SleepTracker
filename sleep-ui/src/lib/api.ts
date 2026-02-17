/**
 * Thin fetch wrappers for the SleepTracker API.
 * - Always include credentials (cookies)
 * - Attach X-CSRF-Token for mutating requests by mirroring CSRF cookie
 */

export type Json = Record<string, unknown> | unknown[];

function isBrowser(): boolean {
  return typeof window !== 'undefined' && typeof document !== 'undefined';
}

export function getCookie(name: string): string | null {
  if (!isBrowser()) return null;
  const cookies = document.cookie ? document.cookie.split('; ') : [];
  for (const c of cookies) {
    const [k, ...rest] = c.split('=');
    if (decodeURIComponent(k) === name) {
      return decodeURIComponent(rest.join('='));
    }
  }
  return null;
}

export interface CookieOptions {
  path?: string;
  maxAge?: number;
  sameSite?: 'Lax' | 'Strict' | 'None';
  secure?: boolean;
}

export function setCookie(name: string, value: string, options: CookieOptions = {}): void {
  if (!isBrowser()) return;
  const parts = [`${encodeURIComponent(name)}=${encodeURIComponent(value)}`];
  if (options.path) parts.push(`Path=${options.path}`);
  if (typeof options.maxAge === 'number') parts.push(`Max-Age=${options.maxAge}`);
  if (options.sameSite) parts.push(`SameSite=${options.sameSite}`);
  if (options.secure) parts.push('Secure');
  document.cookie = parts.join('; ');
}

/**
 * Reads CSRF token from either "__Host-csrf" (secure) or "csrf" (dev)
 */
export function readCsrfToken(): string | null {
  return getCookie('__Host-csrf') ?? getCookie('csrf');
}

function mergeHeaders(a?: HeadersInit, b?: HeadersInit): Headers {
  const h = new Headers(a ?? {});
  const bH = new Headers(b ?? {});
  bH.forEach((v, k) => h.set(k, v));
  return h;
}

export async function apiGet<T = unknown>(path: string, init: RequestInit = {}): Promise<T> {
  const res = await fetch(path, {
    credentials: 'include',
    ...init,
    method: 'GET',
    headers: mergeHeaders(init.headers),
  });
  if (!res.ok) {
    throw new Error(`GET ${path} failed: ${res.status}`);
  }
  const ct = res.headers.get('content-type') ?? '';
  if (ct.includes('application/json')) {
    return (await res.json()) as T;
  }
  return (await res.text()) as unknown as T;
}

export async function apiPost<T = unknown>(path: string, body?: Json, init: RequestInit = {}): Promise<T> {
  const csrf = readCsrfToken();
  const res = await fetch(path, {
    credentials: 'include',
    ...init,
    method: 'POST',
    headers: mergeHeaders(
      {
        'Content-Type': 'application/json',
        ...(csrf ? { 'X-CSRF-Token': csrf } : {}),
      },
      init.headers
    ),
    body: body !== undefined ? JSON.stringify(body) : init.body,
  });
  if (!res.ok) {
    throw new Error(`POST ${path} failed: ${res.status}`);
  }
  const ct = res.headers.get('content-type') ?? '';
  if (ct.includes('application/json')) {
    return (await res.json()) as T;
  }
  return (await res.text()) as unknown as T;
}

export async function apiPut<T = unknown>(path: string, body?: Json, init: RequestInit = {}): Promise<T> {
  const csrf = readCsrfToken();
  const res = await fetch(path, {
    credentials: 'include',
    ...init,
    method: 'PUT',
    headers: mergeHeaders(
      {
        'Content-Type': 'application/json',
        ...(csrf ? { 'X-CSRF-Token': csrf } : {}),
      },
      init.headers
    ),
    body: body !== undefined ? JSON.stringify(body) : init.body,
  });
  if (!res.ok) {
    throw new Error(`PUT ${path} failed: ${res.status}`);
  }
  const ct = res.headers.get('content-type') ?? '';
  if (ct.includes('application/json')) {
    return (await res.json()) as T;
  }
  return (await res.text()) as unknown as T;
}

export async function apiDelete(path: string, init: RequestInit = {}): Promise<void> {
  const csrf = readCsrfToken();
  const res = await fetch(path, {
    credentials: 'include',
    ...init,
    method: 'DELETE',
    headers: mergeHeaders(
      {
        ...(csrf ? { 'X-CSRF-Token': csrf } : {}),
      },
      init.headers
    ),
  });
  if (!res.ok) {
    throw new Error(`DELETE ${path} failed: ${res.status}`);
  }
}

/**
 * Alias for CSRF getter (dev or secure cookie)
 */
export const getCsrfToken = readCsrfToken;

/**
 * Low-level fetch wrapper that always includes credentials and attaches CSRF for mutating methods.
 */
export async function apiFetch(path: string, init: RequestInit = {}): Promise<Response> {
  const method = (init.method ?? 'GET').toString().toUpperCase();
  const isMutating = method === 'POST' || method === 'PUT' || method === 'DELETE' || method === 'PATCH';
  const csrf = isMutating ? readCsrfToken() : null;
  const headers = isMutating && csrf
    ? mergeHeaders({ 'X-CSRF-Token': csrf }, init.headers)
    : mergeHeaders(init.headers);
  return fetch(path, { credentials: 'include', ...init, headers });
}

// ------------------------------
// Types matching OpenAPI schemas
// ------------------------------
export type IsoDate = string; // YYYY-MM-DD
export type IsoTime = string; // HH:mm:ss

export interface SleepListItem {
  id: number;
  date: IsoDate;
  bed_time: IsoTime;
  wake_time: IsoTime;
  latency_min: number;
  awakenings: number;
  quality: number;
  duration_min: number | null;
  session_count?: number | null;
}

export interface SleepInput {
  date: IsoDate;
  bed_time: IsoTime;
  wake_time: IsoTime;
  latency_min: number;
  awakenings: number;
  quality: number;
}

export interface SleepSession extends SleepInput {
  id: number;
  duration_min?: number | null;
  session_date?: IsoDate | null;
}

export interface ExerciseUpsert {
  date: IsoDate;
  intensity: 'none' | 'light' | 'hard';
}

export interface FrictionTelemetryInput {
  form_time_ms: number;
  error_kind: string | null;
  retry_count: number;
  immediate_edit: boolean;
  follow_up_failure: boolean;
}

export type FrictionProposalConfidence = 'high' | 'medium' | 'low';

export interface FrictionProposalEvidence {
  current_occurrences: number;
  prior_occurrences: number;
  current_submit_count: number;
  prior_submit_count: number;
  current_avg_form_time_ms: number;
  prior_avg_form_time_ms: number;
  current_retry_avg: number;
  prior_retry_avg: number;
  current_follow_up_failure_rate: number;
  prior_follow_up_failure_rate: number;
}

export interface FrictionBacklogProposal {
  rank: number;
  action_key: string;
  observed_evidence: FrictionProposalEvidence;
  expected_benefit: string;
  estimated_minutes_saved_per_week: number;
  confidence: FrictionProposalConfidence;
  persistence_two_windows: boolean;
  rollback_condition: string;
  auto_promoted: boolean;
}

export interface FrictionBacklogWindow {
  from: IsoDate;
  to: IsoDate;
  submit_count: number;
}

export interface FrictionBacklogResponse {
  as_of: IsoDate;
  window_days: number;
  minimum_sample_met: boolean;
  current_window: FrictionBacklogWindow;
  prior_window: FrictionBacklogWindow;
  proposals: FrictionBacklogProposal[];
}

export interface PersonalizationWindow {
  from: IsoDate;
  to: IsoDate;
  logged_days: number;
  missing_days: number;
  missing_days_pct: number;
}

export interface DurationBaselineMetric {
  eligible: boolean;
  sample_days: number;
  p10_min: number | null;
  p50_min: number | null;
  p90_min: number | null;
  iqr_min: number | null;
  recent_out_of_range_incidence_pct: number | null;
}

export interface DayTypeTimingBaselineMetric {
  eligible: boolean;
  weekday_sample_days: number;
  weekend_sample_days: number;
  weekday_bed_median_min: number | null;
  weekday_wake_median_min: number | null;
  weekend_bed_median_min: number | null;
  weekend_wake_median_min: number | null;
  midpoint_stable_across_windows: boolean;
  recent_14_day_diverges_from_baseline: boolean;
}

export interface SocialJetlagMetric {
  eligible: boolean;
  weekend_sample_days: number;
  current_delta_min: number | null;
  prior_delta_min: number | null;
  sustained_two_windows: boolean;
}

export interface ScheduleVariabilityMetric {
  eligible: boolean;
  current_variability_min: number | null;
  prior_variability_min: number | null;
  sustained_two_windows: boolean;
  high_data_gap: boolean;
}

export interface RankedQualityFactor {
  factor: string;
  effect: number;
}

export interface QualityFactorRankingMetric {
  eligible: boolean;
  sessions_with_quality: number;
  distinct_quality_values: number;
  stable_across_adjacent_windows: boolean;
  ranked_factors: RankedQualityFactor[];
}

export interface PersonalizationMetrics {
  duration_baseline: DurationBaselineMetric;
  day_type_timing_baseline: DayTypeTimingBaselineMetric;
  social_jetlag: SocialJetlagMetric;
  schedule_variability: ScheduleVariabilityMetric;
  quality_factor_ranking: QualityFactorRankingMetric;
}

export type RecommendationStatus = 'recommended' | 'suppressed';
export type RecommendationConfidence = 'high' | 'medium' | 'low';

export interface ActionRecommendation {
  action_key: string;
  status: RecommendationStatus;
  confidence: RecommendationConfidence;
  rationale: string;
  suppression_reasons: string[];
}

export interface PersonalizationResponse {
  as_of: IsoDate;
  window_days: number;
  current_window: PersonalizationWindow;
  prior_window: PersonalizationWindow;
  metrics: PersonalizationMetrics;
  recommendations: ActionRecommendation[];
}

export interface DurationWarningBounds {
  min: number;
  max: number;
}

export interface DayTypeDefaultTimes {
  bed: string;
  wake: string;
}

const STATIC_DURATION_WARNING = 'The sleep duration is < 2h or > 14h. Proceed anyway?';

function isRecommended(rec: ActionRecommendation | null | undefined): boolean {
  return rec?.status === 'recommended';
}

export function getDurationWarningBoundsFromMetric(
  recommendation: ActionRecommendation | null | undefined,
  metric: DurationBaselineMetric | null | undefined
): DurationWarningBounds | null {
  if (!recommendation) return null;
  if (!isRecommended(recommendation)) return null;
  if (recommendation.confidence !== 'medium' && recommendation.confidence !== 'high') return null;
  if (!metric?.eligible) return null;
  if (metric.p10_min == null || metric.p90_min == null) return null;
  return { min: metric.p10_min, max: metric.p90_min };
}

export function getDurationWarningMessage(
  bounds: DurationWarningBounds | null,
  formatMinutes?: (value: number) => string
): string {
  if (!bounds) return STATIC_DURATION_WARNING;
  const min = formatMinutes ? formatMinutes(bounds.min) : `${bounds.min}`;
  const max = formatMinutes ? formatMinutes(bounds.max) : `${bounds.max}`;
  return `Your recent personal range is ${min} to ${max}. Proceed anyway?`;
}

export function shouldShowDayTypeUsualTimesAction(
  recommendation: ActionRecommendation | null | undefined,
  hasDayTypeDefault: boolean
): boolean {
  if (!hasDayTypeDefault || !recommendation) return false;
  if (!isRecommended(recommendation)) return false;
  return recommendation.confidence === 'medium' || recommendation.confidence === 'high';
}

export function applyDayTypeUsualTimes(
  currentBed: string,
  currentWake: string,
  dayTypeDefault: DayTypeDefaultTimes | null,
  enabled: boolean
): DayTypeDefaultTimes {
  if (!enabled || !dayTypeDefault) {
    return { bed: currentBed, wake: currentWake };
  }
  return { bed: dayTypeDefault.bed, wake: dayTypeDefault.wake };
}

export function selectPrioritizedTrendsExplanation(
  regularityInsight: ActionRecommendation | null | undefined,
  qualityExplanation: ActionRecommendation | null | undefined,
  fallback: string | undefined
): string | undefined {
  if (isRecommended(regularityInsight)) return regularityInsight?.rationale;
  if (isRecommended(qualityExplanation)) return qualityExplanation?.rationale;
  return fallback;
}

// ------------------------------
// Helper APIs for Sleep/Exercise
// ------------------------------
export async function getRecent(days = 7): Promise<SleepListItem[]> {
  return apiGet<SleepListItem[]>(`/api/sleep/recent?days=${days}`);
}

export async function getRange(from: IsoDate, to: IsoDate): Promise<SleepListItem[]> {
  return apiGet<SleepListItem[]>(`/api/sleep/range?from=${from}&to=${to}`);
}

export async function getSleepById(id: number): Promise<SleepSession> {
  return apiGet<SleepSession>(`/api/sleep/${id}`);
}

export async function createSleep(input: SleepInput): Promise<{ id: number }> {
  return apiPost<{ id: number }>('/api/sleep', input as unknown as Json);
}

export async function updateSleep(id: number, input: SleepInput): Promise<void> {
  await apiPut<void>(`/api/sleep/${id}`, input as unknown as Json);
}

export async function deleteSleep(id: number): Promise<void> {
  await apiDelete(`/api/sleep/${id}`);
}

export async function setUserTimezoneIfSupported(timezone: string): Promise<boolean> {
  if (!timezone) return false;
  try {
    const res = await apiFetch('/api/settings/timezone', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ timezone })
    });
    if (res.ok) return true;
    if (res.status === 404 || res.status === 405 || res.status === 501) return false;
    return false;
  } catch {
    return false;
  }
}

export async function getExerciseIntensity(from: IsoDate, to: IsoDate): Promise<{ date: IsoDate; intensity: 'none' | 'light' | 'hard' }[]> {
  return apiGet<{ date: IsoDate; intensity: 'none' | 'light' | 'hard' }[]>(`/api/exercise/intensity?from=${from}&to=${to}`);
}

export async function upsertExercise(payload: ExerciseUpsert): Promise<{ id: number }> {
  return apiPost<{ id: number }>('/api/exercise', payload as unknown as Json);
}

export async function postFrictionTelemetry(payload: FrictionTelemetryInput): Promise<{ id: number }> {
  return apiPost<{ id: number }>('/api/personalization/friction-telemetry', payload as unknown as Json);
}

export interface FrictionBacklogQuery {
  window_days?: number;
  to?: IsoDate;
}

export async function getFrictionBacklog(query: FrictionBacklogQuery = {}): Promise<FrictionBacklogResponse> {
  const search = new URLSearchParams();
  if (typeof query.window_days === 'number') search.set('window_days', String(query.window_days));
  if (query.to) search.set('to', query.to);
  const qs = search.toString();
  return apiGet<FrictionBacklogResponse>(`/api/personalization/friction-backlog${qs ? `?${qs}` : ''}`);
}

export interface PersonalizationQuery {
  window_days?: number;
  to?: IsoDate;
}

export async function getPersonalization(query: PersonalizationQuery = {}): Promise<PersonalizationResponse> {
  const search = new URLSearchParams();
  if (typeof query.window_days === 'number') search.set('window_days', String(query.window_days));
  if (query.to) search.set('to', query.to);
  const qs = search.toString();
  return apiGet<PersonalizationResponse>(`/api/trends/personalization${qs ? `?${qs}` : ''}`);
}
