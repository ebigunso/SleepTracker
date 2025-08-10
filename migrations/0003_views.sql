-- View backed by server-computed duration_min (DST-safe)
CREATE VIEW v_daily_sleep AS
SELECT
  s.id,
  s.date AS wake_date,
  s.bed_time,
  s.wake_time,
  m.latency_min,
  m.awakenings,
  m.quality,
  m.duration_min
FROM sleep_sessions s
JOIN sleep_metrics m ON m.session_id = s.id;
