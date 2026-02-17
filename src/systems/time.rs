use bevy::prelude::*;

/// 游戏时间资源
#[derive(Resource, Clone, Debug)]
pub struct GameTime {
    pub day: u32,                    // 当前天数（1-15）
    pub hour: f32,                   // 当前小时（0-24）
    pub minute: f32,                 // 当前分钟（0-60）
    pub day_length: f32,             // 一天的实际长度（秒）
    pub current_phase: DayPhase,     // 当前昼夜阶段
    pub moon_phase: MoonPhase,       // 当前月相
}

/// 昼夜阶段
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DayPhase {
    Dawn,    // 黎明（5-7点）
    Day,     // 白天（7-18点）
    Dusk,    // 黄昏（18-20点）
    Night,   // 夜晚（20-5点）
}

/// 月相类型（15日周期）
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MoonPhase {
    NewMoon,        // 新月（第1日）
    WaxingCrescent, // 峨眉月（第2-3日）
    FirstQuarter,   // 上弦月（第4日）
    WaxingGibbous,  // 盈凸月（第5-7日）
    FullMoon,       // 满月（第8日）
    WaningGibbous,  // 亏凸月（第9-11日）
    LastQuarter,    // 下弦月（第12日）
    WaningCrescent, // 残月（第13-14日）
    DarkMoon,       // 朔月（第15日）
}

impl MoonPhase {
    /// 根据天数获取月相
    pub fn from_day(day: u32) -> Self {
        match day {
            1 => MoonPhase::NewMoon,
            2..=3 => MoonPhase::WaxingCrescent,
            4 => MoonPhase::FirstQuarter,
            5..=7 => MoonPhase::WaxingGibbous,
            8 => MoonPhase::FullMoon,
            9..=11 => MoonPhase::WaningGibbous,
            12 => MoonPhase::LastQuarter,
            13..=14 => MoonPhase::WaningCrescent,
            15 => MoonPhase::DarkMoon,
            _ => MoonPhase::NewMoon,
        }
    }

    /// 获取月相的资源倍率
    pub fn resource_multiplier(&self) -> f32 {
        match self {
            MoonPhase::NewMoon => 2.0,
            MoonPhase::WaxingCrescent => 1.8,
            MoonPhase::FirstQuarter => 1.6,
            MoonPhase::WaxingGibbous => 1.4,
            MoonPhase::FullMoon => 2.0,  // 满月特殊：资源x200%
            MoonPhase::WaningGibbous => 1.2,
            MoonPhase::LastQuarter => 1.0,
            MoonPhase::WaningCrescent => 0.8,
            MoonPhase::DarkMoon => 0.5,
        }
    }

    /// 获取月相的AI扫描强度
    pub fn scan_intensity(&self) -> f32 {
        match self {
            MoonPhase::NewMoon => 0.1,
            MoonPhase::WaxingCrescent => 0.2,
            MoonPhase::FirstQuarter => 0.3,
            MoonPhase::WaxingGibbous => 0.4,
            MoonPhase::FullMoon => 0.0,  // 满月：AI系统过热停机
            MoonPhase::WaningGibbous => 0.5,
            MoonPhase::LastQuarter => 0.6,
            MoonPhase::WaningCrescent => 0.7,
            MoonPhase::DarkMoon => 0.5,  // 朔月：终极挑战
        }
    }
}

impl DayPhase {
    /// 根据小时获取昼夜阶段
    pub fn from_hour(hour: f32) -> Self {
        if hour >= 5.0 && hour < 7.0 {
            DayPhase::Dawn
        } else if hour >= 7.0 && hour < 18.0 {
            DayPhase::Day
        } else if hour >= 18.0 && hour < 20.0 {
            DayPhase::Dusk
        } else {
            DayPhase::Night
        }
    }

    /// 获取当前阶段的光照强度（0.0-1.0）
    pub fn light_intensity(&self, hour: f32) -> f32 {
        match self {
            DayPhase::Dawn => {
                // 黎明：5点0.0 -> 7点1.0
                (hour - 5.0) / 2.0
            }
            DayPhase::Day => 1.0,
            DayPhase::Dusk => {
                // 黄昏：18点1.0 -> 20点0.0
                1.0 - (hour - 18.0) / 2.0
            }
            DayPhase::Night => 0.2,  // 夜晚保留基础亮度
        }
    }
}

impl Default for GameTime {
    fn default() -> Self {
        Self {
            day: 1,
            hour: 6.0,  // 从早上6点开始
            minute: 0.0,
            day_length: 60.0,  // 默认60秒为一天
            current_phase: DayPhase::Day,
            moon_phase: MoonPhase::NewMoon,
        }
    }
}

/// 更新游戏时间
pub fn update_time(
    time: Res<Time>,
    mut game_time: ResMut<GameTime>,
) {
    // 计算时间增量
    let delta = time.delta_seconds();

    // 计算游戏时间增量（实际秒数 / 一天的实际长度 * 24小时）
    let game_delta_hours = (delta / game_time.day_length) * 24.0;

    // 更新小时和分钟
    game_time.hour += game_delta_hours;
    game_time.minute = (game_time.hour % 1.0) * 60.0;

    // 如果超过24小时，进入下一天
    if game_time.hour >= 24.0 {
        game_time.hour -= 24.0;
        game_time.day += 1;

        // 更新月相
        game_time.moon_phase = MoonPhase::from_day(game_time.day);

        info!("进入第 {} 天，月相: {:?}", game_time.day, game_time.moon_phase);
    }

    // 更新昼夜阶段
    game_time.current_phase = DayPhase::from_hour(game_time.hour);

    // 记录时间变化（每整点）
    if game_time.minute < delta * 60.0 * 24.0 {
        info!(
            "时间: 第{}天 {:.0}:{:02.0}, 阶段: {:?}, 光照: {:.2}",
            game_time.day,
            game_time.hour,
            game_time.minute,
            game_time.current_phase,
            game_time.current_phase.light_intensity(game_time.hour)
        );
    }
}

/// 初始化游戏时间
pub fn init_game_time(mut commands: Commands) {
    commands.insert_resource(GameTime::default());
    info!("游戏时间系统初始化完成");
}
