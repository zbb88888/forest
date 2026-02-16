# 《黑暗森林》(Dark Forest) 开发设计文档

## 1. 项目愿景与市场定位

### 1.1 项目概述
《黑暗森林》是一款结合自动化生产、塔防战斗和双人协作的2D像素风格游戏，旨在通过独特的月相循环机制和数值膨胀系统，为玩家带来全新的策略体验。

### 1.2 市场定位
- **目标排名**: Steam 排行榜 Top 80-100
- **核心标签**: 2D像素、自动化、塔防、在线合作
- **参考定价**: $14.99 - $19.99
- **目标用户**: 喜欢策略游戏、自动化玩法和合作体验的玩家

### 1.3 核心差异化
- 结合《It Takes Two》的双人协作感与《Beyond the Doors》的自动化逻辑
- 引入基于月相的15日极致数值膨胀循环
- 独特的生态系统扩张与资源管理机制

## 2. 技术架构

### 2.1 核心技术栈
- **编程语言**: Rust (内存安全、高性能)
- **游戏引擎**: Bevy 0.18.1 本地源码 (ECS架构，使用 Required Components)
- **网络库**: matchbox/ggrs (P2P双人同步)
- **数值处理**: u128/BigInt库
- **渲染**: 原生 Bevy Sprite 和 Tilemap，不使用外部 crate

### 2.2 ECS数据驱动设计

#### Entities (实体)
- 灵宠 (Pets)
- 自动化机器人 (Robots)
- 基因植物 (Plants)
- AI机械怪 (Enemies)

#### Components (组件)
```rust
struct EnergyStorage {
    current: u128,
    capacity: u128,
}

struct Multiplier {
    value: f64,
    source: MultiplierSource,
}

struct BiologicalLegacy {
    genes: Vec<Gene>,
    inherited_traits: Vec<Trait>,
}

struct ResourceStorage {
    metal: u32,
    soil: u32,
    energy: f32,
    shields: f32,
}
```

#### Systems (系统)
- **DayNightSystem**: 处理昼夜切换与月相逻辑
- **AutomationSystem**: 驱动机器人的逻辑判定
- **CombatSystem**: 处理基于乘法公式的塔防战斗
- **EcologicalSystem**: 管理植物生长和领地扩张
- **ResourceManagementSystem**: 管理资源转换和存储

### 2.3 数值引擎设计
```rust
// 伤害计算公式
fn calculate_damage(
    base_damage: u128,
    equipment_bonus: u128,
    pet_resonance: Vec<f64>,
    territory_coefficient: f64,
    plant_count: u32
) -> u128 {
    let resonance_multiplier: f64 = pet_resonance.iter().sum();
    let territory_factor = territory_coefficient.powi(plant_count as i32);
    
    ((base_damage + equipment_bonus) as f64 
        * resonance_multiplier 
        * territory_factor) as u128
}

// 资源转换公式
fn convert_resources(
    input_resource: ResourceType,
    amount: f32,
    efficiency: f32,
) -> HashMap<ResourceType, f32> {
    let mut outputs = HashMap::new();
    match input_resource {
        ResourceType::Scrap => {
            outputs.insert(ResourceType::Metal, amount * 0.3 * efficiency);
            outputs.insert(ResourceType::Energy, amount * 0.2 * efficiency);
        }
        ResourceType::Metal => {
            outputs.insert(ResourceType::Shields, amount * 0.5 * efficiency);
        }
        ResourceType::Soil => {
            outputs.insert(ResourceType::Energy, amount * 0.1 * efficiency);
        }
    }
    outputs
}
```

## 3. 核心模块实现方案

### 3.1 自动化与生产模块

#### 机制设计
- 玩家通过UI设定"逻辑节点"进行可视化编程
- 机器人根据节点逻辑自动收割植物产出的能源
- 支持条件判断、循环和优先级设置

#### 技术实现
```rust
// 逻辑节点定义
enum LogicNode {
    Harvest { target: Entity },
    Store { source: Entity, target: Entity },
    Condition { check: Condition, true_branch: Vec<LogicNode> },
    Loop { condition: Condition, body: Vec<LogicNode> },
    Convert { from: ResourceType, to: ResourceType },
}

// 异步任务处理
async fn process_robot_tasks(
    robots: Query<&Robot>,
    plants: Query<&Plant>,
    mut energy_storages: Query<&mut EnergyStorage>
) {
    // 并行处理多个机器人的路径规划和任务执行
}
```

### 3.2 领地与植物扩张系统

#### 生态机制
- 植物通过"根系逻辑"破坏AI金属地板
- 采用Tilemap + 元胞自动机算法
- 白天系统启动时，植物根据阳光和能源值向外辐射生长

#### 技术实现
```rust
// 使用原生 Bevy Tilemap 和 Required Components
use bevy::sprite::Sprite;

// 元胞自动机状态组件
#[derive(Component, Clone, Copy, PartialEq)]
struct CellState {
    state_type: CellType,
    energy: f32,
    growth_rate: f32,
}

#[derive(Clone, Copy, PartialEq)]
enum CellType {
    Empty,
    MetalFloor,
    Plant,
    Root,
    Soil { quality: f32 },
}

// 使用 Required Components 创建瓦片实体
fn spawn_tile(mut commands: Commands, position: IVec2, state: CellState) {
    commands.spawn((
        state,
        Sprite {
            image: get_tile_image(&state.state_type),
            custom_size: Some(Vec2::new(32.0, 32.0)),
            ..default()
        },
        Transform::from_xyz(position.x as f32 * 32.0, position.y as f32 * 32.0, 0.0),
        GlobalTransform::default(),
    ));
}

// 生长规则 - 使用 ECS 查询而非直接操作网格
fn apply_growth_rules(
    mut plants: Query<(&mut CellState, &Transform)>,
    mut commands: Commands,
    time: Res<Time>
) {
    for (mut cell_state, transform) in &mut plants {
        if cell_state.state_type == CellType::Plant && cell_state.energy > 0.0 {
            // 向相邻格子扩散新植物
            spread_to_neighbors(&mut commands, transform.translation, cell_state.growth_rate * time.delta_secs());
        }
    }
}
```

