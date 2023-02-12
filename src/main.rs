#[macro_use]
extern crate rocket;

use rocket::fs::relative;
use rocket::fs::FileServer;
use rocket_dyn_templates::context;
use rocket_dyn_templates::Template;
use rocket_sync_db_pools::database;
use rocket_sync_db_pools::rusqlite;
use rocket_sync_db_pools::rusqlite::params;
use rocket_sync_db_pools::rusqlite::Connection;
use serde::Serialize;

#[database("blog_stuff_db")]
struct BlogStuffDB(rusqlite::Connection);

#[derive(FromForm)]
struct LoginDetails {
    username: String,
    password: String,
}

#[derive(Serialize)]
struct BlogPost {
    title: String,
    text: String,
}

#[get("/")]
fn index() -> Template {
    let blog_post_titles_vec = Vec::from(["Welcome to my blog post", "Second Title"]);
    let blog_post_contents_vec = Vec::from([
        "This is a blog post. It's pretty cool and very epic",
        "Even more epic content",
    ]);
    Template::render(
        "home",
        context! {blog_post_titles: blog_post_titles_vec, blog_post_contents: blog_post_contents_vec},
    )
}

#[get("/blog/<id>")]
async fn get_blog_page(id: u32, conn: BlogStuffDB) -> Template {
    let markdown = String::new();
    conn.run(|c| get_blog_post_md(c, id.clone(), &markdown))
        .await;
    Template::render("blog", context! {})
}

fn get_blog_post_md(conn: &Connection, id: u32, markdown_return: &String) {
    conn.execute("SELECT blog_md FROM blogs WHERE id=?1;", params![&id]);
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/files", FileServer::from(relative!("static")))
        .attach(Template::fairing())
}
