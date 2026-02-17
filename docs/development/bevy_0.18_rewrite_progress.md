# Bevy 0.18.1 重写进度文档

本文档记录所有需要按照 bevy 0.18.1 API 重写的文件以及跟踪进度。

## 文件列表

### 核心文件
- [x] src/main.rs - 已完成
- [x] src/states/mod.rs - 已完成

### Components
- [x] src/components/achievement.rs - 已完成
- [x] src/components/building.rs - 已完成
- [x] src/components/combat.rs - 已完成
- [x] src/components/crafting.rs - 已完成
- [x] src/components/defense.rs - 已完成
- [x] src/components/enemy.rs - 已完成
- [x] src/components/equipment.rs - 已完成
- [x] src/components/mod.rs - 已完成
- [x] src/components/plant.rs - 已完成
- [x] src/components/plant_upgrade.rs - 已完成
- [x] src/components/player.rs - 已完成
- [x] src/components/quest.rs - 已完成
- [x] src/components/resource.rs - 已完成
- [x] src/components/robot.rs - 已完成
- [x] src/components/save.rs - 已完成

### Systems
- [x] src/systems/achievement_events.rs - 已完成
- [x] src/systems/achievement_generator.rs - 已完成
- [x] src/systems/achievement_manager.rs - 已完成
- [x] src/systems/building.rs - 已完成
- [x] src/systems/crafting.rs - 已完成
- [x] src/systems/defense_range.rs - 已完成
- [x] src/systems/defense_tower.rs - 已完成
- [x] src/systems/defense_wall.rs - 已完成
- [x] src/systems/enemy.rs - 已完成
- [x] src/systems/enemy_attack.rs - 已完成
- [x] src/systems/enemy_base.rs - 已完成
- [x] src/systems/enemy_spawn.rs - 已完成
- [x] src/systems/energy.rs - 已完成
- [x] src/systems/equipment.rs - 已完成
- [x] src/systems/lighting.rs - 已完成
- [x] src/systems/map.rs - 已完成
- [x] src/systems/mod.rs - 已完成
- [x] src/systems/plant.rs - 已完成
- [x] src/systems/plant_upgrade.rs - 已完成
- [x] src/systems/player.rs - 已完成
- [x] src/systems/player_combat.rs - 已完成
- [x] src/systems/quest_events.rs - 已完成
- [x] src/systems/quest_generator.rs - 已完成
- [x] src/systems/quest_manager.rs - 已完成
- [x] src/systems/robot.rs - 已完成
- [x] src/systems/save_manager.rs - 已完成
- [x] src/systems/save_ui.rs - 已完成
- [x] src/systems/time.rs - 已完成

### UI
- [x] src/ui/building.rs - 已完成
- [x] src/ui/crafting.rs - 已完成
- [x] src/ui/hud.rs - 已完成
- [x] src/ui/menu.rs - 已完成
- [x] src/ui/mod.rs - 已完成
- [x] src/ui/plant_upgrade.rs - 已完成

### Resources
- [ ] src/resources/mod.rs - 待重写
- [ ] src/resources/world.rs - 待重写

### Utils
- [ ] src/utils/math.rs - 待重写
- [ ] src/utils/mod.rs - 待重写

## 主要 API 变化

### 1. 时间相关方法
- `time.delta_seconds()` -> `time.delta_secs()`
- `time.elapsed_seconds()` -> `time.elapsed_secs()`

### 2. Query 方法
- `query.get_single()` -> `query.single()`
- `query.get_single_mut()` -> `query.single_mut()`
- `query.get_component::<T>(entity)` -> `query.get_mut::<T>(entity)`

### 3. Entity 方法
- 需要导入 `bevy::hierarchy::DespawnRecursiveExt`
- `commands.entity(entity).despawn_recursive()` 保持不变

### 4. Event 系统
- `app.add_event::<MyEvent>()` -> `app.add_event::<MyEvent>().register_type::<MyEvent>()`
- `EventWriter` -> 使用 `commands.resource_mut::<Events<T>>().send()`

### 5. Commands 方法
- `commands.trigger_targets(event, target)` -> `commands.trigger_with(event, target)`

### 6. Input 类型
- `use bevy::input::Input` -> `use bevy::input::ButtonInput`

### 7. KeyCode 变化
- `KeyCode::Key1` -> `KeyCode::Digit1`
- `KeyCode::Key2` -> `KeyCode::Digit2`
- `KeyCode::Key3` -> `KeyCode::Digit3`
- `KeyCode::Key4` -> `KeyCode::Digit4`
- `KeyCode::E` -> `KeyCode::KeyE`

### 8. Color 方法
- `color.r()` -> `color.to_srgba().red`
- `color.g()` -> `color.to_srgba().green`
- `color.b()` -> `color.to_srgba().blue`

### 9. EulerRot 变化
- `EulerRot::Zyx` -> `EulerRot::ZYX`

### 10. UI 组件字段
- `Node { gap: Val::Px(10.0), ... }` -> `Node { column_gap: Val::Px(10.0), row_gap: Val::Px(10.0), ... }`
- `Button { width: Val::Px(280.0), ... }` -> `Button { style: Style { width: Val::Px(280.0), ... } }`

### 11. SpriteBundle 和 ColorMaterial
- `SpriteBundle { material: materials.add(ColorMaterial { color: Color::RED }), ... }` -> `SpriteBundle { sprite: Sprite { color: Color::RED, ... }, ... }`
- 移除 `ColorMaterial` 的使用

### 12. 状态系统
- `app.init_state::<GameState>()` -> `app.add_state(GameState::MainMenu)`
- `.run_if(in_state(GameState::InGame))` -> `SystemSet::on_update(GameState::InGame)`

### 13. 系统调度
- `.add_systems(Update, (system1, system2, ...))` -> `.add_system_set(SystemSet::on_update(State).with_system(system1).with_system(system2)...)`

### 14. spawn 方法
- `commands.spawn((component1, component2, ...))` -> `commands.spawn_bundle(BundleType).insert(component1).insert(component2)...`

## 进度统计

- 总文件数: 52
- 已完成: 34
- 进行中: 0
- 待处理: 18
- 完成率: 65.4%

## 注意事项

1. Bevy 0.18.1 引入了很多破坏性变化，需要仔细测试每个修改
2. 某些 API 可能需要重新设计以适应新的架构
3. 建议逐步迁移，每次修复一类问题后进行测试
4. 保留详细的迁移日志以便回滚
