use crate::{Context,Error};

/// Response by an AI
#[poise::command(slash_command)]
pub async fn text(
    ctx: Context<'_>,
    #[description = "Prompt"] prompt: String,
) -> Result<(), Error> {
    let reply = {
        poise::CreateReply::default()
            .content("Generating...")
    };
    let handle_generating_message = ctx.send(reply).await?;

    let generation_ai = &ctx.data().generation_ai;
    let response = generation_ai.generate(prompt).await;
    println!("Complete response: {}", response);
    let lenght_response = response.len();
    let character_limit = 2000;
    let mut complete_response: Vec<&str> = vec![];
    if lenght_response > character_limit {
        let mut response_to_divide = response.as_str();
        let nb_divide = lenght_response / character_limit;
        for _i in 0..nb_divide {
            let split_response = response_to_divide.split_at(character_limit);
            complete_response.push(split_response.0);
            response_to_divide = split_response.1;
        }
        complete_response.push(response_to_divide);
    } else {
        complete_response.push(response.as_str());
    }

    println!("Separate response: {:?}", complete_response);

    for answer in 0..complete_response.len() {
        if answer == 0 {
            handle_generating_message.edit(
                ctx,
                poise::CreateReply::default()
                    .content(complete_response[answer])
            ).await?;
        } else {
            let message = {
                poise::CreateReply::default()
                    .content(complete_response[answer])
            };
            ctx.send(message).await?;
        }
    }

    Ok(())
}

#[poise::command(slash_command)]
pub async fn clear_conversation(
    ctx: Context<'_>,
) -> Result<(), Error> {
    let generation_ai = &ctx.data().generation_ai;
    generation_ai.clear_conversation().await;
    ctx.say("Clear all previous messages").await?;
    Ok(())
}
