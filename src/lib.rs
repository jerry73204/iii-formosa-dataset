//! Se/Deserialization toolkit for Formosa dataset from Institute for the Information Industry

mod common;
mod types;

pub use crate::types::*;

use crate::common::*;

/// The sample corresponds to an image along with annotations
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Sample {
    pub annotation: Annotation,
    pub annotation_file: PathBuf,
    pub image_file: PathBuf,
    pub roi_dir: PathBuf,
    pub roi_files: Vec<PathBuf>,
}

/// Load dataset directory
pub fn load(dataset_dir: impl AsRef<Path>) -> Result<Vec<Sample>> {
    let dataset_dir = dataset_dir.as_ref();

    let samples: Result<Vec<_>> = glob::glob(
        dataset_dir
            .join("**")
            .join("*.xml")
            .to_str()
            .ok_or_else(|| format_err!("the path '{}' is not UTF-8", dataset_dir.display()))?,
    )?
    .map(|result| {
        let annotation_file = result?;
        let parent = annotation_file.parent().unwrap();
        let name = annotation_file
            .file_stem()
            .unwrap()
            .to_str()
            .ok_or_else(|| format_err!("non-UTF-8 filename '{}'", annotation_file.display()))?;
        let image_file = parent.join(format!("{}.jpg", name));
        let roi_dir = parent.join(format!("{}_roi", name));

        let annotation: Annotation = {
            let content = fs::read_to_string(&annotation_file)
                .with_context(|| format!("cannot open file '{}'", annotation_file.display()))?;
            serde_xml_rs::from_str(&content)
                .with_context(|| format!("failed to parse file '{}'", annotation_file.display()))?
        };

        let roi_files: Result<Vec<_>> = (1..=annotation.object.len())
            .map(|index| {
                let file_name = format!("{}.jpg", index);
                let roi_file = roi_dir.join(&file_name);
                ensure!(
                    roi_file.is_file(),
                    "the ROI file '{}' is missing",
                    roi_file.display()
                );
                Ok(roi_file)
            })
            .collect();
        let roi_files = roi_files?;

        Ok(Sample {
            annotation,
            annotation_file,
            image_file,
            roi_dir,
            roi_files,
        })
    })
    .collect();
    let samples = samples?;

    Ok(samples)
}
