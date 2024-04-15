use crate::commands::{Context, Error};


#[poise::command(slash_command, prefix_command)]
pub async fn leave(
    ctx: Context<'_>,
) -> Result<(), Error> {
    ctx.say("Hello!").await?;
    Ok(())
}
