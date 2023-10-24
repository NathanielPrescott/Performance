use crate::imagestorage::image_storage_server::ImageStorage;
use crate::imagestorage::{Image, MessageIdentifier, Statement};
use crate::{imagestorage, ImageStorageService, Images, Size};
use tonic::{IntoRequest, Request, Response, Status};

mod imagestorage;

#[tonic::async_trait]
impl ImageStorage for ImageStorageService {
    async fn get_image(
        &self,
        request: Request<imagestorage::Size>,
    ) -> Result<Response<Image>, Status> {
        let size = Images::from_string(request.into_inner().size.as_str())
            .into_request()
            .into_inner()
            .map_err(|e| Status::invalid_argument(e))?;

        Ok(Response::new(Image {
            image: match size {
                Size::Small => self.images.small.clone(),
                Size::Medium => self.images.medium.clone(),
                Size::Large => self.images.large.clone(),
                Size::Original => self.images.original.clone(),
            },
        }))
    }

    async fn get_message(
        &self,
        _request: Request<MessageIdentifier>,
    ) -> Result<Response<Statement>, Status> {
        Ok(Response::new(Statement {
            text: "Service is running and ready to deliver images".to_string(),
        }))
    }
}
