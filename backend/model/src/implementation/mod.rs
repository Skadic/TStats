use crate::sea_orm_active_enums::OsuMode;

mod tournament;

impl From<OsuMode> for i32 {
    fn from(value: OsuMode) -> Self {
        match value {
            OsuMode::Osu => 0,
            OsuMode::Taiko => 1,
            OsuMode::Catch => 2,
            OsuMode::Mania => 3,
        }
    }
}
