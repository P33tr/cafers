use askama::Template;
use crate::db_module::Cafe;

// Define the HTML template using Askama
#[derive(Template)]
#[template(
    source = r#"
        <h1>Cafes</h1>
        <form action="/add" method="post">
        <input type="text" name="cafe"/>
        <input type="submit" value="Add Cafe"/>
        </form>
        <ul>
        {% for cafe in cafes %}
        <li>{{ cafe }}</li>
        {% endfor %}
        </ul>
    "#,
ext = "html"
)]
pub(crate) struct CafeListTemplate<'a> {
    pub(crate) cafes: &'a Vec<Cafe>,
}