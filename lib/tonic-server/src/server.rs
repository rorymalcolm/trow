use tonic::{Request, Response, Status};
use tokio::sync::mpsc;

pub mod trow_server {
    
    include!("../../protobuf/out/trow.rs");
}

use trow_server::{
    server::Registry,
    UploadRequest, UploadDetails, CatalogEntry, CatalogRequest, Tag
};

pub struct TrowServer {}

#[tonic::async_trait]
impl Registry for TrowServer {

    async fn request_upload(
        &self,
        request: Request<UploadRequest>,
    ) -> Result<Response<UploadDetails>, Status> {
        println!("Got a request: {:?}", request);

        let reply = UploadDetails {
            uuid: format!("Hello test"),
        };

        Ok(Response::new(reply))
    }

    type GetCatalogStream = mpsc::Receiver<Result<CatalogEntry, Status>>;

    async fn get_catalog(
        &self,
        _request: Request<CatalogRequest>,
    ) -> Result<Response<Self::GetCatalogStream>, Status> {
        unimplemented!()
    }

    type ListTagsStream = mpsc::Receiver<Result<Tag, Status>>;


    async fn list_tags(
        &self,
        _request: Request<CatalogEntry>,
    ) -> Result<Response<Self::ListTagsStream>, Status> {
        unimplemented!()
    }

}