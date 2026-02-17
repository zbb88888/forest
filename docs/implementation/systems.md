# Dark Forest 系统实现文档

## 文档说明
本文档记录了游戏各个系统的具体实现细节，包括组件定义、系统逻辑、交互方式等。

---

## 1. 环境系统

### 1.1 地图系统

#### 实现文件
- 组件：`src/components/resource.rs` (TileType, MapTile, WorldMap)
- 系统：`src/systems/map.rs` (setup_map)

#### 地形类型
```rust
pub enum TileType {
    Grass,      // 草地 - 绿色，能源产出 1.0x
    Forest,     // 森林 - 深绿色，能源产出 1.5x
    Mountain,   // 山脉 - 灰色，不可通行，能源产出 0.5x
    Water,      // 水域 - 蓝色，不可通行，能源产出 0.8x
    Desert,     // 沙漠 - 沙黄色，能源产出 0.7x
    DarkForest, // 黑暗森林 - 深暗绿色，能源产出 2.0x
}
```

#### 地图瓦片属性
```rust
pub struct MapTile {
    pub tile_type: TileType,
    pub x: u32,
    pub y: u32,
    pub explored: bool,      // 是否已探索
    pub visible: bool,        // 是否当前可见
}
```

#### 地图生成
- 尺寸：20x20 瓦片
- 瓦片大小：32x32 像素
- 地形分布：
  - 草地：50%
  - 森林：25%
  - 沙漠：10%
  - 山脉：7%
  - 水域：5%
  - 黑暗森林：3%

#### 功能实现
- `get_tile(x, y)` - 获取指定位置瓦片
- `get_tile_mut(x, y)` - 获取可变瓦片引用
- `explore_area(center_x, center_y, radius)` - 探索指定区域

### 1.2 时间系统

#### 实现文件
- 组件：`src/systems/time.rs` (GameTime, DayPhase, MoonPhase)
- 系统：`src/systems/time.rs` (update_time, init_game_time)

#### 昼夜阶段
```rust
pub enum DayPhase {
    Dawn,    // 黎明（5-7点）- 光照 0.0→1.0
    Day,     // 白天（7-18点）- 光照 1.0，植物生长 1.5x
    Dusk,    // 黄昏（18-20点）- 光照 1.0→0.0
    Night,   // 夜晚（20-5点）- 光照 0.2，植物生长 0.5x
}
```

#### 月相系统（15日周期）
```rust
pub enum MoonPhase {
    NewMoon,        // 第1日 - 资源 2.0x，AI扫描 0.1
    WaxingCrescent, // 第2-3日 - 资源 1.8x，AI扫描 0.2
    FirstQuarter,   // 第4日 - 资源 1.6x，AI扫描 0.3
    WaxingGibbous,  // 第5-7日 - 资源 1.4x，AI扫描 0.4
    FullMoon,       // 第8日 - 资源 2.0x，AI扫描 0.0（平安夜）
    WaningGibbous,  // 第9-11日 - 资源 1.2x，AI扫描 0.5
    LastQuarter,    // 第12日 - 资源 1.0x，AI扫描 0.6
    WaningCrescent, // 第13-14日 - 资源 0.8x，AI扫描 0.7
    DarkMoon,       // 第15日 - 资源 0.5x，AI扫描 0.5（终极挑战）
}
```

#### 时间参数
- 一天长度：60秒（可配置）
- 时间更新：基于实际时间增量
- 光照系统：根据昼夜阶段动态调整

### 1.3 光照系统

#### 实现文件
- 组件：`src/systems/lighting.rs` (EnvironmentLighting)
- 系统：`src/systems/lighting.rs` (update_lighting, init_lighting)

#### 光照颜色
```rust
DayPhase::Dawn  -> Color::srgb(1.0, 0.9, 0.7)  // 暖黄色
DayPhase::Day   -> Color::srgb(1.0, 1.0, 1.0)  // 白色
DayPhase::Dusk  -> Color::srgb(1.0, 0.6, 0.4)  // 橙红色
DayPhase::Night -> Color::srgb(0.3, 0.3, 0.5)  // 蓝紫色
```

#### 光照强度
- 黎明：0.0 → 1.0（渐变）
- 白天：1.0
- 黄昏：1.0 → 0.0（渐变）
- 夜晚：0.2（保留基础亮度）

---

## 2. 植物系统

### 2.1 植物组件

