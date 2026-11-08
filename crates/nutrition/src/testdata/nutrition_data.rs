use std::collections::HashMap;
use std::io::{Cursor, Write};

use async_trait::async_trait;
use zip::write::FileOptions;
use zip::{ZipArchive, ZipWriter};

use nutrition::{Result, fdc::parser::FdcFetcher};

pub const FDC_FF_DATASET_1: &str = "ff dataset 1";
pub const FDC_FF_DATASET_2: &str = "ff dataset 2";
pub const FDC_FF_DATASET_3: &str = "ff dataset 3";

const DATASET1: &str = include_str!("../../fixtures/fdc_dataset_1.json");
const DATASET2: &str = include_str!("../../fixtures/fdc_dataset_2.json");
const DATASET3: &str = include_str!("../../fixtures/fdc_dataset_3.json");

pub struct FdcClientForTests<'a> {
    datasets: HashMap<&'a str, &'a str>,
    selected_dataset: String,
}

impl FdcClientForTests<'_> {
    pub fn new(selected_dataset: impl Into<String>) -> Self {
        Self {
            selected_dataset: selected_dataset.into(),
            datasets: HashMap::from([
                (FDC_FF_DATASET_1, DATASET1),
                (FDC_FF_DATASET_2, DATASET2),
                (FDC_FF_DATASET_3, DATASET3),
            ]),
        }
    }
}

#[async_trait]
impl FdcFetcher for FdcClientForTests<'_> {
    async fn fetch_foundation_foods(&self) -> Result<ZipArchive<Cursor<Vec<u8>>>> {
        let dataset = self
            .datasets
            .get(&self.selected_dataset.as_ref())
            .copied()
            .unwrap_or_else(|| panic!("dataset {}", self.selected_dataset));

        let mut zip_buffer = Cursor::new(Vec::new());
        {
            let mut zip = ZipWriter::new(&mut zip_buffer);
            zip.start_file(
                "FoodData_Central_foundation_food_csv_2024-10.csv",
                FileOptions::<()>::default(),
            )
            .unwrap();
            zip.write_all(dataset.as_bytes()).unwrap();
        }

        let bytes = zip_buffer.into_inner();
        Ok(ZipArchive::new(Cursor::new(bytes)).unwrap())
    }
}
