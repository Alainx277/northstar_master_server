#![allow(dead_code)]

mod de;
mod error;

pub use de::Deserializer;
use de::FixedString;
pub use error::{Error, Result};
use serde::Deserialize;
use serde_with::serde_as;

pub fn from_u8(s: &[u8]) -> Result<Box<PlayerData>> {
    de::from_u8(s)
}

#[allow(non_snake_case)] // The naming is so inconsistent I can't be bothered to put renames on every field
#[serde_as]
#[derive(Deserialize)]
pub struct PlayerData {
    initializedVersion: i32,
    announcementVersionSeen: i32,
    pub xp: i32,
    previousXP: i32,
    credits: i32,
    xp_match: [i32; 20],
    xp_count: [i32; 20],
    pub netWorth: i32,
    matchWin: bool,
    matchScoreEvent: bool,
    matchComplete: bool,
    matchSquadBonus: bool,
    showGameSummary: bool,
    regenShowNew: bool,
    spawnAsTitan: bool,
    haveSeenCustomCoop: bool,
    factionGiftsFixed: bool,
    isACheater: bool,
    spendDoubleColiseumTickets: bool,
    privateMatchState: i32,
    playlistShuffle_seed: i32,
    playlistShuffle_seedFlip: bool,
    playlistShuffle_curIndex: i32,
    #[serde_as(as = "FixedString<16>")]
    lastFDTitanRef: String,
    lastFDDifficulty: i32,
    ultimateEdition: bool,
    randomColiseumUnlocks: i32,
    randomPlayerLevelUnlocks: i32,
    randomTitanLevelUnlocks: [i32; TITAN_COUNT],
    #[serde_as(as = "[_; WEAPONS_ABILITIES_COUNT]")]
    randomWeaponLevelUnlocks: [i32; WEAPONS_ABILITIES_COUNT],
    randomFactionLevelUnlocks: [i32; 7],
    doubleXP: i32,
    coliseumTickets: i32,
    coliseumWinStreak: i32,
    coliseumBestStreak: i32,
    coliseumTotalWins: i32,
    coliseumTotalLosses: i32,
    reventUnlocks: [RecentUnlock; 10],
    hasBeenIntroducedToComms: bool,
    lastCommsUseDate: i32,
    numTimesUsedComms: i32,
    custom_emoji_initialized: bool,
    custom_emoji: [i32; 4],
    burnmeterSlot: i32,
    pve: PveData,
    factionChoice: Faction,
    enemyFaction: Faction,
    persistentRewards: [bool; 32],
    consumableRewards: [i32; 32],
    pilotSpawnLoadout: SpawnLoadout,
    titanSpawnLoadout: SpawnLoadout,
    activePilotLoadout: PilotLoadout,
    activeTitanLoadout: TitanLoadout,
    activeTitanLoadoutIndex: i32,
    pilotLoadouts: [PilotLoadout; 10],
    titanLoadouts: [TitanLoadout; 10],
    pinTrackedEntitlements: [bool; 9],
    newPinTrackedEntitlements: [bool; 9],
    activeBCID: i32,
    pub activeCallingCardIndex: i32,
    pub activeCallsignIconIndex: i32,
    pub activeCallsignIconStyleIndex: i32,
    pub gen: i32,
    factionXP: [i32; 7],
    previousFactionXP: [i32; 7],
    titanXP: [i32; 7],
    previousTitanXP: [i32; 7],
    fdTitanXP: [i32; 7],
    fdPreviousTitanXP: [i32; 7],
    titanFDUnlockPoints: [i32; 7],
    previousFDUnlockPoints: [i32; 7],
    fd_match: [i32; 20],
    fd_count: [i32; 20],
    titanClassLockState: [i32; 7],
    fdTutorialBits: i32,
    fdPlaylistBits: i32,
    gameStats: GameStats,
    mapStats: [MapStats; MAP_COUNT],
    timeStats: HoursPlayed,
    distanceStats: MilesTraveled,
    #[serde_as(as = "[_; WEAPONS_ABILITIES_COUNT]")]
    weaponStats: [WeaponStats; WEAPONS_ABILITIES_COUNT],
    #[serde_as(as = "[_; WEAPONS_ABILITIES_COUNT]")]
    weaponKillStats: [WeaponKillStats; WEAPONS_ABILITIES_COUNT],
    killStats: KillStats,
    deathStats: DeathStats,
    miscStats: MiscStats,
    fdStats: FdStats,
    titanStats: [TitanStats; TITAN_COUNT],
    kdratio_lifetime: f32,
    kdratio_lifetime_pvp: f32,
    kdratio_match: [f32; 10],
    kdratiopvp_match: [f32; 10],
    winStreak: i32,
    highestWinStreakEver: i32,
    winStreakIsDraws: bool,
    winLossHistory: [i32; 10],
    winLossHistorySize: i32,
    mostProjectilesCollectedInVortex: i32,
    blackMarketItemsBought: i32,
    respawnKillInfected: bool,
    #[serde_as(as = "[_; 35]")]
    pilotWeapons: [WeaponMain; 35],
    #[serde_as(as = "[_; 35]")]
    pilotOffhands: [WeaponOffHand; 35],
    titanWeapons: [WeaponMain; 15],
    titanOffhands: [WeaponOffHand; 30],
    titanChassis: [TitanMain; 12],
    hasSeenStore: bool,
    newPilotSkins: [i32; 5],
    unlockedPilotSkins: [i32; 5],
    newPrimePilotSkins: i32,
    unlockedPrimePilotSkins: i32,
    newPilotWeapons: [i32; 2],
    unlockedPilotWeapons: [i32; 2],
    newPilotOffhands: [i32; 2],
    unlockedPilotOffhands: [i32; 2],
    newPilotPassives: i32,
    unlockedPilotPassives: i32,
    newTitanOffhands: [i32; 2],
    unlockedTitanOffhands: [i32; 2],
    newTitanPassives: i32,
    unlockedTitanPassives: i32,
    newTitanChassis: i32,
    unlockedTitanChassis: i32,
    newPrimeTitans: i32,
    unlockedPrimeTitans: i32,
    newPilotSuits: i32,
    unlockedPilotSuits: i32,
    newPilotExecutions: i32,
    unlockedPilotExecutions: i32,
    unlockedFeatures: [i32; 2],
    newFeatures: [i32; 2],
    unlockedBoosts: i32,
    newBoosts: i32,
    unlockedFactions: i32,
    newFactions: i32,
    unlockedCallingCards: [i32; 16],
    newCallingCards: [i32; 16],
    unlockedCallsignIcons: [i32; 7],
    newCallsignIcons: [i32; 7],
    unlockedCommsIcons: [i32; 5],
    newCommsIcons: [i32; 5],
    newTitanExecutions: i32,
    unlockedTitanExecutions: i32,
    #[serde_as(as = "[_; CHALLENGE_COUNT]")]
    challenges: [EChallenge; CHALLENGE_COUNT],
    dailychallenges: [EChallenge; DAILY_CHALLENGE_COUNT],
    activeDailyChallenges: [ActiveDailyChallenge; 9],
    trackedChallenges: [i32; 3],
    EOGTrackedChallenges: [i32; 3],
    #[serde_as(as = "[FixedString<64>; 3]")]
    trackedChallengeRefs: [String; 3],
    #[serde_as(as = "[FixedString<64>; 3]")]
    EOGTrackedChallengeRefs: [String; 3],
    dailyChallengeDayIndex: i32,
    newDailyChallenges: bool,
    isPostGameScoreboardValid: bool,
    postGameData: PostGameData,
    isFDPostGameScoreboardValid: bool,
    postGameDataFD: FdPostGameData,
    previousGooserProgress: i32,
    mapHistory: [i32; 24],
    modeHistory: [i32; 10],
    #[serde_as(as = "FixedString<32>")]
    pub lastPlayList: String,
    lastDailyMatchVictory: i32,
    lastTimePlayed: i32,
    lastTimeLoggedIn: i32,
    abandonCountForMode: [i32; GAME_MODE_COUNT],
    lastAbandonedMode: GameMode,
    lastAbandonTime: i32,
    ranked: Ranked,
}