#### 实现文件
- 组件：`src/components/plant.rs` (Plant, PlantType, Growable)
- 系统：`src/systems/plant.rs` (plant_seed, grow_plants, harvest_plants, plant_decay)

#### 植物类型
```rust
pub enum PlantType {
    Grass,        // 草 - 绿色，生长快，产出 1.0x
    Bush,         // 灌木 - 深绿色，生长中，产出 1.5x
    Tree,         // 树木 - 深绿色，生长慢，产出 2.0x
    Flower,       // 花朵 - 粉色，生长中，产出 1.3x
    EnergyFlower, // 能源花 - 青色，生长中，产出 3.0x
}
```

#### 植物属性
```rust
pub struct Plant {
    pub plant_type: PlantType,
    pub growth_stage: u8,      // 生长阶段 (0-5)
    pub health: f32,          // 健康度 (0.0 - 1.0)
    pub maturity: f32,         // 成熟度 (0.0 - 1.0)
    pub water_level: f32,     // 水分等级 (0.0 - 1.0)
    pub nutrient_level: f32,   // 营养等级 (0.0 - 1.0)
    pub max_stages: u8,       // 最大生长阶段
    pub energy_output: f32,    // 能源产出速率
}
```

### 2.2 植物系统功能

#### 种植机制
- **操作**：右键点击地图
- **地形要求**：草地、森林、黑暗森林
- **自动选择**：
  - 草地 → 草
  - 森林 → 灌木
  - 黑暗森林 → 能源花

#### 生长系统
- **生长阶段**：5个阶段（0-4）
- **生长速度**：受昼夜影响
  - 白天：1.5x
  - 黎明/黄昏：1.0x
  - 夜晚：0.5x
- **视觉反馈**：随生长阶段改变大小
- **资源消耗**：水分和营养随时间消耗

#### 收获系统
- **操作**：左键点击成熟植物
- **收获条件**：growth_stage >= 5 且 maturity >= 1.0
- **奖励计算**：基础奖励 × 健康度 × 成熟度
- **能源奖励**：
  - 草：10
  - 灌木：20
  - 树木：50
  - 花朵：15
  - 能源花：30

---

## 3. 自动化机器人系统

### 3.1 机器人组件

#### 实现文件
- 组件：`src/components/robot.rs` (Robot, RobotType, RobotTask, RobotAI, RobotInventory)
- 系统：`src/systems/robot.rs` (spawn_robot, robot_ai_system)

#### 机器人类型
```rust
pub enum RobotType {
    Harvester,  // 采集机器人 - 棕色，速度 2.0，能耗 0.5
    Builder,    // 建造机器人 - 绿色，速度 1.5，能耗 0.8
    Defender,   // 防御机器人 - 红色，速度 2.5，能耗 0.6
    Scout,      // 侦察机器人 - 蓝色，速度 3.0，能耗 0.3
}
```

#### 机器人任务
```rust
pub enum RobotTask {
    Idle,           // 空闲
    Harvest,        // 采集
    Build,          // 建造
    Patrol,         // 巡逻
    Repair,         // 维修
    ReturnToBase,   // 返回基地
}
```

#### 机器人属性
```rust
pub struct Robot {
    pub robot_type: RobotType,
    pub current_task: RobotTask,
    pub energy: f32,
    pub max_energy: f32,
    pub efficiency: f32,
    pub target_position: Option<Vec2>,
    pub task_timer: Timer,
}
```

#### 机器人AI
```rust
pub struct RobotAI {
    pub patrol_radius: f32,      // 巡逻半径 200.0
    pub detection_radius: f32,   // 探测半径 100.0
    pub current_patrol_index: usize,
    pub patrol_points: Vec<Vec2>,
}
```

#### 机器人背包
```rust
pub struct RobotInventory {
    pub capacity: u32,
    pub current: u32,
    pub resource_type: Option<ResourceType>,
}
```

### 3.2 机器人系统功能

#### 生成方式
- **键盘 1**：采集机器人（容量 50）
- **键盘 2**：建造机器人（容量 30）
- **键盘 3**：防御机器人（容量 20）
- **键盘 4**：侦察机器人（容量 10）

#### AI行为
1. **空闲状态**
   - 采集机器人：寻找最近的成熟植物
   - 侦察机器人：随机生成巡逻点

2. **采集任务**
   - 移动到目标植物
   - 采集成熟植物
   - 背包满时返回基地

3. **巡逻任务**
   - 移动到巡逻点
   - 到达后返回空闲

