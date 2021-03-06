use super::prelude::*;
use crate::game::GameState;
use serenity::model::misc::Mentionable;

#[command("gameinfo")]
#[only_in(guilds)]
#[description("Shows info(such as players and started status) about a game")]
pub async fn game_info(ctx: &Context, msg: &Message) -> CommandResult {
    let game = ctx.data.read().await;
    let game = game.get::<GameContainer>();

    match game {
        Some(game) => {
            let game = game.write().await;

            let (players_field_name, players_field_value) = {
                if game.state() == GameState::NotStarted {
                    let mut players = String::new();
                    for user in game.joined_users().iter() {
                        players.push_str(&user.mention());
                        players.push('\n');
                    }
                    if !players.is_empty() {
                        (format!("Players ({})", game.joined_users().len()), players)
                    } else {
                        ("Players".into(), "None have joined yet :(".into())
                    }
                } else {
                    let mut players = String::new();

                    let all_alive_have_won = game.all_alive_have_won();

                    for player in game.players().iter() {
                        if player.1.is_alive() {
                            let mention = player.0.mention();
                            players.push_str(&mention);
                            if all_alive_have_won {
                                players.push_str("(Victory!)");
                            }
                            players.push('\n');
                        } else {
                            let mention = format! {"~~{}~~\n", player.0.mention()};
                            players.push_str(&mention);
                        }
                    }
                    (format!("Players ({})", game.players().len()), players)
                }
            };
            let fields = vec![
                ("Host", game.host().mention(), false),
                (&players_field_name, players_field_value, true),
                ("Meeting room", game.meeting_room().mention(), true),
                (
                    "Announcement channel",
                    game.announcement_channel().mention(),
                    true,
                ),
                ("Player role", game.player_role().mention(), true),
            ];

            msg.channel_id
                .send_message(ctx, |m| {
                    m.embed(|e| {
                        e.author(|a| {
                            if game.state() == GameState::NotStarted {
                                if game.can_start() {
                                    a.icon_url(
                                        "https://cdn.discordapp.com/emojis/764529845756493885.png",
                                    )
                                    .name("Not started")
                                } else {
                                    a.icon_url(
                                        "https://cdn.discordapp.com/emojis/764529845756493885.png",
                                    )
                                    .name("Not started (waiting for players)")
                                }
                            } else {
                                a.icon_url(
                                    "https://cdn.discordapp.com/emojis/764529758998102037.png",
                                )
                                .name(&game.state().to_string())
                            }
                        })
                        .title("Kingdom Royale")
                        .fields(fields)
                        .colour((|| {
                            if game.state() == GameState::NotStarted {
                                if game.can_start() {
                                    0xdea712 // Yellow
                                } else {
                                    0xbf2419 // Red
                                }
                            } else {
                                0x0dd910 // Green
                            }
                        })())
                        .footer(|f| {
                            if let Some(ava) = msg.author.avatar_url() {
                                f.icon_url(ava);
                            }
                            f.text(if game.state() == GameState::NotStarted {
                                msg.author.name.clone()
                            } else {
                                format!("{} | {}", msg.author.name, game.day())
                            })
                        });

                        e
                    })
                })
                .await
                .map(|_| ())?
        }
        None => msg
            .reply_err(
                ctx,
                "you can't get info about a game if there's none running!".into(),
            )
            .await
            .map(|_| ())?,
    }
    Ok(())
}
