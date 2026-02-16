# 生态循环设计文档

## 1. 核心概念（废土到绿洲）

本系统实现了一个完整的生态循环机制，将废弃的废土世界转化为充满生机的绿洲。玩家通过收集废料、转换能源、种植植物、收获资源的循环过程，逐步改善和美化游戏环境。

核心设计理念：
- **可持续性**：资源可以循环利用，没有绝对的废弃物
- **渐进式**：环境改善是一个逐步积累的过程
- **互动性**：每个环节都与玩家行为紧密相关
- **视觉反馈**：环境变化通过视觉效果直观呈现

## 2. 资源循环阶段

### 2.1 废料收集阶段
- 玩家在世界中探索和收集各种废料资源
- 废料类型：金属碎片、塑料、电子元件等
- 收集方式：直接拾取、工具采集

### 2.2 能源转换阶段
- 将废料投入转换装置，生成能源
- 转换效率取决于废料类型和数量
- 产出：能量点数、特殊能源物质

### 2.3 植物种植阶段
- 使用能源购买或激活植物种子
- 选择合适的地形进行种植
- 植物类型：草本植物、灌木、树木等

### 2.4 生长收获阶段
- 植物随时间自然生长
- 成长过程中产生积极的环境影响
- 收获时获得资源回报和环境美化

## 3. 实体定义

### 3.1 基础资源系统

#### 废料（Scrap）
```rust
struct Scrap {
    scrap_type: ScrapType,    // 废料类型
    quality: f32,             // 品质等级 (0.0 - 1.0)
    quantity: u32,            // 数量
    position: Vec3,           // 世界位置
}

enum ScrapType {
    Metal,      // 金属碎片
    Plastic,    // 塑料
    Electronic, // 电子元件
    Organic,    // 有机废料
}
```

#### 金属资源（Metal）
```rust
struct Metal {
    purity: f32,              // 纯度等级 (0.0 - 1.0)
    alloy_type: AlloyType,    // 合金类型
    quantity: u32,            // 数量
}

enum AlloyType {
    Steel,       // 钢材
    Aluminum,    // 铝材
    Titanium,    // 钛合金
    Copper,      // 铜材
}
```

#### 土壤资源（Soil）
```rust
struct Soil {
    fertility: f32,           // 肥沃度 (0.0 - 1.0)
    ph_level: f32,            // pH值 (0.0 - 14.0)
    moisture: f32,            // 湿度 (0.0 - 1.0)
    contamination: f32,       // 污染度 (0.0 - 1.0)
}
```

#### 能源资源（Energy）
```rust
struct Energy {
    energy_type: EnergyType,  // 能源类型
    amount: f32,              // 能量数量
    efficiency: f32,          // 转换效率
}

enum EnergyType {
    Solar,      // 太阳能
    BioMass,    // 生物质能
    Thermal,    // 热能
    Electric,   // 电能
}
```

#### 护盾资源（Shields）
```rust
struct Shield {
    shield_type: ShieldType,  // 护盾类型
    strength: f32,            // 护盾强度
    regeneration_rate: f32, // 再生速率
}

enum ShieldType {
    Energy,     // 能量护盾
    Biological, // 生物护盾
    Metallic,   // 金属护盾
    Composite,  // 复合护盾
}
```

### 3.2 能源花（EnergyFlower）
```rust
struct EnergyFlower {
    energy_output: f32,       // 能源产出速率
    efficiency: f32,          // 转换效率
    bloom_state: f32,         // 开花状态 (0.0 - 1.0)
    lifespan: f32,            // 生命周期
}
```

### 3.3 植物（Plant）
```rust
struct Plant {
    plant_type: PlantType,    // 植物类型
    growth_stage: u8,         // 生长阶段 (0-5)
    health: f32,              // 健康度 (0.0 - 1.0)
    maturity: f32,            // 成熟度 (0.0 - 1.0)
    water_level: f32,         // 水分等级
    nutrient_level: f32,      // 营养等级
}

enum PlantType {
    Grass,      // 草
    Bush,       // 灌木
    Tree,       // 树木
    Flower,     // 花朵
}
```

## 4. 系统逻辑

### 4.1 采集系统
- **废料检测**：扫描周围环境中的废料
- **采集动画**：播放相应的采集动画
- **背包管理**：将废料添加到玩家背包
- **刷新机制**：废料在一定时间后重新生成

### 4.2 种植系统
- **地形检测**：判断当前位置是否适合种植
- **能源消耗**：根据植物类型消耗相应能源
- **种植动画**：播放种植动作和特效
- **初始化设置**：设置植物的初始状态参数

### 4.3 生长系统
- **时间驱动**：基于游戏时间的生长更新
- **环境因子**：光照、水分、营养影响生长速度
- **阶段转换**：达到条件时自动进入下一阶段
- **视觉效果**：模型大小、颜色、特效的渐变

### 4.4 收获系统
- **成熟度检测**：判断植物是否达到可收获状态
- **收获奖励**：根据植物品质给予相应回报
- **资源产出**：获得种子、能源、美化点数等
- **环境改善**：提升区域的环境质量等级

