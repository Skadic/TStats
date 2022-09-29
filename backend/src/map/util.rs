use std::borrow::{BorrowMut, Borrow};

use crate::{error::CacheError, util::get_cached_opt, DBPool};

pub(crate) async fn get_stage_id_cached(
    mut client: impl BorrowMut<redis::aio::Connection>,
    db_pool: impl Borrow<DBPool>,
    tournament_id: u32,
    stage_idx: u32,
) -> Result<Option<u32>, CacheError> {
    // Find the stage id and cache it if needs to be retrieved from the database
    let cache_key = format!("stage:{tournament_id}:{stage_idx}:id");
    get_cached_opt(client.borrow_mut(), &cache_key, || async {
        // find stage id
        sqlx::query!(
            "
            SELECT stage.id FROM stage 
            INNER JOIN tournament ON tournament.id = stage.tournament_id
            WHERE tournament.id=? AND stage.idx=?
            ",
            tournament_id,
            stage_idx
        )
        .fetch_optional(db_pool.borrow())
        .await
        .map(|opt| opt.map(|record| record.id as u32))
    })
    .await
}
