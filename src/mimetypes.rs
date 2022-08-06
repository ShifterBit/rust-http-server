use std::{ffi::OsStr, path::Path};

pub enum HTTPContentType {
    Text(Text),
    Image(Image),
    Audio(Audio),
    Video(Video),
    Application(Application),
}

pub enum Text {
    Css,
    Csv,
    Html,
    Plain,
    Xml,
}

pub enum Image {
    Gif,
    Jpeg,
    Png,
    Tiff,
    VndMicrosoftIcon,
    VndDjvu,
    SvgXml,
    Webp,
}

pub enum Video {
    Mpeg,
    Avi,
    Ogg,
    Mp4,
}
pub enum Audio {
    Mpeg,
    Ogg,
    Webm,
}

pub enum Application {
    JavaArchive,
    EdiX12,
    EDIFACT,
    Javascript,
    Json,
    LdJson,
    OctetStream,
    Ogg,
    Pdf,
    XhtmlXml,
    XShockwaveFlash,
    Xml,
    Zip,
}
fn get_file_extension(filename: &str) -> Option<&str> {
    Path::new(filename).extension().and_then(OsStr::to_str)
}
pub fn infer_mimetype(filename: &str) -> HTTPContentType {
    match get_file_extension(filename) {
        None => HTTPContentType::Application(Application::OctetStream),
        Some(ext) => match ext {
            // Plain Text
            "css" => HTTPContentType::Text(Text::Css),
            "csv" => HTTPContentType::Text(Text::Csv),
            "html" => HTTPContentType::Text(Text::Html),
            "xml" => HTTPContentType::Text(Text::Xml),

            // Images
            "gif" => HTTPContentType::Image(Image::Gif),
            "jpeg" | "jpg" => HTTPContentType::Image(Image::Jpeg),
            "png" => HTTPContentType::Image(Image::Png),
            "tiff" | "tif" => HTTPContentType::Image(Image::Tiff),
            "ico" => HTTPContentType::Image(Image::VndMicrosoftIcon),
            "svg" => HTTPContentType::Image(Image::SvgXml),
            "djvu" | "djv" => HTTPContentType::Image(Image::VndDjvu),

            // Application
            "jar" => HTTPContentType::Application(Application::JavaArchive),
            "edi" => HTTPContentType::Application(Application::EDIFACT),
            "x12" => HTTPContentType::Application(Application::EDIFACT),
            "js" => HTTPContentType::Application(Application::Javascript),
            "json" => HTTPContentType::Application(Application::Json),
            "jsonld" => HTTPContentType::Application(Application::LdJson),
            "bin" => HTTPContentType::Application(Application::OctetStream),
            "ogx" => HTTPContentType::Application(Application::Ogg),
            "xhtml" => HTTPContentType::Application(Application::XhtmlXml),
            "pdf" => HTTPContentType::Application(Application::Pdf),
            "zip" => HTTPContentType::Application(Application::Zip),

            // Audio
            "oga" => HTTPContentType::Audio(Audio::Ogg),
            "weba" => HTTPContentType::Audio(Audio::Webm),
            "mp3" => HTTPContentType::Audio(Audio::Mpeg),

            // Video
            "mpeg" => HTTPContentType::Video(Video::Mpeg),
            "avi" => HTTPContentType::Video(Video::Avi),
            "ogv" => HTTPContentType::Video(Video::Ogg),
            "mp4" => HTTPContentType::Video(Video::Avi),

            _ => HTTPContentType::Text(Text::Plain),
        },
    }
}

impl ToString for HTTPContentType {
    fn to_string(&self) -> String {
        match self {
            Self::Text(t) => t.to_string(),
            Self::Image(t) => t.to_string(),
            Self::Application(t) => t.to_string(),
            Self::Audio(t) => t.to_string(),
            Self::Video(t) => t.to_string(),
        }
    }
}

impl ToString for Text {
    fn to_string(&self) -> String {
        match self {
            Self::Css => "text/css".to_string(),
            Self::Csv => "text/csv".to_string(),
            Self::Html => "text/html".to_string(),
            Self::Plain => "text/plain".to_string(),
            Self::Xml => "text/xml".to_string(),
        }
    }
}

impl ToString for Audio {
    fn to_string(&self) -> String {
        match self {
            Self::Mpeg => "audio/mpeg".to_string(),
            Self::Ogg => "audio/ogg".to_string(),
            Self::Webm => "audio/webm".to_string(),
        }
    }
}

impl ToString for Video {
    fn to_string(&self) -> String {
        match self {
            Self::Mpeg => "video/mpeg".to_string(),
            Self::Avi => "video/x-msvideo".to_string(),
            Self::Ogg => "video/ogg".to_string(),
            Self::Mp4 => "video/mp4".to_string(),
        }
    }
}

impl ToString for Image {
    fn to_string(&self) -> String {
        match self {
            Self::Gif => "image/gif".to_string(),
            Self::Jpeg => "image/jpeg".to_string(),
            Self::Png => "image/png".to_string(),
            Self::Tiff => "image/tiff".to_string(),
            Self::VndMicrosoftIcon => "image/vnd.microsoft.icon".to_string(),
            Self::VndDjvu => "image/vnd.djvu".to_string(),
            Self::SvgXml => "image/svg-xml".to_string(),
            Self::Webp => "image/webp".to_string(),
        }
    }
}

impl ToString for Application {
    fn to_string(&self) -> String {
        match self {
            Self::JavaArchive => "application/java-archive".to_string(),
            Self::EdiX12 => "application/EDIX-12".to_string(),
            Self::EDIFACT => "application/EDIFACT".to_string(),
            Self::Javascript => "application/javascript".to_string(),
            Self::OctetStream => "application/octet-stream".to_string(),
            Self::Ogg => "application/ogg".to_string(),
            Self::Pdf => "application/pdf".to_string(),
            Self::XhtmlXml => "image/xhtml+xml".to_string(),
            Self::Xml => "application/xml".to_string(),
            Self::Json => "application/json".to_string(),
            Self::LdJson => "application/ld+json".to_string(),
            Self::XShockwaveFlash => "application/x-shockwave-flash".to_string(),
            Self::Zip => "application/zip".to_string(),
        }
    }
}
