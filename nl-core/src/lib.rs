pub mod java;
pub mod minecraft;
pub mod cache;

mod download;

pub fn test() {
    download::download().unwrap();
}