const GAME_MODE_COUNT: usize = 14;

#[derive(Deserialize)]
enum GameMode {
    Tdm,
    Cp,
    At,
    Ctf,
    Lts,
    Ps,
    Ffa,
    Coliseum,
    Aitdm,
    Speedball,
    Mfd,
    Ttdm,
    Fra,
    Fd,
}

const MAP_COUNT: usize = 25;

#[allow(non_camel_case_types)]
#[derive(Deserialize)]
enum Map {
    mp_box,
    mp_test_engagement_range,

    // R2
    mp_forwardbase_kodai,
    mp_grave,
    mp_homestead,
    mp_thaw,
    mp_black_water_canal,
    mp_eden,
    mp_drydock,
    mp_crashsite3,
    mp_complex3,
    mp_coliseum,

    // R2 DLC
    mp_angel_city,
    mp_colony02,
    mp_relic02,
    mp_glitch,
    mp_lf_stacks,
    mp_lf_meadow,
    mp_lf_deck,
    mp_lf_traffic,
    mp_lf_township,
    mp_lf_uma,
    mp_coliseum_column,
    mp_wargames,
    mp_rise,
}

const WEAPONS_ABILITIES_COUNT: usize = 100;

#[allow(non_camel_case_types)]
#[derive(Deserialize)]
enum LoadoutWeaponsAbilities {
    NULL,
    melee_pilot_emptyhanded,
    melee_pilot_sword,
    melee_titan_sword,
    melee_titan_sword_aoe,
    mp_ability_cloak,
    mp_ability_grapple,
    mp_ability_heal,
    mp_ability_holopilot,
    mp_ability_phase_rewind,
    mp_ability_shifter,
    mp_titanability_ammo_swap,
    mp_titanability_basic_block,
    mp_titanability_gun_shield,
    mp_titanability_hover,
    mp_titanability_laser_trip,
    mp_titanability_particle_wall,
    mp_titanability_phase_dash,
    mp_titanability_power_shot,
    mp_titanability_slow_trap,
    mp_titanability_smoke,
    mp_titanability_sonar_pulse,
    mp_titanability_tether_trap,
    mp_titanability_rearm,
    mp_titancore_flame_wave,
    mp_titancore_flight_core,
    mp_titancore_laser_cannon,
    mp_titancore_salvo_core,
    mp_titancore_shift_core,
    mp_titancore_siege_mode,
    mp_titancore_upgrade,
    mp_titanweapon_40mm,
    mp_titanweapon_arc_wave,
    mp_titanweapon_flame_wall,
    mp_titanweapon_heat_shield,
    mp_titanweapon_homing_rockets,
    mp_titanweapon_dumbfire_rockets,
    mp_titanweapon_laser_lite,
    mp_titanweapon_leadwall,
    mp_titanweapon_meteor,
    mp_titanweapon_particle_accelerator,
    mp_titanweapon_predator_cannon,
    mp_titanweapon_rocket_launcher,
    mp_titanweapon_rocketeer_rocketstream,
    mp_titanweapon_salvo_rockets,
    mp_titanweapon_sniper,
    mp_titanweapon_sticky_40mm,
    mp_titanweapon_stun_laser,
    mp_titanweapon_tracker_rockets,
    mp_titanweapon_vortex_shield,
    mp_titanweapon_vortex_shield_ion,
    mp_titanweapon_xo16,
    mp_titanweapon_xo16_shorty,
    mp_titanweapon_xo16_vanguard,
    mp_weapon_alternator_smg,
    mp_weapon_arc_launcher,
    mp_weapon_autopistol,
    mp_weapon_car,
    mp_weapon_defender,
    mp_weapon_deployable_cover,
    mp_weapon_dmr,
    mp_weapon_doubletake,
    mp_weapon_epg,
    mp_weapon_esaw,
    mp_weapon_frag_drone,
    mp_weapon_frag_grenade,
    mp_weapon_g2,
    mp_weapon_grenade_electric_smoke,
    mp_weapon_grenade_emp,
    mp_weapon_grenade_gravity,
    mp_weapon_grenade_sonar,
    mp_weapon_hemlok,
    mp_weapon_hemlok_smg,
    mp_weapon_lmg,
    mp_weapon_lstar,
    mp_weapon_mastiff,
    mp_weapon_mgl,
    mp_weapon_pulse_lmg,
    mp_weapon_r97,
    mp_weapon_rocket_launcher,
    mp_weapon_rspn101,
    mp_weapon_rspn101_og,
    mp_weapon_satchel,
    mp_weapon_semipistol,
    mp_weapon_shotgun,
    mp_weapon_shotgun_pistol,
    mp_weapon_smart_pistol,
    mp_weapon_smr,
    mp_weapon_sniper,
    mp_weapon_softball,
    mp_weapon_thermite_grenade,
    mp_weapon_vinson,
    mp_weapon_wingman,
    mp_weapon_wingman_n,
    melee_titan_punch_ion,
    melee_titan_punch_legion,
    melee_titan_punch_northstar,
    melee_titan_punch_scorch,
    melee_titan_punch_tone,
    melee_titan_punch_vanguard,
}

