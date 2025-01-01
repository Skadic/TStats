use crate::AppState;
use model::db::sea_orm_active_enums::{MatchType, OsuMode};
use model::db::{
    country_restriction, pool_bracket, pool_map, stage, team, team_member, tournament,
};
use poem_openapi::OpenApi;
use sea_orm::{ActiveModelTrait, EntityTrait};
use sqlx::types::chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use utils::*;

pub struct DebugApi(pub AppState);

#[OpenApi]
impl DebugApi {
    /// hi
    #[oai(path = "/owc23", method = "get")]
    #[tracing::instrument(skip(self))]
    async fn owc23(&self) -> poem::Result<()> {
        use sea_orm::ActiveValue as A;

        let db = &self.0.db;

        let owc23 = tournament::ActiveModel {
            id: A::NotSet,
            name: A::Set("Osu World Cup 2023".to_owned()),
            shorthand: A::Set("OWC23".to_owned()),
            format: A::Set(4),
            bws: A::Set(false),
            mode: A::Set(OsuMode::Osu),
            banner: A::Set(None),
            start_date: A::Set(None),
            end_date: A::Set(None),
        }
        .insert(db)
        .await
        .log_internal_server_error("could not insert tournament")?;

        country_restriction::ActiveModel {
            tournament_id: A::Set(owc23.id),
            country_code: A::Set("GER".to_owned()),
        }
        .insert(db)
        .await
        .unwrap();

        let add_team_member = |id, player_id| async move {
            team_member::ActiveModel {
                team_id: A::Set(id),
                user_id: A::Set(player_id),
            }
            .insert(db)
            .await
            .unwrap();
        };

        let team_germany = team::ActiveModel {
            id: A::NotSet,
            tournament_id: A::Set(owc23.id),
            name: A::Set("Germany".to_string()),
        }
        .insert(db)
        .await
        .unwrap();

        add_team_member(team_germany.id, 8116659).await;
        add_team_member(team_germany.id, 4504101).await;
        add_team_member(team_germany.id, 3765989).await;
        add_team_member(team_germany.id, 14385814).await;
        add_team_member(team_germany.id, 13300203).await;
        add_team_member(team_germany.id, 12952320).await;
        add_team_member(team_germany.id, 11921197).await;

        let team_usa = team::ActiveModel {
            id: A::NotSet,
            tournament_id: A::Set(owc23.id),
            name: A::Set("USA".to_string()),
        }
        .insert(db)
        .await
        .unwrap();

        add_team_member(team_usa.id, 7075211).await;
        add_team_member(team_usa.id, 7813296).await;
        add_team_member(team_usa.id, 4108547).await;
        add_team_member(team_usa.id, 2590257).await;
        add_team_member(team_usa.id, 4787150).await;
        add_team_member(team_usa.id, 13380270).await;
        add_team_member(team_usa.id, 3533958).await;
        add_team_member(team_usa.id, 4830687).await;

        let team_spain = team::ActiveModel {
            id: A::NotSet,
            tournament_id: A::Set(owc23.id),
            name: A::Set("Spain".to_string()),
        }
        .insert(db)
        .await
        .unwrap();

        add_team_member(team_spain.id, 6995685).await;
        add_team_member(team_spain.id, 12296128).await;
        add_team_member(team_spain.id, 12975612).await;
        add_team_member(team_spain.id, 9582556).await;
        add_team_member(team_spain.id, 13962152).await;
        add_team_member(team_spain.id, 6735738).await;
        add_team_member(team_spain.id, 12760743).await;
        add_team_member(team_spain.id, 6216284).await;

        macro_rules! pool {
            {$pool:ident, $bracket:ident => $($maps:literal),+} => {
                [$($maps),+].iter().copied().enumerate().map(|(i, map_id)| {
                    pool_map::ActiveModel {
                        tournament_id: A::Set(owc23.id),
                        stage_order: A::Set($pool.stage_order),
                        bracket_order: A::Set($bracket.bracket_order),
                        map_id: A::Set(map_id as i64),
                        map_order: A::Set(i as i16),
                    }
                })
            };
            {$pool:ident, $bracket:ident => $($maps:literal),+; $($other_brackets:ident => $($other_maps:literal),+);+} => {
                pool!($pool, $($other_brackets => $($other_maps),+);+).chain(
                pool!($pool, $bracket => $($maps),+))
            };
        }

        {
            let qualis = stage::ActiveModel {
                tournament_id: A::Set(owc23.id),
                name: A::Set("Q".to_owned()),
                best_of: A::Set(0),
                stage_order: A::Set(0),
                start_date: A::Set(None),
                end_date: A::Set(None),
            }
            .insert(db)
            .await
            .unwrap();

            let nm = pool_bracket::ActiveModel {
                bracket_order: A::Set(0),
                name: A::Set("NM".to_owned()),
                tournament_id: A::Set(owc23.id),
                stage_order: A::Set(qualis.stage_order),
            }
            .insert(db)
            .await
            .unwrap();
            let hd = pool_bracket::ActiveModel {
                bracket_order: A::Set(1),
                name: A::Set("HD".to_owned()),
                tournament_id: A::Set(owc23.id),
                stage_order: A::Set(qualis.stage_order),
            }
            .insert(db)
            .await
            .unwrap();
            let hr = pool_bracket::ActiveModel {
                bracket_order: A::Set(2),
                name: A::Set("HR".to_owned()),
                tournament_id: A::Set(owc23.id),
                stage_order: A::Set(qualis.stage_order),
            }
            .insert(db)
            .await
            .unwrap();
            let dt = pool_bracket::ActiveModel {
                bracket_order: A::Set(3),
                name: A::Set("DT".to_owned()),
                tournament_id: A::Set(owc23.id),
                stage_order: A::Set(qualis.stage_order),
            }
            .insert(db)
            .await
            .unwrap();

            let pool_maps = pool! { qualis,
                nm => 4344435, 4344451, 4344441, 4344442;
                hd => 4344469, 4344423;
                hr => 4344412, 4344450;
                dt => 4344474, 4344475, 4344422
            };
            pool_map::Entity::insert_many(pool_maps)
                .exec(db)
                .await
                .unwrap();
        }
        {
            let ro32 = stage::ActiveModel {
                tournament_id: A::Set(owc23.id),
                name: A::Set("RO32".to_owned()),
                best_of: A::Set(9),
                stage_order: A::Set(1),
                start_date: A::Set(None),
                end_date: A::Set(None),
            }
            .insert(db)
            .await
            .unwrap();

            let nm = pool_bracket::ActiveModel {
                bracket_order: A::Set(0),
                name: A::Set("NM".to_owned()),
                tournament_id: A::Set(owc23.id),
                stage_order: A::Set(ro32.stage_order),
            }
            .insert(db)
            .await
            .unwrap();
            let hd = pool_bracket::ActiveModel {
                bracket_order: A::Set(1),
                name: A::Set("HD".to_owned()),
                tournament_id: A::Set(owc23.id),
                stage_order: A::Set(ro32.stage_order),
            }
            .insert(db)
            .await
            .unwrap();
            let hr = pool_bracket::ActiveModel {
                bracket_order: A::Set(2),
                name: A::Set("HR".to_owned()),
                tournament_id: A::Set(owc23.id),
                stage_order: A::Set(ro32.stage_order),
            }
            .insert(db)
            .await
            .unwrap();
            let dt = pool_bracket::ActiveModel {
                bracket_order: A::Set(3),
                name: A::Set("DT".to_owned()),
                tournament_id: A::Set(owc23.id),
                stage_order: A::Set(ro32.stage_order),
            }
            .insert(db)
            .await
            .unwrap();
            let fm = pool_bracket::ActiveModel {
                bracket_order: A::Set(4),
                name: A::Set("FM".to_owned()),
                tournament_id: A::Set(owc23.id),
                stage_order: A::Set(ro32.stage_order),
            }
            .insert(db)
            .await
            .unwrap();
            let tb = pool_bracket::ActiveModel {
                bracket_order: A::Set(5),
                name: A::Set("TB".to_owned()),
                tournament_id: A::Set(owc23.id),
                stage_order: A::Set(ro32.stage_order),
            }
            .insert(db)
            .await
            .unwrap();

            let pool_maps = pool! { ro32,
                nm => 4352819, 4352824,4351786,3332588;
                hd => 4352411,4352324;
                hr => 1414172,2020374;
                dt => 4352790,3840580,2149694;
                fm => 2583501,4351866,4352856;
                tb => 3121101
            };

            pool_map::Entity::insert_many(pool_maps)
                .exec(db)
                .await
                .unwrap();

            let germany_spain_match = model::db::r#match::ActiveModel {
                id: A::NotSet,
                tournament_id: A::Set(ro32.tournament_id),
                stage_order: A::Set(ro32.stage_order),
                date: A::Set(NaiveDateTime::new(
                    NaiveDate::from_ymd_opt(2023, 10, 29).unwrap(),
                    NaiveTime::from_hms_opt(18, 30, 0).unwrap(),
                )),
                match_type: A::Set(MatchType::VersusMatch),
            }
            .insert(db)
            .await
            .unwrap();

            model::db::versus_match::ActiveModel {
                match_id: A::Set(germany_spain_match.id),
                team_red: A::Set(team_germany.id),
                team_blue: A::Set(team_spain.id),
                score_red: A::Set(Some(5)),
                score_blue: A::Set(Some(0)),
                match_type: A::Set(germany_spain_match.match_type),
            }
            .insert(db)
            .await
            .unwrap();

            model::db::match_link::ActiveModel {
                match_id: A::Set(germany_spain_match.id),
                link_order: A::Set(0),
                osu_mp_id: A::Set(111087337),
            }
            .insert(db)
            .await
            .unwrap();

            model::db::score::ActiveModel {
                player_id: A::Set(8116659),
                tournament_id: A::Set(ro32.tournament_id),
                stage_order: A::Set(ro32.stage_order),
                bracket_order: A::Set(0),
                map_order: A::Set(3),
                match_id: A::Set(germany_spain_match.id),
                score: A::Set(987576),
            }
            .insert(db)
            .await
            .unwrap();

            model::db::score::ActiveModel {
                player_id: A::Set(4504101),
                tournament_id: A::Set(ro32.tournament_id),
                stage_order: A::Set(ro32.stage_order),
                bracket_order: A::Set(0),
                map_order: A::Set(3),
                match_id: A::Set(germany_spain_match.id),
                score: A::Set(982767),
            }
            .insert(db)
            .await
            .unwrap();

            model::db::score::ActiveModel {
                player_id: A::Set(12760743),
                tournament_id: A::Set(ro32.tournament_id),
                stage_order: A::Set(ro32.stage_order),
                bracket_order: A::Set(0),
                map_order: A::Set(3),
                match_id: A::Set(germany_spain_match.id),
                score: A::Set(813145),
            }
            .insert(db)
            .await
            .unwrap();

            model::db::score::ActiveModel {
                player_id: A::Set(13962152),
                tournament_id: A::Set(ro32.tournament_id),
                stage_order: A::Set(ro32.stage_order),
                bracket_order: A::Set(0),
                map_order: A::Set(3),
                match_id: A::Set(germany_spain_match.id),
                score: A::Set(699198),
            }
            .insert(db)
            .await
            .unwrap();
        }
        Ok(())
    }
}