### 4.5 资源转换系统
- **废料处理**：将废料转换为可用资源
- **金属精炼**：提高金属纯度和质量
- **土壤修复**：改善土壤肥力和质量
- **能源合成**：组合不同类型能源获得高效能源
- **护盾生成**：消耗资源生成保护性护盾

## 5. Bevy 实现细节

### 5.1 组件定义
```rust
#[derive(Component)]
struct Collectible {
    value: u32,
    collector_type: CollectorType,
}

#[derive(Component)]
struct EnergyConverter {
    conversion_rate: f32,
    active: bool,
}

#[derive(Component)]
struct Growable {
    base_growth_rate: f32,
    current_stage: u8,
    max_stages: u8,
    growth_progress: f32,
}

#[derive(Component)]
struct Harvestable {
    min_maturity: f32,
    reward_multiplier: f32,
    harvest_cooldown: Timer,
}

#[derive(Component)]
struct ResourceStorage {
    metal: u32,
    soil: u32,
    energy: f32,
    shields: f32,
}
```

### 5.2 系统实现
```rust
// 采集系统
fn collection_system(
    mut commands: Commands,
    mut player_query: Query<(&Transform, &mut Inventory), With<Player>>,
    collectible_query: Query<(Entity, &Transform, &Collectible)>,
    input: Res<Input<KeyCode>>,
) {
    if input.just_pressed(KeyCode::E) {
        for (player_transform, mut inventory) in player_query.iter_mut() {
            for (entity, transform, collectible) in collectible_query.iter() {
                let distance = player_transform.translation.distance(transform.translation);
                if distance < COLLECTION_RANGE {
                    inventory.add_item(collectible.collector_type, collectible.value);
                    commands.entity(entity).despawn();
                }
            }
        }
    }
}

// 生长系统
fn growth_system(
    time: Res<Time>,
    mut growable_query: Query<(&mut Growable, &mut Transform)>,
    environment_query: Query<&EnvironmentQuality>,
) {
    for (mut growable, mut transform) in growable_query.iter_mut() {
        let growth_modifier = calculate_environment_bonus(&environment_query);
        growable.growth_progress += growable.base_growth_rate * time.delta_seconds() * growth_modifier;
        
        if growable.growth_progress >= 1.0 {
            growable.current_stage += 1;
            growable.growth_progress = 0.0;
            update_visual_scale(&mut transform, growable.current_stage);
        }
    }
}

// 转换系统
fn energy_conversion_system(
    mut converter_query: Query<(&mut EnergyConverter, &mut Inventory)>,
    mut energy_pool: ResMut<EnergyPool>,
) {
    for (mut converter, mut inventory) in converter_query.iter_mut() {
        if converter.active {
            let scrap_amount = inventory.get_item_amount(CollectorType::Scrap);
            let converted_energy = scrap_amount as f32 * converter.conversion_rate;
            
            energy_pool.add_energy(converted_energy);
            inventory.remove_item(CollectorType::Scrap, scrap_amount);
        }
    }
}

// 资源管理系统
fn resource_management_system(
    mut storage_query: Query<&mut ResourceStorage>,
    time: Res<Time>,
) {
    for mut storage in storage_query.iter_mut() {
        // 护盾自动回复
        if storage.shields < 100.0 {
            storage.shields += 5.0 * time.delta_seconds();
            storage.shields = storage.shields.min(100.0);
        }
        
        // 能源消耗逻辑
        if storage.energy > 0.0 {
            storage.energy -= 0.1 * time.delta_seconds();
            storage.energy = storage.energy.max(0.0);
        }
    }
}
```

### 5.3 事件系统
```rust
#[derive(Event)]
struct ItemCollected {
    collector_type: CollectorType,
    amount: u32,
    position: Vec3,
}

#[derive(Event)]
struct PlantGrown {
    entity: Entity,
    new_stage: u8,
    plant_type: PlantType,
}

#[derive(Event)]
struct EnvironmentImproved {
    area: Rect,
    improvement_amount: f32,
}

#[derive(Event)]
struct ResourceConverted {
    from_type: ResourceType,
    to_type: ResourceType,
    amount: f32,
    efficiency: f32,
}
```

### 5.4 资源管理
```rust
#[derive(Resource)]
struct GameConfig {
    collection_range: f32,
    base_growth_rate: f32,
    conversion_efficiency: f32,
    environment_influence_radius: f32,
    shield_regeneration_rate: f32,
    energy_consumption_rate: f32,
}

#[derive(Resource, Default)]
struct GlobalState {
    total_scrap_collected: u32,
    total_energy_generated: f32,
    plants_grown: u32,
    environment_score: f32,
    metal_reserves: u32,
    soil_quality: f32,
    shield_capacity: f32,
}
```

这个生态循环系统通过清晰的阶段划分和实体定义，为游戏提供了丰富的互动体验和可持续的游戏机制。玩家可以通过不断的循环操作，逐步改善游戏世界的环境质量，获得成就感和视觉满足感。新增的资源系统（金属、土壤、能源、护盾）为游戏提供了更多策略深度和玩法多样性。