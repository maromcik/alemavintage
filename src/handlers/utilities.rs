use rexiv2::Metadata;
use std::fs::File;
use std::io::{BufWriter, Read};

use crate::error::{AppError, AppErrorKind};
use actix_multipart::form::tempfile::TempFile;
use actix_web::HttpRequest;

use crate::MIN_PASS_LEN;
use image::metadata::Orientation;
use uuid::Uuid;

pub struct ImageDimensions {
    pub width: u32,
    pub height: u32,
}

impl ImageDimensions {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}

pub fn validate_file(
    file: &TempFile,
    uuid: Uuid,
    mime: &str,
    prefix: &str,
) -> Result<String, AppError> {
    let extension = match file.file_name.clone() {
        None => String::new(),
        Some(name) => {
            let split_res = name.split('.');
            let vector = split_res.collect::<Vec<&str>>();
            match vector.last() {
                None => String::new(),
                Some(ext) => (*ext).to_string(),
            }
        }
    };
    let file_path = format!("/media/{prefix}_{uuid}_{mime}.{extension}");

    let Some(file_mime) = &file.content_type else {
        return Err(AppError::new(
            AppErrorKind::FileError,
            format!("No MIME type found for {file_path}").as_str(),
        ));
    };

    if !file_mime
        .to_string()
        .starts_with(format!("{mime}/").as_str())
    {
        return Err(AppError::new(
            AppErrorKind::FileError,
            format!("Invalid content type for {file_path}").as_str(),
        ));
    }
    Ok(file_path)
}

pub fn save_file(file: TempFile, path: &str, dimensions: &ImageDimensions) -> Result<(), AppError> {
    let original_path = file.file_name.unwrap_or("NO FILE PROVIDED".to_string());
    log::info!("saving file to .{path}");
    let mut buffer = Vec::default();
    file
        .file
        .into_file()
        .read_to_end(&mut buffer)
        .map_err(|err| {
            AppError::new(
                AppErrorKind::FileError,
                format!(
                    "File '{original_path}' could not be read: {}",
                    err.to_string()
                )
                .as_str(),
            )
        })?;
    let metadata = Metadata::new_from_buffer(&buffer)
        .map_err(|err| {
        AppError::new(
            AppErrorKind::FileError,
            format!(
                "Could not extract metadata from file '{original_path}': {}",
                err.to_string()
            )
            .as_str(),
        )
    })?;
    let orientation = metadata.get_orientation();

    let img = image::load_from_memory(&buffer)
        .map_err(|err| {
            AppError::new(
                AppErrorKind::FileError,
                format!(
                    "File '{original_path}' could not be loaded: {}",
                    err.to_string()
                )
                    .as_str(),
            )
        })?;
    let format = image::guess_format(&buffer)
        .map_err(|err| {
            AppError::new(
                AppErrorKind::FileError,
                format!(
                    "File format of '{original_path}' could not be determined: {}",
                    err.to_string()
                )
                    .as_str(),
            )
        })?;
    let mut resized_img = img.resize(
        dimensions.width,
        dimensions.height,
        image::imageops::FilterType::CatmullRom,
    );

    resized_img.apply_orientation(map_orientation(orientation));

    let path = format!(".{path}");
    let mut output_file = BufWriter::new(File::create(&path)?);
    resized_img.write_to(&mut output_file, format)?;

    Ok(())
}

fn map_orientation(orientation: rexiv2::Orientation) -> Orientation {
    match orientation {
        rexiv2::Orientation::Unspecified | rexiv2::Orientation::Normal => Orientation::NoTransforms,
        rexiv2::Orientation::HorizontalFlip => Orientation::FlipHorizontal,
        rexiv2::Orientation::Rotate180 => Orientation::Rotate180,
        rexiv2::Orientation::VerticalFlip => Orientation::FlipVertical,
        rexiv2::Orientation::Rotate90HorizontalFlip => Orientation::Rotate90FlipH,
        rexiv2::Orientation::Rotate90 => Orientation::Rotate90,
        rexiv2::Orientation::Rotate90VerticalFlip => Orientation::Rotate270FlipH,
        rexiv2::Orientation::Rotate270 => Orientation::Rotate270,
    }
}

pub fn remove_file(path: &str) -> Result<(), AppError> {
    let fs_path = format!(".{path}");
    if !path.is_empty() && std::path::Path::new(&fs_path).exists() {
        std::fs::remove_file(&fs_path)?;
    }
    Ok(())
}

#[macro_export]
macro_rules! authorized {
    ($e:expr, $p:expr) => {{
        match $e {
            None => {
                let path = format!("/user/login?ret={}", $p);
                return Ok(HttpResponse::SeeOther()
                    .insert_header((LOCATION, path))
                    .finish());
            }
            Some(v) => v,
        }
    }};
}

pub fn validate_password(password: &str) -> bool {
    let (lower, upper, numeric, special) =
        password
            .chars()
            .fold((false, false, false, false), |(l, u, n, s), c| {
                (
                    {
                        if c.is_lowercase() {
                            true
                        } else {
                            l
                        }
                    },
                    {
                        if c.is_uppercase() {
                            true
                        } else {
                            u
                        }
                    },
                    {
                        if c.is_numeric() {
                            true
                        } else {
                            n
                        }
                    },
                    {
                        if !c.is_alphanumeric() {
                            true
                        } else {
                            s
                        }
                    },
                )
            });
    lower && upper && numeric && special && password.len() >= MIN_PASS_LEN
}

pub fn is_htmx(request: &HttpRequest) -> bool {
    request
        .headers()
        .get("HX-Request")
        .map_or(false, |v| v == "true")
}
