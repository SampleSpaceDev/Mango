use std::default::Default;
use std::time::Instant;
use crate::types::ui::canvas::{Canvas};
use crate::services::hypixel::HypixelService;
use crate::services::minecraft::PlayerDBService;
use crate::{Context, Error};
use poise::CreateReply;
use serenity::all::CreateAttachment;
use skia_safe::{Color, Image};
use tracing::error;
use images::IRON_INGOT;
use crate::services::render::PlayerRenderService;
use crate::types::hypixel::bedwars::Ratioable;
use crate::types::ui::content_box::{ContentBox, ImageContent, Alignment, TextContent, VerticalAlignment, TableContent};
use crate::types::ui::images;
use crate::types::ui::images::{DIAMOND, EMERALD, GOLD_INGOT};
use crate::types::ui::prestige::{format_level, get_level, get_prestige_progress};
use crate::types::utils::{num, stat};
use crate::types::utils::StatType::{Negative, Positive, Ratio};

#[poise::command(slash_command, prefix_command)]
pub async fn run(
    ctx: Context<'_>,
    #[max_length = 16]
    username: String
) -> Result<(), Error> {
    let start = Instant::now();

    let minecraft = PlayerDBService::instance();
    let player = minecraft.get_player_data(&username.clone()).await?;
    
    let hypixel = HypixelService::instance();
    let data = match hypixel.get_player_data(&player.id).await {
        Ok(data) => data,
        Err(e) => {
            ctx.say(format!("Failed to fetch Hypixel data: {}", e)).await?;
            error!("{}", e);
            return Ok(());
        }
    };
    
    let bedwars = &data.stats.bedwars;
    
    let image_data = {
        // Do async things before canvas creation
        let render = PlayerRenderService::instance();
        let player_render = render.get_player_render(&player.id).await.unwrap_or_else(|_| Vec::new());
        
        let mut canvas = Canvas::new(1000, 1000);
        canvas.draw_image_from_path("assets/bedwars/aquarium.png", 0.0, 0.0, 1000.0, 1000.0);

        let rank_color = data.primary_rank_color();
        
        let title_box = {
            ContentBox::new(10.0, 10.0, 980.0, 80.0)
                .with_background(rank_color.with_a(75))
                .with_border(rank_color.with_a(220))
                .with_padding(20.0)
                .add_text(
                    TextContent::new(
                        format!("{} {}<white>'s</white> <red>Bed</red><white>Wars Stats</white>", &data.rank_formatted(), &data.displayname),
                        0.0,
                        0.0,
                        48.0)
                        .with_shadow(true)
                        .with_alignment(Alignment::Center)
                        .with_vertical_alignment(VerticalAlignment::Middle)
                )
        };

        let default_background_color = Color::from_argb(100, 0, 0, 0);
        let default_border_color = Color::from_argb(180, 0, 0, 0);

        let player_render_box = {
            ContentBox::new(10.0, 100.0, 280.0, 490.0)
                .with_background(default_background_color)
                .with_border(default_border_color)
                .with_padding(20.0)
                .add_text(TextContent::new(format_level(data.achievements.bedwars_level), 0.0, 0.0, 36.0)
                    .with_shadow(true)
                    .with_alignment(Alignment::Center)
                )
                .add_image(ImageContent::new(player_render, 0.0, 25.0, 237.0, 384.0)
                    .with_alignment(Alignment::Center)
                    .with_vertical_alignment(VerticalAlignment::Middle)
                )
        };
        
        let prestige_box = {
            ContentBox::new(300.0, 100.0, 690.0, 80.0)
                .with_background(default_background_color)
                .with_border(default_border_color)
                .add_text(
                    TextContent::new(get_prestige_progress(bedwars.experience as u32), 0.0, 0.0, 40.0)
                        .with_alignment(Alignment::Center)
                        .with_vertical_alignment(VerticalAlignment::Middle)
                )
        };
        
        let projected_stats_box = {
            let mut _box = ContentBox::new(300.0, 190.0, 690.0, 230.0)
                .with_background(default_background_color)
                .with_border(default_border_color)
                .with_padding(20.0);
            
            let level = get_level(bedwars.experience as u32);
            let next_prestige = (level.floor() / 100.0).ceil() * 100.0;
            let next_prestige_formatted = format_level(next_prestige as u32);
            let stars_to_go = (next_prestige - level) as u32;
            
            let projected_kills = bedwars.overall.kills + (((bedwars.overall.kills as f32 / level).round() as u32) * stars_to_go);
            let projected_final_kills = bedwars.overall.final_kills + (((bedwars.overall.final_kills as f32 / level).round() as u32) * stars_to_go);
            let projected_fkdr: f32 = projected_final_kills as f32 / bedwars.overall.final_deaths as f32;
            let projected_beds = bedwars.overall.beds_broken + (((bedwars.overall.beds_broken as f32 / level).round() as u32) * stars_to_go);
            let projected_wins = bedwars.overall.wins + (((bedwars.overall.wins as f32 / level).round() as u32) * stars_to_go);
            
            _box
                .add_text(TextContent::new(
                    format!("<white>Kills at {}: <yellow>{}</yellow> (<green>+{}</green>)",
                            next_prestige_formatted,
                            num(projected_kills),
                            num(projected_kills - bedwars.overall.kills)),
                    0.0, 0.0, 32.0)
                    .with_alignment(Alignment::Left)
                )
                .add_text(TextContent::new(
                    format!("<white>Finals at {}: <yellow>{}</yellow> (<green>+{}</green>)",
                            next_prestige_formatted,
                            num(projected_final_kills),
                            num(projected_final_kills - bedwars.overall.final_kills)),
                    0.0, 34.0, 32.0)
                    .with_alignment(Alignment::Left)
                )
                .add_text(TextContent::new(
                    format!("<white>Beds at {}: <yellow>{}</yellow> (<green>+{}</green>)",
                            next_prestige_formatted,
                            num(projected_beds),
                            num(projected_beds - bedwars.overall.beds_broken)),
                    0.0, 70.0, 32.0)
                    .with_alignment(Alignment::Left)
                )
                .add_text(TextContent::new(
                    format!("<white>Wins at {}: <yellow>{}</yellow> (<green>+{}</green>)",
                            next_prestige_formatted,
                            num(projected_wins),
                            num(projected_wins - bedwars.overall.wins)),
                    0.0, 104.0, 32.0)
                    .with_alignment(Alignment::Left)
                )
                .add_text(TextContent::new(
                    format!("<white>FKDR at {}: <yellow>{}</yellow> (<green>+{}</green>)",
                            next_prestige_formatted,
                            num(projected_fkdr),
                            num(projected_fkdr - bedwars.overall.fkdr())),
                    0.0, 136.0, 32.0)
                    .with_alignment(Alignment::Left)
                )
                .add_text(TextContent::new(
                    "<gray>Note: this assumes no negative stats are taken.</gray>".to_string(),
                        0.0, 5.0, 20.0)
                    .with_alignment(Alignment::Center)
                    .with_vertical_alignment(VerticalAlignment::Bottom)
                )
        };

        let resources_box = {
            ContentBox::new(10.0, 600.0, 280.0, 150.0)
                .with_background(default_background_color)
                .with_border(default_border_color)
                .with_padding(10.0)
                // Iron
                .add_image(ImageContent::new(IRON_INGOT.clone(), -2.0, 11.0, 16.0, 16.0).with_scale(2.0))
                .add_text(TextContent::new(format!("<gray>{}</gray>", num(bedwars.overall.iron_collected)), 40.0, -3.0, 32.0))
                // Gold
                .add_image(ImageContent::new(GOLD_INGOT.clone(), -2.0, 46.0, 16.0, 16.0).with_scale(2.0))
                .add_text(TextContent::new(format!("<yellow>{}</yellow>", num(bedwars.overall.gold_collected)), 40.0, 33.0, 32.0))
                // Diamond
                .add_image(ImageContent::new(DIAMOND.clone(), -2.0, 80.0, 16.0, 16.0).with_scale(2.0))
                .add_text(TextContent::new(format!("<aqua>{}</aqua>", num(bedwars.overall.diamond_collected)), 40.0, 69.0, 32.0))
                // Emerald
                .add_image(ImageContent::new(EMERALD.clone(), -2.0, 118.0, 16.0, 16.0).with_scale(2.0))
                .add_text(TextContent::new(format!("<dark_green>{}</dark_green>", num(bedwars.overall.emerald_collected)), 40.0, 105.0, 32.0))
        };

        let misc_box = {
            let _box = ContentBox::new(10.0, 760.0, 280.0, 230.0)
                .with_background(default_background_color)
                .with_border(default_border_color);

            _box
        };

        let stats_box = {
            let kills_label = TextContent::new("<white>Kills</white>".to_string(), 0.0, 30.0, 16.0).with_alignment(Alignment::Center);
            let finals_label = TextContent::new("<white>Finals</white>".to_string(), 0.0, 74.0, 16.0).with_alignment(Alignment::Center);
            let beds_label = TextContent::new("<white>Beds</white>".to_string(), 0.0, 118.0, 16.0).with_alignment(Alignment::Center);
            let wins_label = TextContent::new("<white>Wins</white>".to_string(), 0.0, 162.0, 16.0).with_alignment(Alignment::Center);
            
            let overall_stats_box = {
                ContentBox::new(0.0, 0.0, 216.0, 265.0)
                    .with_border(default_border_color)
                    .with_padding(10.0)
                    
                    .add_text(TextContent::new("<yellow>Overall</yellow>".to_string(), 0.0, 0.0, 24.0)
                        .with_alignment(Alignment::Center)) // Header

                    .add_text(kills_label) // Kills label
                    .add_text(TextContent::new(stats_line((bedwars.overall.kills, bedwars.overall.deaths, bedwars.overall.kdr())), 0.0, 50.0, 24.0)
                        .with_alignment(Alignment::Center)) // Kill amounts

                    .add_text(finals_label) // Final kills label
                    .add_text(TextContent::new(stats_line((bedwars.overall.final_kills, bedwars.overall.final_deaths, bedwars.overall.fkdr())), 0.0, 94.0, 24.0)
                        .with_alignment(Alignment::Center)) // Final kill amounts

                    .add_text(beds_label) // Beds label
                    .add_text(TextContent::new(stats_line((bedwars.overall.beds_broken, bedwars.overall.beds_lost, bedwars.overall.bblr())), 0.0, 139.0, 24.0)
                        .with_alignment(Alignment::Center)) // Bed amounts

                    .add_text(wins_label) // Wins label
                    .add_text(TextContent::new(stats_line((bedwars.overall.wins, bedwars.overall.losses, bedwars.overall.wlr())), 0.0, 183.0, 24.0)
                        .with_alignment(Alignment::Center)) // Win amounts
            };
            let solo_stats_box = {
                ContentBox::new(227.0, 0.0, 216.0, 265.0)
                    .with_border(default_border_color)
                    .with_padding(10.0)
                    .add_text(TextContent::new("<#ffe900>Solo</#ffe900>".to_string(), 0.0, 0.0, 24.0)
                        .with_alignment(Alignment::Center))
            };
            let doubles_stats_box = {
                ContentBox::new(453.0, 0.0, 216.0, 265.0)
                    .with_border(default_border_color)
                    .with_padding(10.0)
                    .add_text(TextContent::new("<#ffd400>Doubles</#ffd400>".to_string(), 0.0, 0.0, 24.0)
                        .with_alignment(Alignment::Center))
            };
            let threes_stats_box = {
                ContentBox::new(124.0, 275.0, 216.0, 265.0)
                    .with_border(default_border_color)
                    .with_padding(10.0)
                    .add_text(TextContent::new("<#ffbf00>3v3v3v3</#ffbf00>".to_string(), 0.0, 0.0, 24.0)
                        .with_alignment(Alignment::Center))
            };
            let fours_stats_box = {
                ContentBox::new(350.0, 275.0, 216.0, 265.0)
                    .with_border(default_border_color)
                    .with_padding(10.0)
                    .add_text(TextContent::new("<gold>4v4v4v4</gold>".to_string(), 0.0, 0.0, 24.0)
                        .with_alignment(Alignment::Center))
            };
            
            ContentBox::new(300.0, 430.0, 690.0, 560.0)
                .with_background(default_background_color)
                .with_border(default_border_color)
                .with_padding(10.0)
                .add_content_boxes(vec![overall_stats_box, solo_stats_box, doubles_stats_box, threes_stats_box, fours_stats_box])
        };

        fn stats_line(values: (u32, u32, f32)) -> String {
            let (pos, neg, ratio) = (num(values.0), num(values.1), num(values.2));
            format!("<green>{pos}</green> • <red>{neg}</red> • <gold>{ratio}</gold>", )
        }
        
        canvas.render_content_boxes(vec![
            title_box,
            player_render_box,
            prestige_box,
            projected_stats_box,
            resources_box,
            misc_box,
            stats_box,
        ]);
        
        canvas.data()
    };
    
    let attachment = CreateAttachment::bytes(image_data.as_bytes(), "image.png");
    
    ctx.send(CreateReply::default().attachment(attachment).content(format!("{:?}", start.elapsed()))).await?;
    
    Ok(())
}

