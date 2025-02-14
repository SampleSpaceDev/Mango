use serde::Deserialize;

pub trait Ratioable {
    fn kdr(&self) -> f32;
    fn fkdr(&self) -> f32;
    fn bblr(&self) -> f32;
    fn wlr(&self) -> f32;
}

macro_rules! impl_ratios {
    ($($struct:ident),+) => {
        $(
            impl Ratioable for $struct {
                fn kdr(&self) -> f32 {
                    self.kills as f32 / self.deaths as f32
                }
                
                fn fkdr(&self) -> f32 {
                    self.final_kills as f32 / self.final_deaths as f32
                }
                
                fn bblr(&self) -> f32 {
                    self.beds_broken as f32 / self.beds_lost as f32
                }
                
                fn wlr(&self) -> f32 {
                    self.wins as f32 / self.losses as f32
                }
            }
        )+
    };
}
impl_ratios!(HypixelBedwarsOverall, HypixelBedwarsSolo, HypixelBedwarsDoubles, HypixelBedwarsThrees, HypixelBedwarsFours);

#[derive(Clone, Deserialize, Debug, Default)]
pub struct HypixelBedwars {
    #[serde(rename = "Experience")]
    pub experience: f32,
    #[serde(rename = "coins")]
    pub tokens: u32,
    
    #[serde(rename = "games_played_bedwars")]
    pub games_played: u32,
    
    pub slumber: HypixelBedwarsSlumber,
    
    #[serde(flatten)]
    pub overall: HypixelBedwarsOverall,
    #[serde(flatten)]
    pub solo: HypixelBedwarsSolo,
    #[serde(flatten)]
    pub doubles: HypixelBedwarsDoubles,
    #[serde(flatten)]
    pub threes: HypixelBedwarsThrees,
    #[serde(flatten)]
    pub fours: HypixelBedwarsFours,
}

#[derive(Clone, Deserialize, Debug, Default)]
pub struct HypixelBedwarsOverall {
    #[serde(rename = "kills_bedwars", default = "default_number")]
    pub kills: u32,
    #[serde(rename = "deaths_bedwars", default = "default_number")]
    pub deaths: u32,
    #[serde(rename = "final_kills_bedwars", default = "default_number")]
    pub final_kills: u32,
    #[serde(rename = "final_deaths_bedwars", default = "default_number")]
    pub final_deaths: u32,
    #[serde(rename = "wins_bedwars", default = "default_number")]
    pub wins: u32,
    #[serde(rename = "losses_bedwars", default = "default_number")]
    pub losses: u32,
    #[serde(rename = "beds_broken_bedwars", default = "default_number")]
    pub beds_broken: u32,
    #[serde(rename = "beds_lost_bedwars", default = "default_number")]
    pub beds_lost: u32,

    #[serde(rename = "iron_resources_collected_bedwars", default = "default_number")]
    pub iron_collected: u32,
    #[serde(rename = "gold_resources_collected_bedwars", default = "default_number")]
    pub gold_collected: u32,
    #[serde(rename = "diamond_resources_collected_bedwars", default = "default_number")]
    pub diamond_collected: u32,
    #[serde(rename = "emerald_resources_collected_bedwars", default = "default_number")]
    pub emerald_collected: u32,

    #[serde(rename = "games_played_bedwars", default = "default_number")]
    pub games_played: u32,
    #[serde(rename = "winstreak", default = "default_number")]
    pub winstreak: u32,
}

#[derive(Clone, Deserialize, Debug, Default)]
pub struct HypixelBedwarsSolo {
    #[serde(rename = "eight_one_kills_bedwars", default)]
    pub kills: u32,
    #[serde(rename = "eight_one_deaths_bedwars", default)]
    pub deaths: u32,
    #[serde(rename = "eight_one_final_kills_bedwars", default)]
    pub final_kills: u32,
    #[serde(rename = "eight_one_final_deaths_bedwars", default)]
    pub final_deaths: u32,
    #[serde(rename = "eight_one_wins_bedwars", default)]
    pub wins: u32,
    #[serde(rename = "eight_one_losses_bedwars", default)]
    pub losses: u32,
    #[serde(rename = "eight_one_beds_broken_bedwars", default)]
    pub beds_broken: u32,
    #[serde(rename = "eight_one_beds_lost_bedwars", default)]
    pub beds_lost: u32,

    #[serde(rename = "eight_one_iron_resources_collected_bedwars", default)]
    pub iron_collected: u32,
    #[serde(rename = "eight_one_gold_resources_collected_bedwars", default)]
    pub gold_collected: u32,
    #[serde(rename = "eight_one_diamond_resources_collected_bedwars", default)]
    pub diamond_collected: u32,
    #[serde(rename = "eight_one_emerald_resources_collected_bedwars", default)]
    pub emerald_collected: u32,

    #[serde(rename = "eight_one_games_played_bedwars", default)]
    pub games_played: u32,
    #[serde(rename = "eight_one_winstreak", default)]
    pub winstreak: u32,
}

