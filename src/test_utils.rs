use crate::{
    adapted_iter::AdaptedFilesIterBox,
    adapters::{
        AdaptInfo, ReadBox,
        custom::{BUILTIN_SPAWNING_ADAPTERS, CustomSpawningFileAdapter},
    },
    config::RgaConfig,
    matching::{FastFileMatcher, FileMatcher},
    recurse::concat_read_streams,
};
use anyhow::Result;
use std::path::{Path, PathBuf};
use tokio::{fs::File, io::AsyncReadExt};

pub use pretty_assertions::{assert_eq, assert_ne};
pub fn test_data_dir() -> PathBuf {
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("exampledir/test/");
    d
}

pub async fn simple_fs_adapt_info(filepath: &Path) -> Result<(AdaptInfo, FileMatcher)> {
    Ok(simple_adapt_info_full(
        filepath,
        Box::pin(File::open(filepath).await?),
        true,
    ))
}
pub fn simple_adapt_info(filepath: &Path, inp: ReadBox) -> (AdaptInfo, FileMatcher) {
    simple_adapt_info_full(filepath, inp, false)
}

pub fn simple_adapt_info_full(
    filepath: &Path,
    inp: ReadBox,
    is_real_file: bool,
) -> (AdaptInfo, FileMatcher) {
    (
        AdaptInfo {
            filepath_hint: filepath.to_owned(),
            is_real_file,
            archive_recursion_depth: 0,
            inp,
            line_prefix: "PREFIX:".to_string(),
            config: RgaConfig::default(),
            postprocess: true,
        },
        FastFileMatcher::FileExtension(
            filepath
                .extension()
                .unwrap_or_default()
                .to_string_lossy()
                .into_owned(),
        )
        .into(),
    )
}

pub async fn adapted_to_vec(adapted: AdaptedFilesIterBox) -> Result<Vec<u8>> {
    let mut res = concat_read_streams(adapted);

    let mut buf = Vec::new();
    res.read_to_end(&mut buf).await?;
    Ok(buf)
}

pub fn poppler_adapter() -> CustomSpawningFileAdapter {
    let adapter = BUILTIN_SPAWNING_ADAPTERS
        .iter()
        .find(|e| e.name == "poppler")
        .expect("no poppler adapter");

    adapter.to_adapter()
}

#[cfg(test)]
pub fn init_logging() {
    let _ = env_logger::builder().is_test(true).try_init();
}