#[allow(non_camel_case_types)]
#[derive(Deserialize)]
enum PilotMod {
    NULL,
    aog,
    automatic_fire,
    burn_mod_rspn101,
    burn_mod_g2,
    burn_mod_hemlok,
    burn_mod_vinson,
    burn_mod_lstar,
    burn_mod_car,
    burn_mod_r97,
    burn_mod_alternator_smg,
    burn_mod_lmg,
    burn_mod_esaw,
    burn_mod_pulse_lmg,
    burn_mod_sniper,
    burn_mod_dmr,
    burn_mod_doubletake,
    burn_mod_mastiff,
    burn_mod_shotgun,
    burn_mod_softball,
    burn_mod_shotgun_pistol,
    burn_mod_autopistol,
    burn_mod_wingman,
    burn_mod_semipistol,
    burn_mod_smart_pistol,
    burn_mod_emp_grenade,
    burn_mod_frag_grenade,
    burn_mod_satchel,
    burn_mod_proximity_mine,
    burn_mod_grenade_electric_smoke,
    burn_mod_grenade_gravity,
    burn_mod_thermite_grenade,
    burn_mod_defender,
    burn_mod_rocket_launcher,
    burn_mod_arc_launcher,
    burn_mod_smr,
    burn_mod_mgl,
    burst,
    enhanced_targeting,
    extended_ammo,
    fast_lock,
    fast_reload,
    guided_missile,
    hcog,
    high_density,
    holosight,
    iron_sights,
    long_fuse,
    powered_magnets,
    scope_4x,
    scope_6x,
    scope_8x,
    scope_10x,
    scope_12x,
    silencer,
    sniper_assist,
    stabilizer,
    single_shot,
    slammer,
    stabilized_warhead,
    tank_buster,
    amped_wall,
    short_shift,
    burn_mod_epg,
    ricochet,
    ar_trajectory,
    redline_sight,
    threat_scope,
    smart_lock,
    pro_screen,
    delayed_shot,
    pas_run_and_gun,
    tactical_cdr_on_kill,
    pas_fast_ads,
    pas_fast_swap,
    pas_fast_reload,
    jump_kit,
    quick_charge,
    rocket_arena,
}

const TITAN_COUNT: usize = 7;

#[allow(non_camel_case_types)]
#[derive(Deserialize)]
enum TitanClass {
    ion,
    scorch,
    ronin,
    tone,
    northstar,
    legion,
    vanguard,
}

#[allow(non_camel_case_types)]
#[derive(Deserialize)]
enum TitanMod {
    NULL,
    accelerator,
    afterburners,
    arc_triple_threat,
    burn_mod_titan_40mm,
    burn_mod_titan_arc_cannon,
    burn_mod_titan_sniper,
    burn_mod_titan_triple_threat,
    burn_mod_titan_xo16,
    burn_mod_titan_dumbfire_rockets,
    burn_mod_titan_homing_rockets,
    burn_mod_titan_salvo_rockets,
    burn_mod_titan_shoulder_rockets,
    burn_mod_titan_vortex_shield,
    burn_mod_titan_smoke,
    burn_mod_titan_particle_wall,
    burst,
    capacitor,
    extended_ammo,
    fast_lock,
    fast_reload,
    instant_shot,
    overcharge,
    quick_shot,
    rapid_fire_missiles,
    stryder_sniper,
}

#[allow(non_camel_case_types)]
#[derive(Deserialize)]
enum PilotPassive {
    NULL,
    pas_stealth_movement,
    pas_ordnance_pack,
    pas_power_cell,
    pas_wallhang,
    pas_fast_health_regen,
    pas_minimap_ai,
    pas_longer_bubble,
    pas_run_and_gun,
    pas_dead_mans_trigger,
    pas_wall_runner,
    pas_fast_hack,
    pas_cloaked_wallrun,
    pas_cloaked_wallhang,
    pas_smoke_sight,
    pas_fast_embark,
    pas_cdr_on_kill,
    pas_at_hunter,
    pas_ordnance_beam,
    pas_fast_rodeo,
    pas_phase_eject,
    pas_ads_hover,
    pas_enemy_death_icons,
    pas_off_the_grid,
}

#[allow(non_camel_case_types)]
#[derive(Deserialize)]
enum PilotSuit {
    medium,
    geist,
    stalker,
    light,
    heavy,
    grapple,
    nomad,
}

#[allow(non_camel_case_types)]
#[derive(Deserialize)]
enum PilotRace {
    race_human_male,
    race_human_female,
}

const PILOT_EXECUTION_COUNT: usize = 13;

#[allow(non_camel_case_types)]
#[derive(Deserialize)]
enum PilotExecution {
    execution_neck_snap,
    execution_face_stab,
    execution_backshot,
    execution_combo,
    execution_knockout,
    execution_telefrag,
    execution_stim,
    execution_grapple,
    execution_pulseblade,
    execution_random,
    execution_cloak,
    execution_holopilot,
    execution_ampedwall,
}

