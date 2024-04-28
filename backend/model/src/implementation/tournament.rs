use utils::TStatsPaths;

use crate::tournament;

impl tournament::Model {
    pub fn fetch_banner(&self, paths: &TStatsPaths) -> Option<Vec<u8>> {
        let Some(ref file) = self.banner else {
            return None;
        };
        std::fs::read(paths.banner(&file)).ok()
    }
}
