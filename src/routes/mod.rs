use actix_web::{HttpResponse, Resource, Responder, Scope, web};

/*use crate::response::authenticate::Authenticate;
use crate::response::errors::Error;
use crate::response::html::HTML;
use crate::response::trow_token::ValidBasicToken;
use crate::response::trow_token::{self, TrowToken};
*/
use crate::{TrowConfig, auth_middleware::AuthGuard};
use std::str;
use actix_web::http::header::ContentType;
use actix_web::HttpServer;


/*
mod blob;
mod catalog;
mod health;
mod manifest;
mod metrics;
mod readiness;
mod validation;
*/


pub fn homepage(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/").route(web::get().to(get_homepage)));
}

pub fn registry_config(cfg: &mut web::ServiceConfig) {
 
    // Root
    cfg.service(web::resource("/").wrap(AuthGuard).route(web::get().to(dummy)));

    //tags
    cfg.service(web::resource("/{name:((?:[^/]*/)*)(.*)}/tags/list")
        .route(web::get().to(dummy)));

    // manifests
    cfg.service(
        web::resource("/{name:((?:[^/]*/)*)(.*)}/manifests/{reference}")
            // get the full manifest
            .route(web::get().to(dummy))

            // get the digest only of the manifest
            .route(web::head().to(dummy))

            // upload a new manifest
            .route(web::put().to(dummy))

            // delete an existing
            .route(web::delete().to(dummy)),
    );

    //blobs
    cfg.service(
        web::resource("/{name:((?:[^/]*/)*)(.*)}/blobs/{reference}")
            // retrieve a blob -
            .route(web::get().to(dummy))

            // check the existence of a blob -
            .route(web::head().to(dummy))

            // Delete the blob -
            .route(web::delete().to(dummy))
    );

    // blob upload
    cfg.service(
        web::resource("/{name:((?:[^/]*/)*)(.*)}/blobs/uploads/{session_id}")
            // upload blob chunks -
            .route(web::patch().to(dummy))

            // get the status of the blob upload via the session_id -
            .route(web::get().to(dummy))

            // cancel the upload of a blob -
            .route(web::delete().to(dummy))

            // complete the upload -
            .route(web::put().to(dummy))

    );

    // Upload start
    cfg.service(
        web::resource("/{name:((?:[^/]*/)*)(.*)}/blobs/uploads/")
            // start the upload of a blob -
            .route(web::post().to(dummy))
    );
}

async fn dummy() -> impl Responder {
    HttpResponse::Ok().body("Dummy response")
}

/*
 * Welcome message
 */
async fn get_homepage<'a>() -> HttpResponse {
    const ROOT_RESPONSE: &str = "<!DOCTYPE html><html><body>
 <h1>Welcome to Trow, the cluster registry</h1>
 </body></html>";

    HttpResponse::Ok().insert_header(
        ContentType::html()
    ).body(ROOT_RESPONSE)
 }

/*
pub fn routes() -> Vec<rocket::Route> {
    routes![
        get_v2root,
        get_homepage,
        login,
        manifest::get_manifest,
        manifest::get_manifest_2level,
        manifest::get_manifest_3level,
        manifest::get_manifest_4level,
        manifest::put_image_manifest,
        manifest::put_image_manifest_2level,
        manifest::put_image_manifest_3level,
        manifest::put_image_manifest_4level,
        manifest::delete_image_manifest,
        manifest::delete_image_manifest_2level,
        manifest::delete_image_manifest_3level,
        manifest::delete_image_manifest_4level,
        blob::get_blob,
        blob::get_blob_2level,
        blob::get_blob_3level,
        blob::get_blob_4level,
        blob::put_blob,
        blob::put_blob_2level,
        blob::put_blob_3level,
        blob::put_blob_4level,
        blob::patch_blob,
        blob::patch_blob_2level,
        blob::patch_blob_3level,
        blob::patch_blob_4level,
        blob::post_blob_upload,
        blob::post_blob_upload_2level,
        blob::post_blob_upload_3level,
        blob::post_blob_upload_4level,
        blob::post_blob_upload_5level,
        blob::delete_blob,
        blob::delete_blob_2level,
        blob::delete_blob_3level,
        blob::delete_blob_4level,
        catalog::list_tags,
        catalog::list_tags_2level,
        catalog::list_tags_3level,
        catalog::list_tags_4level,
        catalog::get_catalog,
        catalog::get_manifest_history,
        catalog::get_manifest_history_2level,
        catalog::get_manifest_history_3level,
        catalog::get_manifest_history_4level,
        validation::validate_image,
        health::healthz,
        readiness::readiness,
        metrics::metrics
    ]
}

pub fn catchers() -> Vec<rocket::Catcher> {
    catchers![not_found, no_auth]
}

/*
 * v2 - throw Empty
 */
#[get("/v2")]
fn get_v2root(_auth_user: TrowToken) -> Json<JsonValue> {
    Json(json!({}))
}


// Want non HTML return for 404 for docker client
#[catch(404)]
fn not_found(_: &Request) -> Json<String> {
    Json("404 page not found".to_string())
}

#[catch(401)]
fn no_auth(_req: &Request) -> Authenticate {
    Authenticate {}
}

/* login should it be /v2/login?
 * this is where client will attempt to login
 *
 * If login is called with a valid bearer token, return session token
 */
#[get("/login")]
fn login(auth_user: ValidBasicToken, tc: State<TrowConfig>) -> Result<TrowToken, Error> {
    trow_token::new(auth_user, tc).map_err(|_| Error::InternalError)
}
*/