#[allow(non_camel_case_types)]
#[derive(Deserialize)]
enum TitanExecution {
    execution_ion,
    execution_ion_prime,
    execution_tone,
    execution_tone_prime,
    execution_ronin,
    execution_ronin_prime,
    execution_northstar,
    execution_northstar_prime,
    execution_legion,
    execution_legion_prime,
    execution_vanguard,
    execution_scorch,
    execution_scorch_prime,
    execution_random_0,
    execution_random_1,
    execution_random_2,
    execution_random_3,
    execution_random_4,
    execution_random_5,
    execution_random_6,
}

#[allow(non_camel_case_types)]
#[derive(Deserialize)]
enum TitanPassive {
    NULL,
    pas_enhanced_titan_ai,
    pas_auto_eject,
    pas_dash_recharge,
    pas_defensive_core,
    pas_shield_regen,
    pas_assault_reactor,
    pas_hyper_core,
    pas_anti_rodeo,
    pas_build_up_nuclear_core,
    pas_offensive_autoload,
    pas_offensive_hitnrun,
    pas_offensive_regen,
    pas_defensive_tacload,
    pas_defensive_quickdash,
    pas_defensive_domeshield,
    pas_mobility_dash_capacity,
    pas_warpfall,
    pas_bubbleshield,
    pas_ronin_weapon,
    pas_northstar_weapon,
    pas_ion_weapon,
    pas_tone_weapon,
    pas_scorch_weapon,
    pas_legion_weapon,
    pas_ion_tripwire,
    pas_ion_vortex,
    pas_ion_lasercannon,
    pas_tone_rockets,
    pas_tone_sonar,
    pas_tone_wall,
    pas_ronin_arcwave,
    pas_ronin_phase,
    pas_ronin_swordcore,
    pas_northstar_cluster,
    pas_northstar_trap,
    pas_northstar_flightcore,
    pas_scorch_firewall,
    pas_scorch_shield,
    pas_scorch_selfdmg,
    pas_legion_spinup,
    pas_legion_gunshield,
    pas_legion_smartcore,
    pas_ion_weapon_ads,
    pas_tone_burst,
    pas_legion_chargeshot,
    pas_ronin_autoshift,
    pas_northstar_optics,
    pas_scorch_flamecore,
    pas_vanguard_coremeter,
    pas_vanguard_shield,
    pas_vanguard_rearm,
    pas_vanguard_doom,
    pas_vanguard_core1,
    pas_vanguard_core2,
    pas_vanguard_core3,
    pas_vanguard_core4,
    pas_vanguard_core5,
    pas_vanguard_core6,
    pas_vanguard_core7,
    pas_vanguard_core8,
    pas_vanguard_core9,
}

#[derive(Deserialize)]
enum TitanIsPrime {
    No,
    Yes,
}

#[allow(non_camel_case_types)]
#[derive(Deserialize)]
enum Faction {
    faction_apex,
    faction_64,
    faction_vinson,
    faction_marauder,
    faction_aces,
    faction_ares,
    faction_marvin,
}

#[allow(non_camel_case_types)]
#[derive(Deserialize)]
enum OwnedEntitlements {
    ET_DLC7_WEAPON_BUNDLE, // BUNDLE MUST BE FIRST!!!!
    ET_DLC7_R201_WARPAINT,
    ET_DLC7_G2A5_WARPAINT,
    ET_DLC7_FLATLINE_WARPAINT,
    ET_DLC7_CAR_WARPAINT,
    ET_DLC7_ALTERNATOR_WARPAINT,
    ET_DLC7_EVA8_WARPAINT,
    ET_DLC7_WINGMAN_WARPAINT,
    ET_DLC7_ARCHER_WARPAINT,
}

#[derive(Deserialize)]
struct SpawnLoadout {
    index: i32,
}

#[allow(non_camel_case_types)]
#[serde_as]
#[derive(Deserialize)]
struct PilotLoadout {
    #[serde_as(as = "FixedString<42>")]
    name: String,
    suit: PilotSuit,
    race: PilotRace,
    execution: PilotExecution,
    primary: LoadoutWeaponsAbilities,
    primaryAttachment: PilotMod,
    primaryMod1: PilotMod,
    primaryMod2: PilotMod,
    primaryMod3: PilotMod,
    secondary: LoadoutWeaponsAbilities,
    secondaryMod1: PilotMod,
    secondaryMod2: PilotMod,
    secondaryMod3: PilotMod,
    weapon3: LoadoutWeaponsAbilities,
    weapon3Mod1: PilotMod,
    weapon3Mod2: PilotMod,
    weapon3Mod3: PilotMod,
    ordnance: LoadoutWeaponsAbilities,
    passive1: PilotPassive,
    passive2: PilotPassive,
    skinIndex: i32,
    camoIndex: i32,
    primarySkinIndex: i32,
    primaryCamoIndex: i32,
    secondarySkinIndex: i32,
    secondaryCamoIndex: i32,
    weapon3SkinIndex: i32,
    weapon3CamoIndex: i32,
}

#[allow(non_camel_case_types)]
#[serde_as]
#[derive(Deserialize)]
struct TitanLoadout {
    #[serde_as(as = "FixedString<42>")]
    name: String,
    titanClass: TitanClass,
    primaryMod: TitanMod,
    special: LoadoutWeaponsAbilities,
    antirodeo: LoadoutWeaponsAbilities,
    passive1: TitanPassive,
    passive2: TitanPassive,
    passive3: TitanPassive,
    passive4: TitanPassive,
    passive5: TitanPassive,
    passive6: TitanPassive,
    titanExecution: TitanExecution,
    skinIndex: i32,
    camoIndex: i32,
    decalIndex: i32,
    primarySkinIndex: i32,
    primaryCamoIndex: i32,
    isPrime: TitanIsPrime, //Really should be bool, but script for loadouts is not easily set up to handle bools unfortunately...
    primeSkinIndex: i32,
    primeCamoIndex: i32,
    primeDecalIndex: i32,
    showArmBadge: i32,
}

