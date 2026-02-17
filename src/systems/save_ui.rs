use bevy::prelude::*;
use crate::components::save::{SaveInfo, SaveType};

/// 存档UI系统插件
pub struct SaveUIPlugin;

impl Plugin for SaveUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            update_save_ui,
        ).run_if(in_state(crate::states::GameState::InGame)));
    }
}

/// 存档UI状态
#[derive(Resource, Default)]
pub struct SaveUIState {
    pub show_save_menu: bool,
    pub show_load_menu: bool,
    pub selected_save_id: Option<String>,
    pub save_list: Vec<SaveInfo>,
}

/// 更新存档UI
fn update_save_ui(
    mut ui_state: ResMut<SaveUIState>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    // ESC键关闭存档菜单
    if keyboard_input.just_pressed(KeyCode::Escape) {
        if ui_state.show_save_menu || ui_state.show_load_menu {
            ui_state.show_save_menu = false;
            ui_state.show_load_menu = false;
            ui_state.selected_save_id = None;
        }
    }

    // F5键快速存档
    if keyboard_input.just_pressed(KeyCode::F5) {
        // 触发快速存档
        info!("触发快速存档");
    }

    // F9键快速加载
    if keyboard_input.just_pressed(KeyCode::F9) {
        // 触发快速加载
        info!("触发快速加载");
    }
}

/// 显示存档菜单
pub fn show_save_menu(ui_state: &mut SaveUIState) {
    ui_state.show_save_menu = true;
    ui_state.show_load_menu = false;
}

/// 显示加载菜单
pub fn show_load_menu(ui_state: &mut SaveUIState) {
    ui_state.show_load_menu = true;
    ui_state.show_save_menu = false;
}

/// 隐藏存档菜单
pub fn hide_save_menu(ui_state: &mut SaveUIState) {
    ui_state.show_save_menu = false;
    ui_state.show_load_menu = false;
    ui_state.selected_save_id = None;
}

/// 选择存档
pub fn select_save(ui_state: &mut SaveUIState, save_id: String) {
    ui_state.selected_save_id = Some(save_id);
}

/// 获取选中的存档ID
pub fn get_selected_save_id(ui_state: &SaveUIState) -> Option<&String> {
    ui_state.selected_save_id.as_ref()
}

/// 格式化存档时间
pub fn format_save_time(real_time: f64) -> String {
    use std::time::{UNIX_EPOCH, Duration};

    let datetime = UNIX_EPOCH + Duration::from_secs_f64(real_time);

    // 使用chrono库格式化时间
    // 这里简化处理
    format!("{:?}", datetime)
}

/// 格式化游戏时间
pub fn format_game_time(game_time: f32) -> String {
    let hours = (game_time / 3600.0) as u32;
    let minutes = ((game_time % 3600.0) / 60.0) as u32;
    let seconds = (game_time % 60.0) as u32;

    format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
}

/// 获取存档类型名称
pub fn get_save_type_name(save_type: SaveType) -> &'static str {
    match save_type {
        SaveType::Auto => "自动存档",
        SaveType::Manual => "手动存档",
        SaveType::Quick => "快速存档",
        SaveType::Checkpoint => "检查点",
    }
}

/// 获取存档类型颜色
pub fn get_save_type_color(save_type: SaveType) -> Color {
    match save_type {
        SaveType::Auto => Color::srgb(0.5, 0.8, 1.0),
        SaveType::Manual => Color::srgb(0.8, 0.8, 0.8),
        SaveType::Quick => Color::srgb(1.0, 0.8, 0.5),
        SaveType::Checkpoint => Color::srgb(1.0, 0.5, 0.5),
    }
}
