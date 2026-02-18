# Dark Forest 系统依赖关系图

## 文档说明

本文档定义了游戏各系统之间的单向依赖关系，是后续开发优先级判断的第一参考。

**依赖原则**：

1. 依赖关系必须是单向的（避免循环依赖）
2. 基础系统优先实现
3. 高层系统依赖低层系统
4. 同层系统之间尽量减少依赖

---

## 系统分层 (Sys0 - Sys999)

### Sys0: 纯净地图 (基础渲染 + UI)

**依赖**: 无

包含系统：

- `map` - 地图系统，提供地形、瓦片数据
- `player` - 玩家系统，提供玩家实体
- `time` - 时间系统，提供昼夜、月相数据
- `lighting` - 光照系统，提供动态光照效果
- `menu` - 菜单系统
- `hud` - HUD界面系统

---

### Sys1: 实体生成

**依赖**: Sys0

包含系统：

- `enemy_spawn` - 敌人生成系统
- `enemy_base` - 敌人基地系统
- `plant` - 植物系统（种植、生长、收获）

---

### Sys2: 实体行为

**依赖**: Sys1

包含系统：

- `enemy` - 敌人AI系统
- `robot` - 机器人系统
- `equipment` - 装备系统

---

### Sys3: 战斗系统

**依赖**: Sys2

包含系统：

- `enemy_attack` - 敌人攻击系统
- `player_combat` - 玩家战斗系统
- `combat` - 战斗核心系统
- `combat_effects` - 战斗效果系统
- `defense_tower` - 防御塔系统
- `defense_wall` - 墙壁防御系统
- `defense_range` - 防御范围系统

---

### Sys4: 生产建造

**依赖**: Sys3

包含系统：

- `plant_upgrade` - 植物升级系统
- `crafting` - 合成系统
- `building` - 建筑系统
- UI: `PlantUpgradeUI`, `CraftingUI`, `BuildingUI`

---

### Sys50: 任务成就

**依赖**: Sys4

包含系统：

- `quest_manager` - 任务管理器
- `quest_events` - 任务事件系统
- `quest_generator` - 任务生成器
- `achievement_manager` - 成就管理器
- `achievement_events` - 成就事件系统
- `achievement_generator` - 成就生成器

---

### Sys999: 存档系统 (完整版)

**依赖**: Sys50

包含系统：

- `save_manager` - 存档管理器
- `save_ui` - 存档UI系统

---

## 依赖关系图

```
Sys0: 纯净地图 (基础渲染 + UI)
├── map (地图)
├── player (玩家)
├── time (时间)
├── lighting (光照)
├── menu (菜单)
└── hud (HUD)
        │
        ▼
Sys1: 实体生成 ◄─────────────┐
├── enemy_spawn (敌人生成)    │
├── enemy_base (敌人基地)     │
└── plant (植物) ◄────────────┤
        │                     │
        ▼                     │
Sys2: 实体行为 ◄──────────────┤
├── enemy (敌人AI)            │
├── robot (机器人)            │
└── equipment (装备) ◄────────┤
        │                     │
        ▼                     │
Sys3: 战斗系统 ◄──────────────┤
├── enemy_attack (敌人攻击)   │
├── player_combat (玩家战斗)  │
├── combat (战斗核心)         │
├── combat_effects (战斗效果) │
├── defense_tower (防御塔)    │
├── defense_wall (墙壁)       │
└── defense_range (防御范围)  │
        │                     │
        ▼                     │
Sys4: 生产建造 ◄──────────────┤
├── plant_upgrade (植物升级)  │
├── crafting (合成)           │
├── building (建筑)           │
└── UI (升级/合成/建筑界面)   │
        │                     │
        ▼                     │
Sys50: 任务成就 ◄─────────────┤
├── quest_manager (任务管理)  │
├── quest_events (任务事件)   │
├── quest_generator (任务生成)│
├── achievement_manager (成就)│
├── achievement_events (成就) │
└── achievement_generator     │
        │                     │
        ▼                     │
Sys999: 存档系统 ◄────────────┘
├── save_manager (存档管理)
└── save_ui (存档UI)
```

---

## 详细依赖关系

### 1. Sys0: 纯净地图

**依赖**: 无

**包含**:

- `src/systems/map.rs` - 地图系统
- `src/systems/player.rs` - 玩家系统
- `src/systems/time.rs` - 时间系统
- `src/systems/lighting.rs` - 光照系统
- `src/ui/menu.rs` - 菜单系统
- `src/ui/hud.rs` - HUD系统

**被依赖**: 所有其他系统

---

### 2. Sys1: 实体生成

**依赖**: Sys0

**包含**:

- `src/systems/enemy_spawn.rs` - 敌人生成
- `src/systems/enemy_base.rs` - 敌人基地
- `src/systems/plant.rs` - 植物系统

**依赖 Sys0**:

- 地图系统（位置检测）
- 玩家系统（交互）
- 时间系统（生长影响）

