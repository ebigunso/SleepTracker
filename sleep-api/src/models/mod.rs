#![doc = r#"Data models

Structures and enums used as request/response payloads and DB projections.

Key types: [`SleepInput`], [`SleepSession`], [`ExerciseInput`], [`NoteInput`], [`Quality`], [`Intensity`].

See also: [`repository`] for persistence operations and [`time::compute_duration_min`] for DST-aware duration computation.

[`repository`]: crate::repository
"#]

pub mod exercise;
pub mod intensity;
pub mod note;
pub mod quality;
pub mod sleep;

pub use exercise::ExerciseInput;
#[allow(unused_imports)]
pub use intensity::Intensity;
pub use note::NoteInput;
#[allow(unused_imports)]
pub use quality::Quality;
pub use sleep::{SleepInput, SleepListItem, SleepSession};
