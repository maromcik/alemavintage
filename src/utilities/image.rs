use crate::error::{AppError, AppErrorKind};
use actix_multipart::form::tempfile::TempFile;
use image::metadata::Orientation;
use image::ImageDecoder;
use image::{DynamicImage, ImageFormat, ImageReader};
use std::fs::File;
use std::io::{BufWriter, Read};
use uuid::Uuid;

pub struct AppImage {
    pub path: String,
    pub width: i32,
    pub height: i32,
}

impl AppImage {
    pub fn new(path: &str, width: i32, height: i32) -> Self {
        Self {
            path: path.to_owned(),
            width,
            height,
        }
    }
}

pub struct ImageDimensions {
    pub width: u32,
    pub height: u32,
}

impl ImageDimensions {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}

pub struct ImageProcessor {
    pub dynamic_image: DynamicImage,
    pub orientation: Orientation,
    pub format: ImageFormat,
    pub extension: String,
}

impl ImageProcessor {
    pub fn builder(input_image: TempFile) -> ImageProcessorBuilder {
        ImageProcessorBuilder { input_image }
    }

    pub fn resize_img(
        &self,
        target_dimensions: &ImageDimensions,
        base_path: &str,
    ) -> Result<AppImage, AppError> {
        let path = format!(
            "{}_{}.{}",
            base_path,
            &Uuid::new_v4().to_string().as_str()[0..8],
            self.extension.as_str()
        );
        log::info!("resizing image .{path}");
        let mut resized_img = self.dynamic_image.resize(
            target_dimensions.width,
            target_dimensions.height,
            image::imageops::FilterType::CatmullRom,
        );

        resized_img.apply_orientation(self.orientation);
        let fs_path = format!(".{path}");
        let mut output_file = BufWriter::new(File::create(&fs_path)?);
        resized_img.write_to(&mut output_file, self.format)?;
        log::info!("image saved to .{path}");
        Ok(AppImage::new(
            path.as_str(),
            resized_img.width() as i32,
            resized_img.height() as i32,
        ))
    }
}

pub struct ImageProcessorBuilder {
    pub input_image: TempFile,
}

impl ImageProcessorBuilder {
    pub fn validate(&self) -> Result<String, AppError> {
        let filename = self.input_image.file_name.clone().unwrap_or_default();
        let split_res = filename.split('.');
        let vector = split_res.collect::<Vec<&str>>();
        let extension = match vector.last() {
            None => String::new(),
            Some(ext) => (*ext).to_string(),
        };

        let Some(file_mime) = &self.input_image.content_type else {
            return Err(AppError::new(
                AppErrorKind::FileError,
                format!("No MIME type found for {filename}").as_str(),
            ));
        };

        if !file_mime.to_string().starts_with("image/") {
            return Err(AppError::new(
                AppErrorKind::FileError,
                format!("Invalid content type for {filename}").as_str(),
            ));
        }
        Ok(extension)
    }

    pub fn load_image_processor(self) -> Result<ImageProcessor, AppError> {
        let extension = self.validate()?;
        let original_path = self
            .input_image
            .file_name
            .unwrap_or("NO FILE PROVIDED".to_string());
        let mut buffer = Vec::default();
        self.input_image
            .file
            .into_file()
            .read_to_end(&mut buffer)
            .map_err(|err| {
                AppError::new(
                    AppErrorKind::FileError,
                    format!("File '{original_path}' could not be read: {err}").as_str(),
                )
            })?;

        let dynamic_image = image::load_from_memory(&buffer).map_err(|err| {
            AppError::new(
                AppErrorKind::FileError,
                format!("File '{original_path}' could not be loaded: {err}").as_str(),
            )
        })?;

        let cursor = std::io::Cursor::new(&buffer);

        let reader = ImageReader::new(cursor).with_guessed_format()?;

        let format = reader.format().ok_or_else(|| {
            AppError::new(
                AppErrorKind::FileError,
                format!("File format of '{original_path}' could not be determined",).as_str(),
            )
        })?;

        let mut decoder = reader.into_decoder()?;
        let orientation = decoder.orientation()?;

        Ok(ImageProcessor {
            dynamic_image,
            orientation,
            format,
            extension,
        })
    }
}
