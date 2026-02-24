use std::{
    fs,
    time::{SystemTime, UNIX_EPOCH},
};

use leptos::prelude::*;

use crate::{
    constant::{BLOG_CONTENT_DIR, BLOG_COVER_DIR},
    dto::{blog_category_dto::BlogCategoryDto, blog_dto::BlogDto},
};

/**
 * 发布博客
 */
#[server]
pub async fn insert_blog(blog_dto: BlogDto) -> Result<bool, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use crate::{entity::blog::ActiveModel, state::app_state::AppState};
        use sea_orm::{ActiveModelTrait, ActiveValue::Set};

        let state = expect_context::<AppState>();
        let db = state.db();

        let content = blog_dto.get_content();
        if content.is_none() {
            return Err(ServerFnError::ServerError(
                "You have to privide blog content with markdonw format!".to_string(),
            ));
        }

        let tags = blog_dto
            .get_tags()
            .split(",")
            .map(|tag| tag.trim().to_string())
            .collect::<Vec<String>>();

        let cover_image_base64 = blog_dto.get_cover_image_base64().expect("图片base64错误");

        let cover_image_dir = save_image(&cover_image_base64, BLOG_COVER_DIR).await?;

        let new_blog = ActiveModel {
            blog_title: Set(blog_dto.get_blog_title()),
            introduction: Set(blog_dto.get_introduction()),
            content: Set(content.unwrap()),
            tags: Set(tags),
            cover_image_url: Set(Some(cover_image_dir)),
            category_id: Set(blog_dto.get_category_id()),
            ..Default::default()
        };

        new_blog.insert(db).await?;
        Ok(true)
    }

    #[cfg(not(feature = "ssr"))]
    unreachable!("insert_blog should only run on the server");
}

/**
 * 上传图片，用于博客markdown正文
 */
#[server]
pub async fn upload_markdown_image(base64_data: String) -> Result<String, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use chrono::Local;

        let date_path = Local::now().format("%Y-%m-%d").to_string();

        let to_path = format!("{}{}/", BLOG_CONTENT_DIR, date_path);
        let path = save_image(&base64_data, &to_path).await?;
        Ok(path)
    }

    #[cfg(not(feature = "ssr"))]
    unreachable!("upload_markdown_image should only run on the server");
}

/**
 * 加载所有的博客类型
 */
#[server]
pub async fn load_blog_categories() -> Result<Vec<BlogCategoryDto>, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use sea_orm::EntityTrait;

        use crate::{entity::prelude::BlogCategory, state::app_state::AppState};

        let state = expect_context::<AppState>();
        let db = state.db();

        let category_model = BlogCategory::find().all(db).await.unwrap();

        let category_dtos = category_model
            .into_iter()
            .map(|category| BlogCategoryDto::new(category.id, category.category_name))
            .collect::<Vec<BlogCategoryDto>>();

        Ok(category_dtos)
    }

    #[cfg(not(feature = "ssr"))]
    unreachable!("update_about_page should only run on the server");
}

/**
 * 将图片base64保存到文件，并返回保存的文件路径
 */
async fn save_image(base64_data: &str, to_path: &str) -> Result<String, ServerFnError> {
    use base64::engine::general_purpose::STANDARD;
    use base64::Engine;

    // 分离 header 和 body 数据
    let mut parts = base64_data.split(',');

    let header = match parts.next() {
        Some(header) => header,
        None => {
            return Err(ServerFnError::ServerError("invalid base64 header".into()));
        }
    };

    let data = match parts.next() {
        Some(data) => data,
        None => {
            return Err(ServerFnError::ServerError("invalid base64 body".into()));
        }
    };

    // 获取图片扩展名
    let extension = match header {
        "data:image/png;base64" => "png",
        "data:image/jpeg;base64" => "jpg",
        "data:image/jpg;base64" => "jpg",
        "data:image/webp;base64" => "webp",
        "data:image/gif;base64" => "gif",
        _ => return Err(ServerFnError::ServerError("unsupported image type".into())),
    };

    // decode base64
    let image_bytes = match STANDARD.decode(data) {
        Ok(image_bytes) => image_bytes,
        Err(_) => return Err(ServerFnError::ServerError("decode error".into())),
    };

    // 限制文件大小（例如 5MB）
    if image_bytes.len() > 5 * 1024 * 1024 {
        return Err(ServerFnError::ServerError(
            "image too large, the image size shouldn't be larger than 5MB".into(),
        ));
    }

    // 创建目录
    let upload_dir = format!("./data{}", to_path);
    fs::create_dir_all(&upload_dir)?;

    // 使用 系统时间 生成文件名
    let duration_now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("get system time error！");
    let filename = format!("{}.{}", duration_now.as_millis(), extension);
    let filepath = format!("{}/{}", upload_dir, filename);

    fs::write(&filepath, image_bytes)?;

    Ok(format!("{}{}", to_path, filename))
}
