# 装备和武器系统

## 系统概述
装备和武器系统提供了丰富的装备类型、稀有度机制和升级系统，让玩家通过收集和升级装备来提升角色能力。

---

## 1. 装备类型

### 1.1 实现文件
- **组件**：`src/components/equipment.rs` (Equipment, EquipmentType, EquipmentRarity, EquipmentBar)
- **系统**：`src/systems/equipment.rs` (spawn_random_equipment, pickup_equipment, upgrade_equipment)

### 1.2 装备分类

#### 武器类

| 名称 | 伤害 | 攻速 | 暴击率 | 暴击倍率 | 特殊效果 |
|------|------|------|--------|----------|----------|
| 激光枪 | 10 | 1.0 | 5% | 1.5x | 基础武器 |
| 等离子炮 | 25 | 0.6 | 10% | 2.0x | 高伤害 |
| EMP爆破枪 | 15 | 0.8 | 15% | 1.8x | 对机械有效 |
| 轨道炮 | 40 | 0.4 | 20% | 2.5x | 极高伤害 |

#### 护甲类

| 名称 | 防御 | 能量加成 | 特殊效果 |
|------|------|----------|----------|
| 轻型护甲 | 15 | 0 | 基础防御 |
| 重型护甲 | 30 | 0 | 高防御 |
| 能量护盾 | 20 | 10 | 防御+能量 |

#### 饰品类

| 名称 | 能量加成 | 暴击率 | 特殊效果 |
|------|----------|--------|----------|
| 太阳能板 | 5 | 0% | 能量恢复 |
| 电池包 | 50 | 0% | 能量上限 |
| 瞄准系统 | 0 | 15% | 暴击提升 |

### 1.3 装备属性

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

---

## 2. 稀有度系统

### 2.1 稀有度类型

| 稀有度 | 颜色 | 倍率 | 出现概率 |
|--------|------|------|----------|
| Common | 白色 | 1.0x | 60% |
| Uncommon | 绿色 | 1.2x | 25% |
| Rare | 蓝色 | 1.5x | 10% |
| Legendary | 紫色 | 2.0x | 4% |
| Mythic | 金色 | 3.0x | 1% |

### 2.2 稀有度影响

#### 属性计算
```rust
let base_stats = equipment_type.base_stats();
let rarity_multiplier = rarity.multiplier();

final_stats = EquipmentStats {
    damage: base_stats.damage * rarity_multiplier,
    attack_speed: base_stats.attack_speed,
    defense: base_stats.defense * rarity_multiplier,
    energy_bonus: base_stats.energy_bonus * rarity_multiplier,
    crit_chance: base_stats.crit_chance * rarity_multiplier,
    crit_multiplier: base_stats.crit_multiplier,
};
```

#### 视觉表现
- 不同稀有度显示不同颜色
- 稀有度越高，颜色越鲜艳
- 神话装备有特殊光效（待实现）

---

## 3. 装备栏系统

### 3.1 装备槽位

```rust
pub struct EquipmentBar {
    pub weapon: Option<Entity>,     // 武器槽
    pub armor: Option<Entity>,      // 护甲槽
    pub accessory: Option<Entity>,  // 饰品槽
}
```

### 3.2 装备规则
- 每个槽位只能装备一件装备
- 同类型装备会自动替换
- 替换时旧装备被销毁

### 3.3 属性计算
```rust
pub fn total_stats(&self, equipment_query: &Query<&Equipment>) -> EquipmentStats {
    let mut total = EquipmentStats::default();

    for &entity in &[weapon, armor, accessory] {
        if let Some(e) = entity {
            if let Ok(equipment) = equipment_query.get(e) {
                total.damage += equipment.stats.damage;
                total.attack_speed += equipment.stats.attack_speed;
                total.defense += equipment.stats.defense;
                total.energy_bonus += equipment.stats.energy_bonus;
                total.crit_chance += equipment.stats.crit_chance;
                total.crit_multiplier = total.crit_multiplier.max(equipment.stats.crit_multiplier);
            }
        }
    }

    total
}
```

---

## 4. 生成系统

### 4.1 生成方式
- **操作**：键盘 E
- **位置**：玩家当前位置
- **生成条件**：无限制

### 4.2 生成逻辑

#### 类型选择
```rust
let equipment_types = [
    LaserGun, PlasmaCannon, EMPBlaster, Railgun,
    LightArmor, HeavyArmor, EnergyShield,
    SolarPanel, BatteryPack, TargetingSystem,
];
let equipment_type = equipment_types[random_index];
```

#### 稀有度确定
```rust
let rarity_roll = random(0..100);
let rarity = match rarity_roll {
    0..=60 => Common,      // 60%
    61..=85 => Uncommon,    // 25%
    86..=95 => Rare,        // 10%
    96..=99 => Legendary,   // 4%
    _ => Mythic,            // 1%
};
```

