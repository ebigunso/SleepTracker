#![doc = r#"Views and templates

Server-side HTML rendering using Askama templates. Currently provides the trends page
consumed by the `/trends` route in [`app`].

Template: `templates/trends.html`

[`app`]: crate::app
"#]

use askama::Template;

#[derive(Template)]
#[template(path = "trends.html")]
#[doc = r#"Trends page template.

Renders the trends page used by the `/trends` route.

# Example

```rust,no_run
# use askama::Template;
# fn main() -> Result<(), askama::Error> {
let html = sleep_api::views::TrendsTemplate.render()?;
assert!(!html.is_empty());
# Ok(()) }
```
"#]
pub struct TrendsTemplate;