#[allow(non_camel_case_types)]
#[derive(Deserialize)]
struct RecentUnlock {
    refGuid: i32,
    parentRefGuid: i32,
    count: i32,
}

#[allow(non_camel_case_types)]
#[derive(Deserialize)]
struct PveData {
    version: i32,
    currency: i32,
    currencyInLatestMatch: i32,
    tacticalUnlocks: [i32; 6],
    feathersForMap: [i32; MAP_COUNT],
}

#[allow(non_camel_case_types)]
#[derive(Deserialize)]
enum UnlockRef {
    edit_pilots, // these two must come first
    edit_titans,
    pilot_custom_loadout_1,
    pilot_custom_loadout_2,
    pilot_custom_loadout_3,
    pilot_custom_loadout_4,
    pilot_custom_loadout_5,
    titan_custom_loadout_1,
    titan_custom_loadout_2,
    titan_custom_loadout_3,
    titan_custom_loadout_4,
    titan_custom_loadout_5,
    burn_card_slot_1,
    burn_card_slot_2,
    burn_card_slot_3,
    burn_card_pack_1,
    burn_card_pack_2,
    burn_card_pack_3,
    burn_card_pack_4,
    burn_card_pack_5,
    challenges,
}

#[allow(non_camel_case_types)]
#[derive(Deserialize)]
enum BurnCard {
    NULL,
    bc_conscription,
    bc_double_xp,
    bc_free_xp,
    bc_fast_cooldown1,
    bc_fast_cooldown2,
    bc_super_stim,
    bc_super_cloak,
    bc_super_sonar,
    bc_summon_ogre,
    bc_cloak_forever,
    bc_stim_forever,
    bc_sonar_forever,
    bc_summon_stryder,
    bc_spectre_virus,
    bc_play_spectre,
    bc_double_agent,
    bc_minimap,
    bc_summon_atlas,
    bc_megaturrets,
    bc_summon_dogfighter,
    bc_wifi_spectre_hack,
    bc_nuclear_core,
    bc_core_charged,
    bc_smart_pistol_m2,
    bc_r97_m2,
    bc_rspn101_m2,
    bc_dmr_m2,
    bc_shotgun_m2,
    bc_lmg_m2,
    bc_g2_m2,
    bc_car_m2,
    bc_hemlok_m2,
    bc_sniper_m2,
    bc_smr_m2,
    bc_mgl_m2,
    bc_defender_m2,
    bc_rocket_launcher_m2,
    bc_semipistol_m2,
    bc_autopistol_m2,
    bc_wingman_m2,
    bc_satchel_m2,
    bc_frag_m2,
    bc_arc_m2,
    bc_prox_m2,
    bc_pilot_warning,
    bc_rematch,
    bc_minimap_scan,
    bc_free_build_time_1,
    bc_free_build_time_2,
    bc_fast_build_1,
    bc_fast_build_2,
    bc_hunt_soldier,
    bc_hunt_spectre,
    bc_hunt_titan,
    bc_hunt_pilot,
    bc_auto_sonar,
    bc_fast_movespeed,
    bc_auto_refill,
    bc_dice_ondeath,
    bc_titan_40mm_m2,
    bc_titan_arc_cannon_m2,
    bc_titan_rocket_launcher_m2,
    bc_titan_sniper_m2,
    bc_titan_triple_threat_m2,
    bc_titan_xo16_m2,
    bc_titan_dumbfire_missile_m2,
    bc_titan_homing_rockets_m2,
    bc_titan_salvo_rockets_m2,
    bc_titan_shoulder_rockets_m2,
    bc_titan_vortex_shield_m2,
    bc_titan_electric_smoke_m2,
    bc_titan_shield_wall_m2,
    bc_titan_melee_m2,
    bc_extra_dash,
    bc_lstar_m2,
    bc_mastiff_m2,
    bc_vinson_m2,
}

#[allow(non_camel_case_types)]
#[derive(Deserialize)]
struct ActiveBurnCardData {
    cardRef: BurnCard,
    lastCardRef: BurnCard,
    clearOnStart: bool,
}

#[allow(non_camel_case_types)]
#[derive(Deserialize)]
struct HistoryBurnCardData {
    collected: i32,
    spent: i32,
}

#[allow(non_camel_case_types)]
#[derive(Deserialize)]
struct BlackMarketBurnCardUpgrade {
    cardRef: BurnCard,
}

#[allow(non_camel_case_types)]
#[derive(Deserialize)]
struct MapStats {
    gamesJoined: [i32; GAME_MODE_COUNT],
    gamesCompleted: [i32; GAME_MODE_COUNT],
    gamesWon: [i32; GAME_MODE_COUNT],
    gamesLost: [i32; GAME_MODE_COUNT],
    topPlayerOnTeam: [i32; GAME_MODE_COUNT],
    top3OnTeam: [i32; GAME_MODE_COUNT],
    hoursPlayed: [f32; GAME_MODE_COUNT],
    timesScored100AttritionPoints_byMap: i32,
    winsByDifficulty: [i32; 5],
    matchesByDifficulty: [i32; 5],
    perfectMatchesByDifficulty: [i32; 5],
}

#[allow(non_camel_case_types)]
#[derive(Deserialize)]
struct GameStats {
    modesPlayed: [i32; GAME_MODE_COUNT],
    previousModesPlayed: [i32; GAME_MODE_COUNT],
    modesWon: [i32; GAME_MODE_COUNT],
    mvp_total: i32,
    gamesCompletedTotal: i32,
    gamesWonTotal: i32,
    gamesWonAsIMC: i32,
    gamesWonAsMilitia: i32,
    gamesCompletedAsIMC: i32,
    gamesCompletedAsMilitia: i32,
    pvpKills: [i32; GAME_MODE_COUNT],
    timesKillDeathRatio2to1: [i32; GAME_MODE_COUNT],
    timesKillDeathRatio2to1_pvp: [i32; GAME_MODE_COUNT],
    timesScored100AttritionPoints_total: i32,
}