// todo!("Maybe refine layout of stats box. Below is a semi-functional table layout.")
/* let stats_box = {
//     let headers = vec!["",
//         "<aqua>Kills</aqua>", "<#00d3d3>Deaths</#00d3d3>", "<dark_aqua>KDR</dark_aqua>",
//         "Finals", "Deaths", "FKDR",
//         "Beds", "Losses", "BBLR",
//         "Wins", "Losses", "WLR"
//     ].iter().map(|str| str.to_string()).collect();
//     
//     let kills_table = TableContent::new(-300.0, 0.0, 13, 80.0, 22.0)
//         .with_text_size(18.0)
//         .with_border_color(default_border_color.with_a(220))
//         .add_row(headers)
//         .add_row(vec!["<#ffff00>Solo</#ffff00>".to_string(),
//                       stat(bedwars.solo.kills, Positive), stat(bedwars.solo.deaths, Negative), stat(bedwars.solo.kdr(), Ratio),
//                       stat(bedwars.solo.final_kills, Positive), stat(bedwars.solo.final_deaths, Negative), stat(bedwars.solo.fkdr(), Ratio),
//                       stat(bedwars.solo.beds_broken, Positive), stat(bedwars.solo.beds_lost, Negative), stat(bedwars.solo.bblr(), Ratio),
//                       stat(bedwars.solo.wins, Positive), stat(bedwars.solo.losses, Negative), stat(bedwars.solo.wlr(), Ratio),
//         ]);
//         // .add_row(vec!["<#ffe900>Doubles</#ffe900>".to_string(),
//         //               stat(bedwars.doubles.kills, Positive), stat(bedwars.doubles.deaths, Negative), stat(bedwars.doubles.kdr(), Ratio)])
//         // .add_row(vec!["<#ffd400>3v3v3v3</#ffd400>".to_string(),
//         //               stat(bedwars.threes.kills, Positive), stat(bedwars.threes.deaths, Negative), stat(bedwars.threes.kdr(), Ratio)])
//         // .add_row(vec!["<#ffbf00>4v4v4v4</#ffbf00>".to_string(),
//         //               stat(bedwars.fours.kills, Positive), stat(bedwars.fours.deaths, Negative), stat(bedwars.fours.kdr(), Ratio)])
//         // .add_row(vec!["<#ffaa00>Overall</#ffaa00>".to_string(),
//         //               stat(bedwars.overall.kills, Positive), stat(bedwars.overall.deaths, Negative), stat(bedwars.overall.kdr(), Ratio)]);
//     
//     ContentBox::new(300.0, 430.0, 690.0, 560.0)
//         .with_background(default_background_color)
//         .with_border(default_border_color)
//         .with_padding(10.0)
//         .add_table(kills_table)
// }; */