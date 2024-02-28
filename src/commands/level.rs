use crate::{ApplicationContext, Error};
use poise::command;


#[command(slash_command, category="Levels")]
pub async fn level(ctx: ApplicationContext<'_> ) -> Result<(), Error> {
    let user = ctx.author();
    ctx.reply(format!("you're {user}")).await?;
    Ok(())    
}