#[allow(non_camel_case_types)]
#[derive(Deserialize)]
struct HoursPlayed {
    total: f32,
    asTitan: [f32; TITAN_COUNT],
    asPilot: f32,
    asTitanTotal: f32,
    dead: f32,
    wallhanging: f32,
    wallrunning: f32,
    inAir: f32,
}

#[allow(non_camel_case_types)]
#[derive(Deserialize)]
struct MilesTraveled {
    total: f32,
    asTitan: [f32; TITAN_COUNT],
    asPilot: f32,
    asTitanTotal: f32,
    wallrunning: f32,
    inAir: f32,
    ziplining: f32,
    onFriendlyTitan: f32,
    onEnemyTitan: f32,
}

#[allow(non_camel_case_types)]
#[derive(Deserialize)]
struct WeaponStats {
    hoursUsed: f32,
    hoursEquipped: f32,
    shotsFired: i32,
    shotsHit: i32,
    headshots: i32,
    critHits: i32,
    titanDamage: i32,
}

#[allow(non_camel_case_types)]
#[derive(Deserialize)]
struct WeaponKillStats {
    total: i32,
    pilots: i32,
    ejecting_pilots: i32,
    spectres: i32,
    marvins: i32,
    grunts: i32,
    ai: i32,
    titansTotal: i32,
    titans: [i32; TITAN_COUNT],
    npcTitans: [i32; TITAN_COUNT],
    assistsTotal: i32,
    killingSprees: i32,
}

#[allow(non_camel_case_types)]
#[derive(Deserialize)]
struct KillStats {
    total: i32,
    totalWhileUsingBurnCard: i32,
    titansWhileTitanBCActive: i32,
    totalPVP: i32,
    pilots: i32,
    spectres: i32,
    marvins: i32,
    grunts: i32,
    totalTitans: i32,
    totalTitansWhileDoomed: i32,
    totalPilots: i32,
    totalNPC: i32,
    asPilot: i32,
    asTitan: [i32; TITAN_COUNT],
    firstStrikes: i32,
    ejectingPilots: i32,
    whileEjecting: i32,
    cloakedPilots: i32,
    whileCloaked: i32,
    wallrunningPilots: i32,
    whileWallrunning: i32,
    wallhangingPilots: i32,
    whileWallhanging: i32,
    pilotExecution: i32,
    pilotExecutePilot: i32,
    pilotExecutePilotByType: [i32; PILOT_EXECUTION_COUNT],
    pilotKickMelee: i32,
    pilotKickMeleePilot: i32,
    titanMelee: i32,
    titanMeleePilot: i32,
    titanStepCrush: i32,
    titanStepCrushPilot: i32,
    titanExocutionIon: i32,
    titanExocutionScorch: i32,
    titanExocutionNorthstar: i32,
    titanExocutionRonin: i32,
    titanExocutionTone: i32,
    titanExocutionLegion: i32,
    titanExocutionVanguard: i32,
    titanFallKill: i32,
    petTitanKillsFollowMode: i32,
    petTitanKillsGuardMode: i32,
    rodeo_total: i32,
    rodeo_stryder: i32,
    rodeo_buddy: i32,
    rodeo_atlas: i32,
    rodeo_ogre: i32,
    pilot_headshots_total: i32,
    evacShips: i32,
    flyers: i32,
    nuclearCore: i32,
    evacuatingEnemies: i32,
    exportTrapKills: i32,
    coopChallenge_NukeTitan_Kills: i32,
    coopChallenge_MortarTitan_Kills: i32,
    coopChallenge_EmpTitan_Kills: i32,
    coopChallenge_BubbleShieldGrunt_Kills: i32,
    coopChallenge_CloakDrone_Kills: i32,
    coopChallenge_Dropship_Kills: i32,
    coopChallenge_SuicideSpectre_Kills: i32,
    coopChallenge_Turret_Kills: i32,
    coopChallenge_Sniper_Kills: i32,
    ampedVortexKills: i32,
    meleeWhileCloaked: i32,
    pilotKillsWhileUsingActiveRadarPulse: i32,
    titanKillsAsPilot: i32,
    pilotKillsWhileStimActive: i32,
    pilotKillsAsTitan: i32,
    totalAssists: i32,
    killingSpreeds: [i32; TITAN_COUNT],
    pilotKillsAsPilot: i32,
    titanKillsAsTitan: i32,
    telefragKils: i32,
    grappleKills: i32,
    throughAWallKills: i32,
    distractedKills: i32,
    pilotExecutePilotWhileCloaked: i32,
    pilotKillsWithHoloPilotActive: i32,
    pilotKillsWithAmpedWallActive: i32,
}

#[allow(non_camel_case_types)]
#[derive(Deserialize)]
struct DeathStats {
    total: i32,
    totalPVP: i32,
    asPilot: i32,
    asTitan: [i32; TITAN_COUNT],
    byPilots: i32,
    bySpectres: i32,
    byGrunts: i32,
    byTitans: [i32; TITAN_COUNT],
    byNPCTitans: [i32; TITAN_COUNT],
    suicides: i32,
    whileEjecting: i32,
}

#[allow(non_camel_case_types)]
#[derive(Deserialize)]
struct MiscStats {
    titanFalls: i32,
    titanFallsFirst: i32,
    titanEmbarks: i32,
    rodeos: i32,
    rodeosFromEject: i32,
    timesEjected: i32,
    timesEjectedNuclear: i32,
    burnCardsEarned: i32,
    burnCardsSpent: i32,
    boostsActivated: i32,
    spectreLeeches: i32,
    spectreLeechesByMap: [i32; MAP_COUNT],
    evacsAttempted: i32,
    evacsSurvived: i32,
    flagsCaptured: i32,
    flagsReturned: i32,
    arcCannonMultiKills: i32,
    gruntsConscripted: i32,
    hardpointsCaptured: i32,
    challengeTiersCompleted: i32,
    challengesCompleted: i32,
    dailyChallengesCompleted: i32,
    timesLastTitanRemaining: i32,
    killingSprees: i32,
    coopChallengesCompleted: i32,
    forgedCertificationsUsed: i32,
    regenForgedCertificationsUsed: i32,
}