### 4.3 生成参数
- **大小**：20x20像素
- **层级**：Z=1
- **颜色**：根据稀有度
- **初始等级**：1

---

## 5. 拾取系统

### 5.1 拾取方式
- **操作**：键盘 F
- **范围**：50像素
- **条件**：装备在拾取范围内

### 5.2 拾取逻辑
```rust
if distance < pickup_range {
    let slot = equipment.equipment_type.slot();

    // 卸下旧装备
    if let Some(old_equipment) = equipment_bar.unequip(slot) {
        commands.entity(old_equipment).despawn();
    }

    // 装备新物品
    equipment_bar.equip(slot, entity);
}
```

### 5.3 自动装备
- 根据装备类型自动选择槽位
- 槽位已满时自动替换
- 显示装备信息

---

## 6. 升级系统

### 6.1 升级方式
- **操作**：键盘 U
- **成本**：50能源/次
- **等级上限**：10级

### 6.2 升级计算

#### 属性提升
```rust
let upgrade_multiplier = 1.0 + (level as f32 * 0.1);

let base_stats = equipment_type.base_stats();
let rarity_multiplier = rarity.multiplier();

new_stats = EquipmentStats {
    damage: base_damage * rarity_multiplier * upgrade_multiplier,
    attack_speed: base_attack_speed,
    defense: base_defense * rarity_multiplier * upgrade_multiplier,
    energy_bonus: base_energy * rarity_multiplier * upgrade_multiplier,
    crit_chance: base_crit * rarity_multiplier,
    crit_multiplier: base_crit_multiplier,
};
```

#### 升级效果
- **伤害/防御/能量**：每级提升10%
- **攻击速度**：不变
- **暴击率**：不变
- **暴击倍率**：不变

### 6.3 升级限制
- **等级限制**：最高10级
- **能源限制**：需要50能源
- **装备限制**：只能升级已装备的装备

---

## 7. 战斗系统（待实现）

### 7.1 伤害计算
```rust
// 基础伤害
let base_damage = weapon_stats.damage;

// 暴击判定
if random() < crit_chance {
    damage = base_damage * crit_multiplier;
} else {
    damage = base_damage;
}

// 防御减免
let final_damage = damage * (1.0 - defense / (defense + 100));
```

### 7.2 攻击速度
```rust
let attack_interval = 1.0 / attack_speed;
let can_attack = time_since_last_attack >= attack_interval;
```

### 7.3 特殊效果
- **EMP爆破枪**：对机械敌人造成额外伤害
- **等离子炮**：穿透效果（待实现）
- **轨道炮**：范围伤害（待实现）

---

## 8. 系统集成

### 8.1 初始化
- 玩家生成时初始化装备栏
- 装备栏默认为空

### 8.2 更新顺序
```rust
.add_systems(Update, (
    systems::equipment::spawn_random_equipment,
    systems::equipment::pickup_equipment,
    systems::equipment::upgrade_equipment,
    systems::equipment::display_equipment_info,
).run_if(in_state(GameState::InGame)))
```

### 8.3 依赖关系
- 依赖玩家系统（获取玩家位置）
- 依赖资源系统（消耗能源）
- 依赖UI系统（显示装备信息）

---

## 9. 性能优化

### 9.1 查询优化
- 缓存装备查询结果
- 避免重复计算总属性
- 使用组件过滤器

### 9.2 生成优化
- 限制装备数量
- 使用对象池
- 延迟生成

### 9.3 升级优化
- 批量升级（待实现）
- 升级预览
- 快速升级界面

---

## 10. 待实现功能

### 10.1 装备特性
- [ ] 套装效果
- [ ] 附魔系统
- [ ] 装备强化
- [ ] 装备重铸

### 10.2 交互增强
- [ ] 装备对比
- [ ] 装备预览
- [ ] 装备交易
- [ ] 装备分解

### 10.3 UI优化
- [ ] 装备栏界面
- [ ] 装备详情面板
- [ ] 装备筛选
- [ ] 装备排序

---

## 11. 已知问题

### 11.1 功能问题
- 装备无法卸下保存
- 装备替换无确认
- 缺少装备对比功能

### 11.2 平衡问题
- 稀有度差距过大
- 升级成本固定
- 缺少装备平衡调整

### 11.3 UI问题
- 装备信息显示简陋
- 缺少装备栏UI
- 升级无预览

---

## 12. 更新日志

### v0.1.0 (当前版本)
- ✅ 实现10种装备类型
- ✅ 实现5种稀有度
- ✅ 实现装备生成系统
- ✅ 实现装备拾取系统
- ✅ 实现装备升级系统
- ✅ 实现装备栏系统
- ✅ 实现属性计算系统
