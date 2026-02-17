use bevy::prelude::*;

/// 装备槽位类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EquipmentSlot {
    Weapon,     // 武器
    Armor,      // 护甲
    Accessory,  // 饰品
}

/// 装备类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EquipmentType {
    // 武器
    LaserGun,       // 激光枪
    PlasmaCannon,   // 等离子炮
    EMPBlaster,     // EMP 爆破枪
    Railgun,        // 轨道炮

    // 护甲
    LightArmor,     // 轻型护甲
    HeavyArmor,     // 重型护甲
    EnergyShield,    // 能量护盾

    // 饰品
    SolarPanel,     // 太阳能板（增加能量恢复）
    BatteryPack,    // 电池包（增加能量上限）
    TargetingSystem,// 瞄准系统（增加暴击率）
}

impl EquipmentType {
    /// 获取装备的名称
    pub fn name(&self) -> &str {
        match self {
            EquipmentType::LaserGun => "激光枪",
            EquipmentType::PlasmaCannon => "等离子炮",
            EquipmentType::EMPBlaster => "EMP爆破枪",
            EquipmentType::Railgun => "轨道炮",
            EquipmentType::LightArmor => "轻型护甲",
            EquipmentType::HeavyArmor => "重型护甲",
            EquipmentType::EnergyShield => "能量护盾",
            EquipmentType::SolarPanel => "太阳能板",
            EquipmentType::BatteryPack => "电池包",
            EquipmentType::TargetingSystem => "瞄准系统",
        }
    }

    /// 获取装备的槽位
    pub fn slot(&self) -> EquipmentSlot {
        match self {
            EquipmentType::LaserGun | EquipmentType::PlasmaCannon | 
            EquipmentType::EMPBlaster | EquipmentType::Railgun => EquipmentSlot::Weapon,
            EquipmentType::LightArmor | EquipmentType::HeavyArmor | 
            EquipmentType::EnergyShield => EquipmentSlot::Armor,
            EquipmentType::SolarPanel | EquipmentType::BatteryPack | 
            EquipmentType::TargetingSystem => EquipmentSlot::Accessory,
        }
    }

    /// 获取装备的基础属性
    pub fn base_stats(&self) -> EquipmentStats {
        match self {
            EquipmentType::LaserGun => EquipmentStats {
                damage: 10.0,
                attack_speed: 1.0,
                defense: 0.0,
                energy_bonus: 0.0,
                crit_chance: 0.05,
                crit_multiplier: 1.5,
            },
            EquipmentType::PlasmaCannon => EquipmentStats {
                damage: 25.0,
                attack_speed: 0.6,
                defense: 0.0,
                energy_bonus: 0.0,
                crit_chance: 0.1,
                crit_multiplier: 2.0,
            },
            EquipmentType::EMPBlaster => EquipmentStats {
                damage: 15.0,
                attack_speed: 0.8,
                defense: 0.0,
                energy_bonus: 0.0,
                crit_chance: 0.15,
                crit_multiplier: 1.8,
            },
            EquipmentType::Railgun => EquipmentStats {
                damage: 40.0,
                attack_speed: 0.4,
                defense: 0.0,
                energy_bonus: 0.0,
                crit_chance: 0.2,
                crit_multiplier: 2.5,
            },
            EquipmentType::LightArmor => EquipmentStats {
                damage: 0.0,
                attack_speed: 0.0,
                defense: 15.0,
                energy_bonus: 0.0,
                crit_chance: 0.0,
                crit_multiplier: 1.0,
            },
            EquipmentType::HeavyArmor => EquipmentStats {
                damage: 0.0,
                attack_speed: 0.0,
                defense: 30.0,
                energy_bonus: 0.0,
                crit_chance: 0.0,
                crit_multiplier: 1.0,
            },
            EquipmentType::EnergyShield => EquipmentStats {
                damage: 0.0,
                attack_speed: 0.0,
                defense: 20.0,
                energy_bonus: 10.0,
                crit_chance: 0.0,
                crit_multiplier: 1.0,
            },
            EquipmentType::SolarPanel => EquipmentStats {
                damage: 0.0,
                attack_speed: 0.0,
                defense: 0.0,
                energy_bonus: 5.0,
                crit_chance: 0.0,
                crit_multiplier: 1.0,
            },
            EquipmentType::BatteryPack => EquipmentStats {
                damage: 0.0,
                attack_speed: 0.0,
                defense: 0.0,
                energy_bonus: 50.0,
                crit_chance: 0.0,
                crit_multiplier: 1.0,
            },
            EquipmentType::TargetingSystem => EquipmentStats {
                damage: 0.0,
                attack_speed: 0.0,
                defense: 0.0,
                energy_bonus: 0.0,
                crit_chance: 0.15,
                crit_multiplier: 1.0,
            },
        }
    }
}

