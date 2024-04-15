use crate::commands::{Context, Error};

#[poise::command(slash_command, prefix_command)]
pub async fn join(
    ctx: Context<'_>,
    #[description = "Channel to join"] channel: Option<poise::serenity_prelude::Channel>,
) -> Result<(), Error> {
    ctx.say(format!("Joining {}", channel.unwrap().id())).await?;
    let channels = &ctx.guild_channel().await.unwrap().guild_id;
    // let first_voice_channel = channels.len();
    ctx.say(format!("There are {} channels", channels)).await?;
    
    Ok(())
}