### 3.3 双人协同与灵宠系统

#### 协作机制
- 两人无固定职业，通过交换灵宠与装备改变属性载体
- 灵宠可装备各种组件，具有随机肉鸽属性
- 支持实时P2P网络同步

#### 灵宠系统设计
```rust
// 灵宠实体组件
struct Pet {
    name: String,
    level: u32,
    equipment_slots: Vec<EquipmentSlot>,
    base_stats: Stats,
    resource_affinity: ResourceType, // 资源亲和性
}

struct Equipment {
    slot_type: EquipmentSlot,
    stats: Stats,
    special_effects: Vec<Effect>,
    resource_bonus: HashMap<ResourceType, f32>,
}

// 装备共鸣系统
fn calculate_resonance_bonus(pets: &[&Pet]) -> f64 {
    let mut bonus = 1.0;
    for combo in pets.windows(2) {
        if has_synergy(combo[0], combo[1]) {
            bonus *= 1.25;
        }
    }
    bonus
}
```

## 4. 15日月相算法

### 4.1 月相周期设计
```rust
const MOON_PHASES: [MoonPhase; 15] = [
    MoonPhase { name: "新月", scan_intensity: 0.1, resource_multiplier: 2.0 },
    MoonPhase { name: "峨眉月", scan_intensity: 0.2, resource_multiplier: 1.8 },
    // ... 其他月相
    MoonPhase { name: "朔月", scan_intensity: 0.5, resource_multiplier: 0.5 },
];

struct MoonPhase {
    name: &'static str,
    scan_intensity: f32,      // AI扫描强度
    resource_multiplier: f32, // 资源产出系数
}
```

### 4.2 视觉实现
- 使用Bevy Shader实现从月圆到月缺的平滑过渡
- 月圆: 高亮度、暖色调
- 月缺: 极暗、冷色调、惊悚氛围

```rust
// 月相着色器
fn moon_phase_shader(phase: f32, time: f32) -> Color {
    let brightness = (phase * PI).sin();
    let temperature = lerp(3000.0, 8000.0, phase); // 暖色到冷色
    
    Color::hsl(
        240.0 - temperature * 0.01, // 色相
        0.8,                        // 饱和度
        brightness * 0.8,           // 亮度
    )
}
```

## 5. 开发路线图

### 5.1 原型阶段 (第1-2月)
- [x] 搭建Bevy 0.18.1基础ECS框架（使用 Required Components）
- [x] 实现基础机器人自动采集逻辑
- [x] 完成能源存取系统原型
- [x] 验证核心游戏循环
- [x] 实现基础资源系统（金属、土壤、能源、护盾）
- [x] 迁移至原生 Sprite 和 Tilemap 渲染（移除 SpriteBundle 依赖）

### 5.2 数值跑通 (第3月)
- [ ] 实现乘法倍率系统
- [ ] 测试数值从10膨胀到1,000,000的稳定性
- [ ] 优化大数运算性能
- [ ] 平衡各系统数值关系
- [ ] 完善资源转换平衡性

### 5.3 美术与氛围 (第4-5月)
- [ ] AI生成2D像素资产
- [ ] 实现昼夜视觉反差
- [ ] 打磨UI/UX体验
- [ ] 添加音效和背景音乐
- [ ] 资源图标和特效设计

### 5.4 联机测试 (第6月)
- [ ] 完善P2P网络同步
- [ ] 测试双人协作稳定性
- [ ] 优化网络延迟和丢包处理
- [ ] Steam Demo发布准备
- [ ] 资源系统压力测试

## 6. 技术优势

### 6.1 AI友好性
- ECS架构的组件化设计便于AI生成代码
- 数据驱动的开发模式降低复杂度
- Rust的类型系统提供编译时安全保障

### 6.2 性能优势
- Rust的零成本抽象确保高性能
- Bevy的并行系统处理大量实体
- 内存安全避免运行时崩溃

### 6.3 扩展性
- 模块化设计支持后续功能扩展
- ECS架构便于添加新实体和组件
- 网络架构支持更多玩家接入
- 资源系统支持新资源类型添加

## 7. 风险评估与应对

### 7.1 技术风险
- **Rust学习曲线**: 提供详细文档和示例代码
- **Bevy生态成熟度**: 积极参与社区，贡献代码
- **网络同步复杂性**: 采用成熟的P2P库
- **资源平衡难度**: 建立完善的测试体系

### 7.2 市场风险
- **竞争激烈**: 突出自动化+协作的差异化
- **用户获取**: 通过Demo和早期测试建立社区
- **定价策略**: 根据市场反馈灵活调整

## 8. 总结

《黑暗森林》通过结合Rust的性能优势、Bevy的ECS架构，以及独特的月相循环和数值膨胀机制，有望在Steam平台获得良好的市场表现。项目的成功关键在于：

1. 扎实的数值系统设计
2. 流畅的自动化玩法
3. 出色的双人协作体验
4. 引人入胜的视觉氛围
5. 丰富的资源管理机制

通过分阶段的开发路线图和严格的质量控制，我们有信心实现Steam Top 80-100的目标。新增的资源系统为游戏提供了更深层的策略性和可玩性，让玩家在废土重建过程中体验更多样化的游戏机制。