#[derive(Clone, Deserialize, Debug, Default)]
pub struct HypixelBedwarsDoubles {
    #[serde(rename = "eight_two_kills_bedwars", default)]
    pub kills: u32,
    #[serde(rename = "eight_two_deaths_bedwars", default)]
    pub deaths: u32,
    #[serde(rename = "eight_two_final_kills_bedwars", default)]
    pub final_kills: u32,
    #[serde(rename = "eight_two_final_deaths_bedwars", default)]
    pub final_deaths: u32,
    #[serde(rename = "eight_two_wins_bedwars", default)]
    pub wins: u32,
    #[serde(rename = "eight_two_losses_bedwars", default)]
    pub losses: u32,
    #[serde(rename = "eight_two_beds_broken_bedwars", default)]
    pub beds_broken: u32,
    #[serde(rename = "eight_two_beds_lost_bedwars", default)]
    pub beds_lost: u32,

    #[serde(rename = "eight_two_iron_resources_collected_bedwars", default)]
    pub iron_collected: u32,
    #[serde(rename = "eight_two_gold_resources_collected_bedwars", default)]
    pub gold_collected: u32,
    #[serde(rename = "eight_two_diamond_resources_collected_bedwars", default)]
    pub diamond_collected: u32,
    #[serde(rename = "eight_two_emerald_resources_collected_bedwars", default)]
    pub emerald_collected: u32,

    #[serde(rename = "eight_two_games_played_bedwars", default)]
    pub games_played: u32,
    #[serde(rename = "eight_two_winstreak", default)]
    pub winstreak: u32,
}

#[derive(Clone, Deserialize, Debug, Default)]
pub struct HypixelBedwarsThrees {
    #[serde(rename = "four_three_kills_bedwars", default)]
    pub kills: u32,
    #[serde(rename = "four_three_deaths_bedwars", default)]
    pub deaths: u32,
    #[serde(rename = "four_three_final_kills_bedwars", default)]
    pub final_kills: u32,
    #[serde(rename = "four_three_final_deaths_bedwars", default)]
    pub final_deaths: u32,
    #[serde(rename = "four_three_wins_bedwars", default)]
    pub wins: u32,
    #[serde(rename = "four_three_losses_bedwars", default)]
    pub losses: u32,
    #[serde(rename = "four_three_beds_broken_bedwars", default)]
    pub beds_broken: u32,
    #[serde(rename = "four_three_beds_lost_bedwars", default)]
    pub beds_lost: u32,

    #[serde(rename = "four_three_iron_resources_collected_bedwars", default)]
    pub iron_collected: u32,
    #[serde(rename = "four_three_gold_resources_collected_bedwars", default)]
    pub gold_collected: u32,
    #[serde(rename = "four_three_diamond_resources_collected_bedwars", default)]
    pub diamond_collected: u32,
    #[serde(rename = "four_three_emerald_resources_collected_bedwars", default)]
    pub emerald_collected: u32,

    #[serde(rename = "four_three_games_played_bedwars", default)]
    pub games_played: u32,
    #[serde(rename = "four_three_winstreak", default)]
    pub winstreak: u32,
}

#[derive(Clone, Deserialize, Debug, Default)]
pub struct HypixelBedwarsFours {
    #[serde(rename = "four_four_kills_bedwars", default)]
    pub kills: u32,
    #[serde(rename = "four_four_deaths_bedwars", default)]
    pub deaths: u32,
    #[serde(rename = "four_four_final_kills_bedwars", default)]
    pub final_kills: u32,
    #[serde(rename = "four_four_final_deaths_bedwars", default)]
    pub final_deaths: u32,
    #[serde(rename = "four_four_wins_bedwars", default)]
    pub wins: u32,
    #[serde(rename = "four_four_losses_bedwars", default)]
    pub losses: u32,
    #[serde(rename = "four_four_beds_broken_bedwars", default)]
    pub beds_broken: u32,
    #[serde(rename = "four_four_beds_lost_bedwars", default)]
    pub beds_lost: u32,

    #[serde(rename = "four_four_iron_resources_collected_bedwars", default)]
    pub iron_collected: u32,
    #[serde(rename = "four_four_gold_resources_collected_bedwars", default)]
    pub gold_collected: u32,
    #[serde(rename = "four_four_diamond_resources_collected_bedwars", default)]
    pub diamond_collected: u32,
    #[serde(rename = "four_four_emerald_resources_collected_bedwars", default)]
    pub emerald_collected: u32,

    #[serde(rename = "four_four_games_played_bedwars", default)]
    pub games_played: u32,
    #[serde(rename = "four_four_winstreak", default)]
    pub winstreak: u32,
}

#[derive(Clone, Deserialize, Debug, Default)]
pub struct HypixelBedwarsDreams {}

#[derive(Clone, Deserialize, Debug, Default)]
pub struct HypixelBedwarsSlumber {
    #[serde(default = "default_number")]
    pub tickets: u32,
    
    #[serde(rename = "total_tickets_earned", default = "default_number")]
    pub total_tickets: u32,
}

fn default_number() -> u32 {
    0
}