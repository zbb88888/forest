# 植物系统

## 系统概述
植物系统实现了植物的种植、生长、收获完整生命周期，是游戏资源获取的核心机制。

---

## 1. 植物类型

### 1.1 实现文件
- **组件**：`src/components/plant.rs` (Plant, PlantType, Growable)
- **系统**：`src/systems/plant.rs` (plant_seed, grow_plants, harvest_plants, plant_decay)

### 1.2 植物类型表

| 类型 | 颜色 | 生长速度 | 能源产出 | 基础奖励 | 特殊要求 |
|------|------|----------|----------|----------|----------|
| Grass | 绿色 | 1.0x | 1.0x | 10 | 草地 |
| Bush | 深绿色 | 0.8x | 1.5x | 20 | 森林 |
| Tree | 深绿色 | 0.5x | 2.0x | 50 | 森林 |
| Flower | 粉色 | 0.7x | 1.3x | 15 | 草地/森林 |
| EnergyFlower | 青色 | 0.6x | 3.0x | 30 | 黑暗森林 |

### 1.3 植物属性

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

---

## 2. 种植系统

### 2.1 种植机制
- **操作方式**：右键点击地图
- **种植范围**：点击位置周围32像素
- **地形要求**：
  - 草地：可种植草、花朵
  - 森林：可种植灌木、树木
  - 黑暗森林：可种植能源花
  - 其他地形：不可种植

### 2.2 自动选择规则
```rust
match tile_type {
    TileType::Grass => PlantType::Grass,
    TileType::Forest => PlantType::Bush,
    TileType::DarkForest => PlantType::EnergyFlower,
    _ => PlantType::Grass,
}
```

### 2.3 种植参数
- **瓦片大小**：32x32像素
- **植物大小**：初始为瓦片的60%
- **初始状态**：
  - 生长阶段：0
  - 健康度：1.0
  - 成熟度：0.0
  - 水分：0.5
  - 营养：0.5

---

## 3. 生长系统

### 3.1 生长阶段
- **阶段数量**：5个（0-4）
- **阶段转换**：当growth_progress >= 1.0时进入下一阶段
- **视觉反馈**：随生长阶段改变大小
  - 阶段0：60% 瓦片大小
  - 阶段1-4：线性增长到100%

### 3.2 生长速度计算
```rust
let day_multiplier = match game_time.current_phase {
    DayPhase::Day => 1.5,
    DayPhase::Dawn | DayPhase::Dusk => 1.0,
    DayPhase::Night => 0.5,
};

let growth_rate = base_growth_rate * day_multiplier * plant.health;
```

### 3.3 昼夜影响

| 昼夜阶段 | 生长速度 | 说明 |
|----------|----------|------|
| Dawn | 1.0x | 正常生长 |
| Day | 1.5x | 快速生长 |
| Dusk | 1.0x | 正常生长 |
| Night | 0.5x | 缓慢生长 |

### 3.4 资源消耗
- **水分消耗**：0.01/秒
- **营养消耗**：0.01/秒
- **健康衰减**：0.001/秒

### 3.5 健康度影响
- 健康度 <= 0：植物死亡
- 水分 <= 0：停止生长
- 营养 <= 0：停止生长

---

## 4. 收获系统

### 4.1 收获条件
- **生长阶段**：>= 5
- **成熟度**：>= 1.0
- **操作方式**：左键点击植物
- **点击范围**：32像素

### 4.2 收获奖励计算
```rust
let base_reward = match plant_type {
    PlantType::Grass => 10,
    PlantType::Bush => 20,
    PlantType::Tree => 50,
    PlantType::Flower => 15,
    PlantType::EnergyFlower => 30,
};

let multiplier = plant.health * plant.maturity;
let final_reward = (base_reward as f32 * multiplier) as u32;
```

### 4.3 收获效果
- 植物被移除
- 能源加入玩家背包
- 显示收获信息

---

## 5. 植物与环境的交互

### 5.1 地形影响
- **能源产出倍率**：不同地形有不同的能源产出倍率
  - 草地：1.0x
  - 森林：1.5x
  - 黑暗森林：2.0x
- **种植限制**：只能在特定地形种植
- **生长速度**：受地形影响（待实现）

### 5.2 时间影响
- **昼夜循环**：白天生长更快
- **月相影响**：满月资源产出加倍
- **季节变化**：待实现

### 5.3 机器人交互
- 机器人自动采集成熟植物
- 采集后植物被移除
- 资源加入机器人背包

---

## 6. 系统集成

### 6.1 初始化
- 无需特殊初始化
- 植物通过玩家操作动态生成

### 6.2 更新顺序
```rust
.add_systems(Update, (
    systems::plant::plant_seed,
    systems::plant::grow_plants,
    systems::plant::harvest_plants,
    systems::plant::plant_decay,
).run_if(in_state(GameState::InGame)))
```

### 6.3 依赖关系
- 依赖时间系统（获取昼夜阶段）
- 依赖地图系统（获取地形类型）
- 依赖玩家系统（获取玩家位置）

---

## 7. 性能优化

### 7.1 查询优化
- 使用组件过滤器
- 避免不必要的查询
- 缓存常用查询结果

### 7.2 更新优化
- 仅更新可见植物
- 批量更新相同类型植物
- 使用插值平滑动画

### 7.3 内存优化
- 对象池管理植物实体
- 预分配植物数组
- 避免频繁创建/销毁

---

## 8. 待实现功能

### 8.1 植物特性
- [ ] 植物疾病系统
- [ ] 植物杂交机制
- [ ] 植物进化系统
- [ ] 特殊植物品种

### 8.2 环境交互
- [ ] 天气系统影响
- [ ] 季节变化
- [ ] 灾害事件
- [ ] 生态系统平衡

### 8.3 视觉效果
- [ ] 生长动画
- [ ] 收获特效
- [ ] 植物摇摆动画
- [ ] 粒子效果

---

## 9. 已知问题

### 9.1 功能问题
- 植物健康度无法恢复
- 水分和营养无法补充
- 植物死亡后无提示

### 9.2 性能问题
- 大量植物时帧率下降
- 收获判定范围过大
- 生长计算频繁

### 9.3 视觉问题
- 植物外观单一
- 缺少生长动画
- 收获无特效

---

## 10. 更新日志

### v0.1.0 (当前版本)
- ✅ 实现5种植物类型
- ✅ 实现种植机制
- ✅ 实现生长系统（受昼夜影响）
- ✅ 实现收获系统
- ✅ 实现健康和资源系统
- ✅ 实现植物与地形交互
