use askama::Template;

#[derive(Template)]
#[template(path = "trends.html")]
pub struct TrendsTemplate;
