
use serenity::async_trait;
use songbird::{Event, EventContext, EventHandler as VoiceEventHandler, TrackEvent};

use crate::commands::{Context, Error};

#[poise::command(slash_command, prefix_command)]
pub async fn join(
    ctx: Context<'_>,
    #[description = "Channel to join"] channel: Option<poise::serenity_prelude::Channel>,
) -> Result<(), Error> {
    let manager = songbird::get(ctx.serenity_context())
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();
    ctx.say("Joined channel!").await?;
    if let Ok(handler_lock) = manager.join(ctx.guild_id().unwrap(), channel.unwrap().id()).await {
        let mut handler = handler_lock.lock().await;
        handler.add_global_event(TrackEvent::Error.into(), TrackErrorNotifier);
    };
    
    Ok(())
}

struct TrackErrorNotifier;

#[async_trait]
impl VoiceEventHandler for TrackErrorNotifier {
    async fn act(&self, ctx: &EventContext<'_>) -> Option<Event> {
        if let EventContext::Track(track_list) = ctx {
            for (state, handle) in *track_list {
                println!(
                    "Track {:?} encountered an error: {:?}",
                    handle.uuid(),
                    state.playing
                );
            }
        }

        None
    }
}
