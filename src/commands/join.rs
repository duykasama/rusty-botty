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
    manager.join(ctx.guild_id().unwrap(), channel.unwrap().id()).await?;
    ctx.say("Joined channel!").await?;
    
    // if let Ok(handler_lock) = manager.join(ctx.guild_id().unwrap(), channel.unwrap().id()).await {
    //     let _call_lock_for_evt = Arc::downgrade(&handler_lock);
    //     let _handler = handler_lock.lock().await;
    //     ctx.say("Joined channel!").await?;
    //
    // } else {
    //     ctx.say("An error occurred").await?;
    // }

    Ok(())
}
