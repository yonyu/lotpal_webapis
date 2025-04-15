use utoipa::OpenApi;
use crate::domain::models::Cn649;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::adapters::controllers::get_all,
        crate::adapters::controllers::get_by_id,
        crate::adapters::controllers::create,
        crate::adapters::controllers::update,
        crate::adapters::controllers::delete,
    ),
    components(
        schemas(Cn649)
    ),
    tags(
        (name = "cn649", description = "CN649 Lottery Management API")
    )
)]
pub struct ApiDoc;