/// 装备属性
#[derive(Debug, Clone, Copy)]
pub struct EquipmentStats {
    pub damage: f32,          // 伤害
    pub attack_speed: f32,    // 攻击速度
    pub defense: f32,        // 防御力
    pub energy_bonus: f32,    // 能量加成
    pub crit_chance: f32,     // 暴击率
    pub crit_multiplier: f32,  // 暴击倍率
}

impl Default for EquipmentStats {
    fn default() -> Self {
        Self {
            damage: 0.0,
            attack_speed: 0.0,
            defense: 0.0,
            energy_bonus: 0.0,
            crit_chance: 0.0,
            crit_multiplier: 1.0,
        }
    }
}

/// 装备组件
#[derive(Component, Clone, Debug)]
pub struct Equipment {
    pub equipment_type: EquipmentType,
    pub level: u32,
    pub stats: EquipmentStats,
    pub rarity: EquipmentRarity,
}

impl Equipment {
    pub fn new(equipment_type: EquipmentType, rarity: EquipmentRarity) -> Self {
        let base_stats = equipment_type.base_stats();
        let multiplier = rarity.multiplier();

        Self {
            equipment_type,
            level: 1,
            stats: EquipmentStats {
                damage: base_stats.damage * multiplier,
                attack_speed: base_stats.attack_speed,
                defense: base_stats.defense * multiplier,
                energy_bonus: base_stats.energy_bonus * multiplier,
                crit_chance: base_stats.crit_chance * multiplier,
                crit_multiplier: base_stats.crit_multiplier,
            },
            rarity,
        }
    }

    /// 升级装备
    pub fn upgrade(&mut self) {
        self.level += 1;
        let upgrade_multiplier = 1.0 + (self.level as f32 * 0.1);

        let base_stats = self.equipment_type.base_stats();
        let rarity_multiplier = self.rarity.multiplier();

        self.stats = EquipmentStats {
            damage: base_stats.damage * rarity_multiplier * upgrade_multiplier,
            attack_speed: base_stats.attack_speed,
            defense: base_stats.defense * rarity_multiplier * upgrade_multiplier,
            energy_bonus: base_stats.energy_bonus * rarity_multiplier * upgrade_multiplier,
            crit_chance: base_stats.crit_chance * rarity_multiplier,
            crit_multiplier: base_stats.crit_multiplier,
        };
    }
}

/// 装备稀有度
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EquipmentRarity {
    Common,      // 普通（白色）
    Uncommon,    // 稀有（绿色）
    Rare,        // 史诗（蓝色）
    Legendary,   // 传说（紫色）
    Mythic,      // 神话（金色）
}

impl EquipmentRarity {
    /// 获取稀有度的倍率
    pub fn multiplier(&self) -> f32 {
        match self {
            EquipmentRarity::Common => 1.0,
            EquipmentRarity::Uncommon => 1.2,
            EquipmentRarity::Rare => 1.5,
            EquipmentRarity::Legendary => 2.0,
            EquipmentRarity::Mythic => 3.0,
        }
    }

    /// 获取稀有度的颜色
    pub fn color(&self) -> Color {
        match self {
            EquipmentRarity::Common => Color::srgb(0.9, 0.9, 0.9),
            EquipmentRarity::Uncommon => Color::srgb(0.3, 0.9, 0.3),
            EquipmentRarity::Rare => Color::srgb(0.3, 0.3, 0.9),
            EquipmentRarity::Legendary => Color::srgb(0.6, 0.3, 0.9),
            EquipmentRarity::Mythic => Color::srgb(0.9, 0.8, 0.2),
        }
    }
}

/// 装备栏组件
#[derive(Component, Clone, Debug)]
pub struct EquipmentBar {
    pub weapon: Option<Entity>,
    pub armor: Option<Entity>,
    pub accessory: Option<Entity>,
}

impl Default for EquipmentBar {
    fn default() -> Self {
        Self {
            weapon: None,
            armor: None,
            accessory: None,
        }
    }
}

impl EquipmentBar {
    /// 装备物品
    pub fn equip(&mut self, slot: EquipmentSlot, entity: Entity) {
        match slot {
            EquipmentSlot::Weapon => self.weapon = Some(entity),
            EquipmentSlot::Armor => self.armor = Some(entity),
            EquipmentSlot::Accessory => self.accessory = Some(entity),
        }
    }

    /// 卸下装备
    pub fn unequip(&mut self, slot: EquipmentSlot) -> Option<Entity> {
        match slot {
            EquipmentSlot::Weapon => self.weapon.take(),
            EquipmentSlot::Armor => self.armor.take(),
            EquipmentSlot::Accessory => self.accessory.take(),
        }
    }

    /// 获取装备的总属性
    pub fn total_stats(&self, equipment_query: &Query<&Equipment>) -> EquipmentStats {
        let mut total = EquipmentStats::default();

        for &entity in &[self.weapon, self.armor, self.accessory] {
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
}