#[allow(non_camel_case_types)]
#[derive(Deserialize)]
struct FdStats {
    arcMinesPlaced: i32,
    turretsPlaced: i32,
    rodeos: i32,
    rodeoNukes: i32,
    arcMineZaps: i32,
    turretKills: i32,
    harvesterBoosts: i32,
    wavesComplete: i32,
    easyWins: i32,
    normalWins: i32,
    hardWins: i32,
    masterWins: i32,
    insaneWins: i32,
    highestTitanFDLevel: i32,
}

#[allow(non_camel_case_types)]
#[derive(Deserialize)]
struct TitanStats {
    pilots: i32,
    titansTotal: i32,
    ejections: i32,
    titansWhileDoomed: i32,
    titanDamage: i32,
    titansAsPrime: i32,
    pilotsAsPrime: i32,
    executionsAsPrime: i32,
    coresEarned: i32,
    matchesByDifficulty: [i32; 5],
    perfectMatchesByDifficulty: [i32; 5],
}

#[allow(non_camel_case_types)]
#[derive(Deserialize)]
struct WeaponMain {
    weaponStats: WeaponStats,
    weaponKillStats: WeaponKillStats,
    weaponXP: i32,
    previousWeaponXP: i32,
    proScreenKills: i32,
    previousProScreenKills: i32,
    newMods: i32,
    unlockedMods: i32,
    newWeaponSkins: [i32; 5],
    unlockedWeaponSkins: [i32; 5],
    newPrimeWeaponSkins: [i32; 6],
    unlockedPrimeWeaponSkins: [i32; 6],
    newFeatures: i32,
    unlockedFeatures: i32,
}

#[allow(non_camel_case_types)]
#[derive(Deserialize)]
struct WeaponOffHand {
    weaponStats: WeaponStats,
    weaponKillStats: WeaponKillStats,
}

#[allow(non_camel_case_types)]
#[derive(Deserialize)]
struct TitanMain {
    newPassives: [i32; 2],
    unlockedPassives: [i32; 2],
    newSkins: [i32; 5],
    unlockedSkins: [i32; 5],
    newPrimeSkins: [i32; 2],
    unlockedPrimeSkins: [i32; 2],
    newWeaponSkins: [i32; 5],
    unlockedWeaponSkins: [i32; 5],
    newPrimeWeaponSkins: i32,
    unlockedPrimeWeaponSkins: i32,
    newTitanDecals: [i32; 3],
    unlockedTitanDecals: [i32; 3],
    newPrimeTitanDecals: i32,
    unlockedPrimeTitanDecals: i32,
    unlockedFDUpgrades: [i32; 2],
    newFDUpgrades: [i32; 2],
}

const CHALLENGE_COUNT: usize = 177;

#[allow(non_camel_case_types)]
#[derive(Deserialize)]
enum Challenge {
    NULL,
    // General
    ch_games_played,
    ch_games_won,
    ch_games_mvp,
    ch_titan_falls,
    ch_rodeos,
    ch_times_ejected,
    ch_spectres_leeched,

    // Time
    ch_hours_played,
    ch_hours_played_pilot,
    ch_hours_played_titan,
    ch_hours_wallhang,

    // Distance
    ch_dist_total,
    ch_dist_pilot,
    ch_dist_titan,
    ch_dist_wallrun,
    ch_dist_inair,
    ch_dist_zipline,
    ch_dist_on_friendly_titan,
    ch_dist_on_enemy_titan,

    // Kills
    ch_grunt_kills,
    ch_spectre_kills,
    ch_marvin_kills,
    ch_first_strikes,
    ch_ejecting_pilot_kills,
    ch_kills_while_ejecting,
    ch_cloaked_pilot_kills,
    ch_kills_while_cloaked,
    ch_wallrunning_pilot_kills,
    ch_wallhanging_pilot_kills,
    ch_kills_while_wallrunning,
    ch_kills_while_wallhanging,
    ch_pilotExecutePilot,
    ch_pilotKickMelee,
    ch_pilotKickMeleePilot,
    ch_titanMelee,
    ch_titanMeleePilot,
    ch_titanStepCrush,
    ch_titanStepCrushPilot,
    ch_titanExocutionStryder,
    ch_titanExocutionBuddy,
    ch_titanExocutionAtlas,
    ch_titanExocutionOgre,
    ch_titanFallKill,
    ch_petTitanKillsFollowMode,
    ch_petTitanKillsGuardMode,
    ch_rodeo_kills,

    // Titan Primary
    ch_40mm_kills,
    ch_40mm_pilot_kills,
    ch_40mm_titan_kills,
    ch_40mm_spectre_kills,
    ch_40mm_grunt_kills,
    ch_40mm_hours_used,
    ch_40mm_crits,

    ch_xo16_kills,
    ch_xo16_pilot_kills,
    ch_xo16_titan_kills,
    ch_xo16_spectre_kills,
    ch_xo16_grunt_kills,
    ch_xo16_hours_used,
    ch_xo16_headshots,
    ch_xo16_crits,

    ch_titan_sniper_kills,
    ch_titan_sniper_pilot_kills,
    ch_titan_sniper_titan_kills,
    ch_titan_sniper_spectre_kills,
    ch_titan_sniper_grunt_kills,
    ch_titan_sniper_hours_used,
    ch_titan_sniper_crits,

    ch_rocket_launcher_kills,
    ch_rocket_launcher_pilot_kills,
    ch_rocket_launcher_titan_kills,
    ch_rocket_launcher_spectre_kills,
    ch_rocket_launcher_grunt_kills,
    ch_rocket_launcher_hours_used,

    ch_triple_threat_kills,
    ch_triple_threat_pilot_kills,
    ch_triple_threat_titan_kills,
    ch_triple_threat_spectre_kills,
    ch_triple_threat_grunt_kills,
    ch_triple_threat_hours_used,

    // Titan Ordnance
    ch_salvo_rockets_kills,
    ch_salvo_rockets_pilot_kills,
    ch_salvo_rockets_titan_kills,
    ch_salvo_rockets_spectre_kills,
    ch_salvo_rockets_grunt_kills,
    ch_salvo_rockets_hours_used,