4. **返回基地**
   - 移动到玩家位置
   - 卸载资源
   - 充满能量

#### 能量管理
- **能量消耗**：每秒消耗固定能量
- **能量不足**：自动返回基地
- **基地充能**：返回后充满能量

#### 夜晚影响
- 夜晚机器人效率降低至 50%
- 采集和移动速度变慢

---

## 4. 装备和武器系统

### 4.1 装备组件

#### 实现文件
- 组件：`src/components/equipment.rs` (Equipment, EquipmentType, EquipmentRarity, EquipmentBar)
- 系统：`src/systems/equipment.rs` (spawn_random_equipment, pickup_equipment, upgrade_equipment)

#### 装备类型
```rust
pub enum EquipmentType {
    // 武器
    LaserGun,       // 激光枪 - 伤害 10，攻速 1.0，暴击 5%
    PlasmaCannon,   // 等离子炮 - 伤害 25，攻速 0.6，暴击 10%
    EMPBlaster,     // EMP爆破枪 - 伤害 15，攻速 0.8，暴击 15%
    Railgun,        // 轨道炮 - 伤害 40，攻速 0.4，暴击 20%

    // 护甲
    LightArmor,     // 轻型护甲 - 防御 15
    HeavyArmor,     // 重型护甲 - 防御 30
    EnergyShield,    // 能量护盾 - 防御 20，能量+10

    // 饰品
    SolarPanel,     // 太阳能板 - 能量+5
    BatteryPack,    // 电池包 - 能量+50
    TargetingSystem,// 瞄准系统 - 暴击+15%
}
```

#### 装备稀有度
```rust
pub enum EquipmentRarity {
    Common,      // 普通 - 白色，1.0x，60%
    Uncommon,    // 稀有 - 绿色，1.2x，25%
    Rare,        // 史诗 - 蓝色，1.5x，10%
    Legendary,   // 传说 - 紫色，2.0x，4%
    Mythic,      // 神话 - 金色，3.0x，1%
}
```

#### 装备属性
```rust
pub struct EquipmentStats {
    pub damage: f32,          // 伤害
    pub attack_speed: f32,    // 攻击速度
    pub defense: f32,        // 防御力
    pub energy_bonus: f32,    // 能量加成
    pub crit_chance: f32,     // 暴击率
    pub crit_multiplier: f32,  // 暴击倍率
}
```

#### 装备栏
```rust
pub struct EquipmentBar {
    pub weapon: Option<Entity>,
    pub armor: Option<Entity>,
    pub accessory: Option<Entity>,
}
```

### 4.2 装备系统功能

#### 生成装备
- **操作**：键盘 E
- **位置**：玩家当前位置
- **随机性**：
  - 随机选择装备类型
  - 加权随机稀有度
- **视觉**：根据稀有度显示不同颜色

#### 拾取装备
- **操作**：键盘 F
- **范围**：50像素内
- **自动装备**：自动装备到对应槽位
- **替换规则**：槽位已有装备时自动替换

#### 升级装备
- **操作**：键盘 U
- **成本**：50能源
- **等级上限**：10级
- **升级效果**：
  - 伤害、防御、能量加成：1.0 + (等级 × 0.1)
  - 暴击率：保持不变
  - 暴击倍率：保持不变

#### 属性计算
总属性 = 所有装备属性之和（暴击倍率取最大值）

---

## 5. UI系统

### 5.1 HUD显示

#### 实现文件
- 系统：`src/ui/hud.rs` (setup_hud, update_hud)

#### 左侧资源栏
- 能源（黄色）
- 金属（灰色）
- 土壤（棕色）

#### 右侧信息栏
- 时间（白色）- 显示天数和当前时间
- 昼夜阶段（浅黄色）- Dawn/Day/Dusk/Night
- 月相（浅蓝色）- 显示当前月相名称

---

## 6. 玩家系统

### 6.1 玩家组件

#### 实现文件
- 组件：`src/components/player.rs` (Player, PlayerBundle)
- 系统：`src/systems/player.rs` (spawn_player, move_player_randomly)

#### 玩家属性
```rust
pub struct Player {
    pub id: u64,
    pub name: String,
}
```

#### 玩家背包
```rust
pub struct Inventory {
    pub metal: u32,
    pub soil: u32,
    pub energy: u32,
}
```

#### 玩家装备栏
```rust
pub struct EquipmentBar {
    pub weapon: Option<Entity>,
    pub armor: Option<Entity>,
    pub accessory: Option<Entity>,
}
```

