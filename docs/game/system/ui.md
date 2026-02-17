# UI系统

## 系统概述
UI系统提供了游戏的用户界面，包括主菜单、HUD（抬头显示）和各种信息面板，是玩家与游戏交互的重要桥梁。

---

## 1. 主菜单系统

### 1.1 实现文件
- **系统**：`src/ui/menu.rs` (MenuPlugin, setup_menu, menu_action, cleanup_menu)

### 1.2 菜单结构

```
MenuRoot (全屏容器)
├── Title (标题文本)
├── Spacing (间距)
└── StartButton (开始按钮)
    └── ButtonText (按钮文本)
```

### 1.3 菜单样式

| 元素 | 属性 | 值 |
|------|------|-----|
| 背景 | 颜色 | (0.1, 0.1, 0.1, 1.0) |
| 标题 | 字体大小 | 60.0 |
| 标题 | 颜色 | 白色 |
| 按钮 | 宽度 | 200px |
| 按钮 | 高度 | 65px |
| 按钮 | 背景色 | (0.2, 0.2, 0.2, 1.0) |
| 按钮 | 悬停色 | (0.3, 0.3, 0.3, 1.0) |
| 按钮文本 | 字体大小 | 30.0 |
| 按钮文本 | 颜色 | 白色 |

### 1.4 交互逻辑

#### 按钮状态
```rust
match interaction {
    Interaction::Pressed => {
        game_state.set(GameState::InGame);  // 进入游戏
    }
    Interaction::Hovered => {
        color.0 = Color::srgb(0.3, 0.3, 0.3);  // 悬停效果
    }
    Interaction::None => {
        color.0 = Color::srgb(0.2, 0.2, 0.2);  // 默认状态
    }
}
```

### 1.5 生命周期
- **OnEnter**：创建菜单
- **Update**：处理交互
- **OnExit**：清理菜单

---

## 2. HUD系统

### 2.1 实现文件
- **系统**：`src/ui/hud.rs` (HUDPlugin, setup_hud, update_hud, cleanup_hud)

### 2.2 HUD结构

#### 左侧资源栏
```
HUDRoot (资源栏容器)
├── EnergyText (能源文本)
├── MetalText (金属文本)
└── SoilText (土壤文本)
```

#### 右侧信息栏
```
InfoRoot (信息栏容器)
├── TimeText (时间文本)
├── DayPhaseText (昼夜阶段文本)
└── MoonPhaseText (月相文本)
```

### 2.3 资源栏样式

| 元素 | 属性 | 值 |
|------|------|-----|
| 容器 | 位置 | 左上角 (10, 10) |
| 容器 | 背景色 | (0.0, 0.0, 0.0, 0.5) |
| 能源文本 | 颜色 | 黄色 (1.0, 1.0, 0.0) |
| 金属文本 | 颜色 | 灰色 (0.8, 0.8, 0.8) |
| 土壤文本 | 颜色 | 棕色 (0.6, 0.4, 0.2) |
| 文本 | 字体大小 | 20.0 |

### 2.4 信息栏样式

| 元素 | 属性 | 值 |
|------|------|-----|
| 容器 | 位置 | 右上角 (10, 10) |
| 容器 | 对齐 | 右对齐 |
| 容器 | 背景色 | (0.0, 0.0, 0.0, 0.5) |
| 时间文本 | 颜色 | 白色 |
| 昼夜阶段文本 | 颜色 | 浅黄 (0.9, 0.9, 0.7) |
| 月相文本 | 颜色 | 浅蓝 (0.7, 0.7, 0.9) |
| 文本 | 字体大小 | 18-20.0 |

### 2.5 更新逻辑

#### 资源更新
```rust
for mut text in energy_query.iter_mut() {
    text.0 = format!("Energy: {}", inventory.energy);
}
for mut text in metal_query.iter_mut() {
    text.0 = format!("Metal: {}", inventory.metal);
}
for mut text in soil_query.iter_mut() {
    text.0 = format!("Soil: {}", inventory.soil);
}
```

#### 时间更新
```rust
text.0 = format!("Day {} {:02.0}:{:02.0}",
    game_time.day,
    game_time.hour,
    game_time.minute
);
```

#### 昼夜阶段更新
```rust
let phase_name = match game_time.current_phase {
    DayPhase::Dawn => "Dawn",
    DayPhase::Day => "Day",
    DayPhase::Dusk => "Dusk",
    DayPhase::Night => "Night",
};
```

#### 月相更新
```rust
let phase_name = match game_time.moon_phase {
    MoonPhase::NewMoon => "New Moon",
    MoonPhase::WaxingCrescent => "Waxing Crescent",
    MoonPhase::FirstQuarter => "First Quarter",
    MoonPhase::WaxingGibbous => "Waxing Gibbous",
    MoonPhase::FullMoon => "Full Moon",
    MoonPhase::WaningGibbous => "Waning Gibbous",
    MoonPhase::LastQuarter => "Last Quarter",
    MoonPhase::WaningCrescent => "Waning Crescent",
    MoonPhase::DarkMoon => "Dark Moon",
};
```

---

## 3. 系统集成

### 3.1 初始化
```rust
app.add_plugins((MenuPlugin, HUDPlugin));
```

### 3.2 更新顺序
```rust
// 主菜单
.add_systems(OnEnter(GameState::MainMenu), setup_menu)
.add_systems(Update, menu_action.run_if(in_state(GameState::MainMenu)))
.add_systems(OnExit(GameState::MainMenu), cleanup_menu)

// HUD
.add_systems(OnEnter(GameState::InGame), setup_hud)
.add_systems(Update, update_hud.run_if(in_state(GameState::InGame)))
.add_systems(OnExit(GameState::InGame), cleanup_hud)
```

### 3.3 依赖关系
- 依赖游戏状态系统
- 依赖玩家系统（获取背包）
- 依赖时间系统（获取时间）

---

## 4. 性能优化

### 4.1 更新优化
- 仅更新变化的文本
- 使用脏标记
- 批量更新

### 4.2 渲染优化
- 使用UI层级
- 避免重叠绘制
- 使用纹理图集

### 4.3 内存优化
- 重用UI组件
- 避免频繁创建/销毁
- 对象池管理

---

## 5. 待实现功能

### 5.1 主菜单扩展
- [ ] 设置菜单
- [ ] 加载游戏
- [ ] 退出游戏
- [ ] 背景动画

### 5.2 HUD扩展
- [ ] 小地图
- [ ] 任务列表
- [ ] 技能栏
- [ ] 快捷键提示

### 5.3 新界面
- [ ] 背包界面
- [ ] 装备栏界面
- [ ] 技能树界面
- [ ] 设置界面
- [ ] 暂停菜单
- [ ] 游戏结束界面

### 5.4 UI效果
- [ ] 动画过渡
- [ ] 粒子效果
- [ ] 音效反馈
- [ ] 震动反馈

---

## 6. 已知问题

### 6.1 功能问题
- 缺少背包界面
- 缺少装备栏界面
- 缺少设置菜单
- 缺少暂停功能

### 6.2 显示问题
- 文字可能重叠
- 缺少自适应布局
- 分辨率适配不完善

### 6.3 交互问题
- 缺少快捷键提示
- 缺少操作反馈
- 缺少确认对话框

---

## 7. 更新日志

### v0.1.0 (当前版本)
- ✅ 实现主菜单
- ✅ 实现HUD资源栏
- ✅ 实现HUD信息栏
- ✅ 实现动态更新
- ✅ 实现按钮交互