    ch_homing_rockets_titan_kills,
    ch_homing_rockets_hours_used,

    ch_dumbfire_rockets_kills,
    ch_dumbfire_rockets_pilot_kills,
    ch_dumbfire_rockets_titan_kills,
    ch_dumbfire_rockets_spectre_kills,
    ch_dumbfire_rockets_grunt_kills,
    ch_dumbfire_rockets_hours_used,

    ch_shoulder_rockets_titan_kills,
    ch_shoulder_rockets_hours_used,

    // Pilot Primary
    ch_smart_pistol_kills,
    ch_smart_pistol_pilot_kills,
    ch_smart_pistol_spectre_kills,
    ch_smart_pistol_grunt_kills,
    ch_smart_pistol_hours_used,

    ch_shotgun_kills,
    ch_shotgun_pilot_kills,
    ch_shotgun_spectre_kills,
    ch_shotgun_grunt_kills,
    ch_shotgun_hours_used,

    ch_r97_kills,
    ch_r97_pilot_kills,
    ch_r97_spectre_kills,
    ch_r97_grunt_kills,
    ch_r97_hours_used,
    ch_r97_headshots,

    ch_car_kills,
    ch_car_pilot_kills,
    ch_car_spectre_kills,
    ch_car_grunt_kills,
    ch_car_hours_used,
    ch_car_headshots,

    ch_lmg_kills,
    ch_lmg_pilot_kills,
    ch_lmg_spectre_kills,
    ch_lmg_grunt_kills,
    ch_lmg_hours_used,
    ch_lmg_headshots,

    ch_rspn101_kills,
    ch_rspn101_pilot_kills,
    ch_rspn101_spectre_kills,
    ch_rspn101_grunt_kills,
    ch_rspn101_hours_used,
    ch_rspn101_headshots,

    ch_hemlok_kills,
    ch_hemlok_pilot_kills,
    ch_hemlok_spectre_kills,
    ch_hemlok_grunt_kills,
    ch_hemlok_hours_used,
    ch_hemlok_headshots,

    ch_g2_kills,
    ch_g2_pilot_kills,
    ch_g2_spectre_kills,
    ch_g2_grunt_kills,
    ch_g2_hours_used,
    ch_g2_headshots,

    ch_dmr_kills,
    ch_dmr_pilot_kills,
    ch_dmr_spectre_kills,
    ch_dmr_grunt_kills,
    ch_dmr_hours_used,
    ch_dmr_headshots,

    ch_sniper_kills,
    ch_sniper_pilot_kills,
    ch_sniper_spectre_kills,
    ch_sniper_grunt_kills,
    ch_sniper_hours_used,

    // Pilot Secondary
    ch_smr_titan_kills,
    ch_smr_crits,

    ch_mgl_titan_kills,

    ch_archer_titan_kills,

    ch_defender_titan_kills,
    ch_defender_crits,

    // Pilot Ordnance
    ch_frag_grenade_throws,
    ch_frag_grenade_kills,
    ch_frag_grenade_pilot_kills,
    ch_frag_grenade_grunt_kills,

    ch_emp_grenade_throws,
    ch_emp_grenade_kills,
    ch_emp_grenade_pilot_kills,
    ch_emp_grenade_grunt_kills,
    ch_emp_grenade_spectre_kills,

    ch_proximity_mine_throws,
    ch_proximity_mine_kills,
    ch_proximity_mine_pilot_kills,
    ch_proximity_mine_grunt_kills,

    ch_satchel_throws,
    ch_satchel_kills,
    ch_satchel_pilot_kills,
    ch_satchel_grunt_kills,
}

const DAILY_CHALLENGE_COUNT: usize = 4;

#[allow(non_camel_case_types)]
#[derive(Deserialize)]
enum DailyChallenge {
    NULL,
    ch_daily_xo16_pilot_kills,
    ch_daily_emp_grenade_kills,
    ch_daily_kills_nuclear_core,
}

#[allow(non_camel_case_types)]
#[derive(Deserialize)]
struct EChallenge {
    progress: f32,
    previousProgress: f32,
}

#[allow(non_camel_case_types)]
#[derive(Deserialize)]
struct ActiveDailyChallenge {
    reference: DailyChallenge,
    day: i32,
}

#[allow(non_camel_case_types)]
#[serde_as]
#[derive(Deserialize)]
struct PostGamePlayer {
    #[serde_as(as = "FixedString<32>")]
    name: String,
    #[serde_as(as = "FixedString<22>")]
    xuid: String,
    level: i32,
    gen: i32,
    team: i32,
    scores: [i32; 4],
    playingRanked: bool,
    rank: i32,
    callsignIconIndex: i32,
    matchPerformance: f32,
}

#[allow(non_camel_case_types)]
#[serde_as]
#[derive(Deserialize)]
struct PostGameData {
    gameMode: i32,
    map: i32,
    #[serde_as(as = "FixedString<22>")]
    myXuid: String,
    myTeam: i32,
    maxTeamSize: i32,
    factionIMC: Faction,
    factionMCOR: Faction,
    scoreIMC: i32,
    scoreMCOR: i32,
    teams: bool,
    privateMatch: bool,
    ranked: bool,
    hadMatchLossProtection: bool,
    challengeUnlocks: [RecentUnlock; 6],
    players: [PostGamePlayer; 16],
}

#[allow(non_camel_case_types)]
#[serde_as]
#[derive(Deserialize)]
struct FdPostGamePlayer {
    #[serde_as(as = "FixedString<32>")]
    name: String,
    #[serde_as(as = "FixedString<22>")]
    xuid: String,
    awardId: i32,
    awardValue: f32,
    suitIndex: i32,
}

#[allow(non_camel_case_types)]
#[derive(Deserialize)]
struct FdPostGameData {
    gameMode: i32,
    map: i32,
    myIndex: i32,
    numPlayers: i32,
    players: [FdPostGamePlayer; 4],
}

#[allow(non_camel_case_types)]
#[derive(Deserialize)]
struct Ranked {
    isPlayingRanked: bool,
    currentRank: i32,
}