### 6.2 玩家功能

#### 生成
- 位置：地图中心
- 初始资源：能源 100
- 初始装备：无

#### 移动
- 当前实现：随机移动（每1秒随机方向）
- 未来计划：WASD控制

---

## 7. 系统集成

### 7.1 主程序集成

#### 实现文件
- 主程序：`src/main.rs`

#### 系统调度
```rust
.add_systems(Startup, setup)
.add_systems(Startup, systems::time::init_game_time)
.add_systems(Startup, systems::lighting::init_lighting)
.add_systems(OnEnter(GameState::InGame), (
    systems::map::setup_map,
    systems::player::spawn_player
).chain())
.add_systems(Update, (
    systems::time::update_time,
    systems::lighting::update_lighting,
    systems::player::move_player_randomly,
    systems::plant::plant_seed,
    systems::plant::grow_plants,
    systems::plant::harvest_plants,
    systems::plant::plant_decay,
    systems::robot::spawn_robot,
    systems::robot::robot_ai_system,
    systems::equipment::spawn_random_equipment,
    systems::equipment::pickup_equipment,
    systems::equipment::upgrade_equipment,
    systems::equipment::display_equipment_info,
).run_if(in_state(GameState::InGame)))
```

### 7.2 游戏状态

#### 实现文件
- 组件：`src/states/mod.rs` (GameState)

#### 游戏状态
```rust
pub enum GameState {
    Loading,    // 加载状态
    MainMenu,   // 主菜单（默认）
    InGame,     // 游戏中
    Paused,     // 暂停状态
    GameOver,   // 游戏结束
}
```

---

## 8. 待实现功能

### 8.1 战斗系统
- 敌人组件和AI
- 武器攻击系统
- 伤害计算
- 暴击系统

### 8.2 建造系统
- 建筑类型定义
- 建造机制
- 建筑功能
- 防御塔系统

### 8.3 UI优化
- 装备栏显示
- 技能栏
- 更多游戏信息显示

### 8.4 数值平衡
- 各系统数值调整
- 游戏难度曲线优化
- 资源平衡

---

## 9. 技术栈

### 9.1 核心技术
- **语言**：Rust
- **引擎**：Bevy 0.18.1
- **架构**：ECS（Entity-Component-System）

### 9.2 主要依赖
- bevy（游戏引擎）
- serde（序列化）
- rand（随机数生成）

### 9.3 渲染
- 原生 Bevy Sprite
- Required Components
- 2D 渲染

---

## 10. 代码组织

### 10.1 目录结构
```
src/
├── components/       # ECS组件
│   ├── player.rs
│   ├── plant.rs
│   ├── robot.rs
│   ├── equipment.rs
│   └── ...
├── systems/         # 游戏系统
│   ├── map.rs
│   ├── time.rs
│   ├── plant.rs
│   ├── robot.rs
│   ├── equipment.rs
│   └── ...
├── resources/       # 游戏资源
│   └── world.rs
├── states/         # 游戏状态
│   └── mod.rs
├── ui/            # 用户界面
│   ├── hud.rs
│   ├── menu.rs
│   └── ...
└── main.rs         # 程序入口
```

### 10.2 模块划分
- **components**：所有ECS组件定义
- **systems**：所有游戏逻辑系统
- **resources**：全局资源定义
- **states**：游戏状态管理
- **ui**：用户界面系统
- **utils**：工具函数

---

## 11. 性能优化

### 11.1 ECS优化
- 使用查询过滤器
- 避免频繁组件添加/删除
- 合理的系统调度

### 11.2 渲染优化
- 视锥体剔除（待实现）
- 批处理渲染
- LOD系统（待实现）

### 11.3 内存管理
- 对象池模式（待实现）
- 预分配内存
- 避免运行时分配

---

## 12. 扩展性

### 12.1 模块化设计
- 插件化架构
- 系统间低耦合
- 清晰的接口定义

### 12.2 数据驱动
- 配置化游戏参数
- 可热重载的资源
- 灵活的实体模板系统（待实现）

---

## 更新日志

### v0.1.0 (当前版本)
- ✅ 完成环境系统（地图、时间、光照）
- ✅ 完成植物系统（种植、生长、收获）
- ✅ 完成机器人系统（AI、任务、采集）
- ✅ 完成装备系统（生成、拾取、升级）
- ✅ 基础UI系统（HUD、菜单）
- ✅ 游戏状态管理