---

### 3. Sys2: 实体行为

**依赖**: Sys1

**包含**:

- `src/systems/enemy.rs` - 敌人AI
- `src/systems/robot.rs` - 机器人系统
- `src/systems/equipment.rs` - 装备系统

**依赖 Sys1**:

- 敌人生成系统（敌人实体）
- 植物系统（采集目标）

---

### 4. Sys3: 战斗系统

**依赖**: Sys2

**包含**:

- `src/systems/enemy_attack.rs` - 敌人攻击
- `src/systems/player_combat.rs` - 玩家战斗
- `src/systems/combat.rs` - 战斗核心
- `src/systems/combat_effects.rs` - 战斗效果
- `src/systems/defense_tower.rs` - 防御塔
- `src/systems/defense_wall.rs` - 墙壁
- `src/systems/defense_range.rs` - 防御范围

**依赖 Sys2**:

- 敌人AI（攻击行为）
- 机器人系统（防御机器人）
- 装备系统（武器属性）

---

### 5. Sys4: 生产建造

**依赖**: Sys3

**包含**:

- `src/systems/plant_upgrade.rs` - 植物升级
- `src/systems/crafting.rs` - 合成系统
- `src/systems/building.rs` - 建筑系统
- `src/ui/plant_upgrade.rs` - 植物升级UI
- `src/ui/crafting.rs` - 合成UI
- `src/ui/building.rs` - 建筑UI

**依赖 Sys3**:

- 战斗系统（防御建筑）
- 装备系统（合成材料）

---

### 6. Sys50: 任务成就

**依赖**: Sys4

**包含**:

- `src/systems/quest_manager.rs` - 任务管理
- `src/systems/quest_events.rs` - 任务事件
- `src/systems/quest_generator.rs` - 任务生成
- `src/systems/achievement_manager.rs` - 成就管理
- `src/systems/achievement_events.rs` - 成就事件
- `src/systems/achievement_generator.rs` - 成就生成

**依赖 Sys4**:

- 生产建造系统（任务目标）
- 战斗系统（击杀任务）

---

### 7. Sys999: 存档系统

**依赖**: Sys50

**包含**:

- `src/systems/save_manager.rs` - 存档管理
- `src/systems/save_ui.rs` - 存档UI

**依赖 Sys5**:

- 所有系统（需要保存完整游戏状态）

---

## 依赖关系矩阵

| 系统 | Sys0 | Sys1 | Sys2 | Sys3 | Sys4 | Sys50 | Sys999 |
|------|------|------|------|------|------|-------|--------|
| Sys0 | - | - | - | - | - | - | - |
| Sys1 | ✓ | - | - | - | - | - | - |
| Sys2 | ✓ | ✓ | - | - | - | - | - |
| Sys3 | ✓ | ✓ | ✓ | - | - | - | - |
| Sys4 | ✓ | ✓ | ✓ | ✓ | - | - | - |
| Sys50 | ✓ | ✓ | ✓ | ✓ | ✓ | - | - |
| Sys999 | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | - |

**说明**: ✓ 表示依赖（行依赖列）

---

## 开发规则

### 基础规则

1. **单向依赖**: 系统A依赖系统B，则B不能依赖A
2. **分层原则**: 高层系统只能依赖低层系统
3. **最小依赖**: 系统应尽量减少依赖
4. **接口清晰**: 依赖通过明确的接口进行

### 开发规则

1. **自底向上**: 从低层系统开始实现
2. **依赖优先**: 实现系统前确保其依赖已实现
3. **接口先行**: 先定义接口，再实现功能
4. **逐步集成**: 每完成一层，进行集成测试

### 测试规则

使用 Makefile 进行分层测试：

```bash
make run-sys0   # 测试纯净地图
make run-sys1   # 测试实体生成
make run-sys2   # 测试实体行为
make run-sys3   # 测试战斗系统
make run-sys4   # 测试生产建造
make run-sys50  # 测试任务成就
make run-sys999 # 测试完整存档系统
```

---

## 版本历史

### v2.0.0 (当前版本)

- 重构为 Sys0-Sys6 六层架构
- 更新 Makefile 使用 `run-sys*` 命名
- 明确每层包含的系统
- 记录系统间实际依赖关系

### v1.0.0

- 初始版本
- 定义10个基础系统依赖关系

---

## 维护说明

### 更新流程

1. 新增系统时：
   - 确定系统所属层级
   - 列出所有依赖
   - 检查是否产生循环
   - 更新依赖关系图
   - 更新依赖关系矩阵

2. 修改系统时：
   - 检查依赖关系是否变化
   - 如有变化，更新本文档

3. 删除系统时：
   - 更新被依赖系统
   - 从依赖关系图中移除
   - 更新依赖关系矩阵

### 注意事项

- 保持依赖关系的单向性
- 避免跨层依赖
- 最小化依赖数量
- 定期审查依赖关系合理性
