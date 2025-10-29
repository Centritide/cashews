use std::time::Duration;

use chron_db::models::EntityKind;
use serde::Deserialize;

use crate::workers::{IntervalWorker, WorkerContext};

pub struct PollCutscenes;

#[derive(Deserialize)]
struct CutsceneList {
    cutscenes: Vec<Cutscene>,
}

#[derive(Deserialize)]
struct Cutscene {
    // TODO: save thumbnail/art as well
    slug: String,
}

impl IntervalWorker for PollCutscenes {
    fn interval() -> tokio::time::Interval {
        tokio::time::interval(Duration::from_secs(60 * 10))
    }

    async fn tick(&mut self, ctx: &mut WorkerContext) -> anyhow::Result<()> {
        let resp = ctx
            .fetch_and_save(
                "https://mmolb.com/api/cutscenes",
                EntityKind::CutsceneList,
                "cutscenes",
            )
            .await?;

        let cutscenes = resp.parse::<CutsceneList>()?;
        for c in cutscenes.cutscenes {
            ctx.fetch_and_save(
                format!("https://mmolb.com/api/cutscenes/{}", c.slug),
                EntityKind::Cutscene,
                c.slug,
            )
            .await?;
        }

        Ok(())
    }
}
