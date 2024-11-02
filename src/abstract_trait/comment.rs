use std::sync::Arc;
use sea_orm::DbErr;
use async_trait::async_trait;

use crate::{
    domain::{ApiResponse, CommentResponse, CreateCommentRequest, UpdateCommentRequest},
    entities::comments,
    utils::AppError,
};

// Type aliases for dependency injection
pub type DynCommentRepository = Arc<dyn CommentRepositoryTrait + Send + Sync>;
pub type DynCommentService = Arc<dyn CommentServiceTrait + Send + Sync>;

#[async_trait]
pub trait CommentRepositoryTrait {
    async fn find_all(&self) -> Result<Vec<comments::Model>, DbErr>;
    async fn find_by_id(&self, id: i32) -> Result<Option<comments::Model>, DbErr>;
    async fn create(&self, input: &CreateCommentRequest) -> Result<comments::Model, DbErr>;
    async fn update(&self, input: &UpdateCommentRequest) -> Result<comments::Model, DbErr>;
    async fn delete(&self, id: i32) -> Result<(), DbErr>;
}

#[async_trait]
pub trait CommentServiceTrait {
    async fn get_comments(&self) -> Result<Vec<ApiResponse<CommentResponse>>, AppError>;
    async fn get_comment(&self, id: i32) -> Result<Option<ApiResponse<CommentResponse>>, AppError>;
    async fn create_comment(&self, input: &CreateCommentRequest) -> Result<ApiResponse<CommentResponse>, AppError>;
    async fn update_comment(
        &self,
        input: &UpdateCommentRequest
    ) -> Result<Option<ApiResponse<CommentResponse>>, AppError>;
    async fn delete_comment(&self, id: i32) -> Result<ApiResponse<()>, AppError>;
}
