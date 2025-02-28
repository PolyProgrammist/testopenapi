use utoipa::{OpenApi, ToSchema};

#[derive(schemars::JsonSchema)]
struct CatAttrs {
    tail_length: u64,
}

#[derive(ToSchema)]
struct Pet {
   id: u64,
   name: String,
   age: Option<i32>,
//    cat_attrs: CatAttrs,
}

mod pet_api {
    use crate::{Pet, CatAttrs};
    /// Get pet by id
    ///
    /// Get pet from database by pet id
    #[utoipa::path(
        get,
        path = "/pets/{id}",
        responses(
            (status = 200, description = "Pet found successfully", body = Pet),
            (status = NOT_FOUND, description = "Pet was not found")
        ),
        params(
            ("id" = u64, Path, description = "Pet database id to get Pet for"),
        )
    )]
    async fn get_pet_by_id(pet_id: u64) -> Pet {
        Pet {
            id: pet_id,
            age: None,
            name: "lightning".to_string(),
            // cat_attrs: CatAttrs { tail_length: 7 }
        }
    }
}

#[derive(OpenApi)]
#[openapi(paths(pet_api::get_pet_by_id))]
struct ApiDoc;


fn main() {
    println!("{}", ApiDoc::openapi().to_pretty_json().unwrap());
}
