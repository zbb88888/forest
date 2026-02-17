# 自动化机器人系统

## 系统概述
自动化机器人系统实现了"零肝度"的核心玩法，通过智能AI自动执行采集、建造、巡逻等任务，让玩家专注于宏观策略。

---

## 1. 机器人类型

### 1.1 实现文件
- **组件**：`src/components/robot.rs` (Robot, RobotType, RobotTask, RobotAI, RobotInventory)
- **系统**：`src/systems/robot.rs` (spawn_robot, robot_ai_system)

### 1.2 机器人类型表

| 类型 | 颜色 | 移动速度 | 能量消耗 | 背包容量 | 主要功能 |
|------|------|----------|----------|----------|----------|
| Harvester | 棕色 | 2.0 | 0.5/秒 | 50 | 采集资源 |
| Builder | 绿色 | 1.5 | 0.8/秒 | 30 | 建造维修 |
| Defender | 红色 | 2.5 | 0.6/秒 | 20 | 防御战斗 |
| Scout | 蓝色 | 3.0 | 0.3/秒 | 10 | 探索侦察 |

### 1.3 机器人属性

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

### 1.4 机器人AI属性

```rust
pub struct RobotAI {
    pub patrol_radius: f32,      // 巡逻半径 200.0
    pub detection_radius: f32,   // 探测半径 100.0
    pub current_patrol_index: usize,
    pub patrol_points: Vec<Vec2>,
}
```

### 1.5 机器人背包

```rust
pub struct RobotInventory {
    pub capacity: u32,
    pub current: u32,
    pub resource_type: Option<ResourceType>,
}
```

---

## 2. 任务系统

### 2.1 任务类型

| 任务类型 | 说明 | 适用机器人 |
|----------|------|------------|
| Idle | 空闲状态 | 所有 |
| Harvest | 采集资源 | Harvester |
| Build | 建造建筑 | Builder |
| Patrol | 巡逻区域 | Scout, Defender |
| Repair | 维修设施 | Builder |
| ReturnToBase | 返回基地 | 所有 |

### 2.2 任务流程

#### 采集任务流程
```
Idle → 寻找目标 → Harvest → 移动到目标 → 采集 → 
背包满? → Yes → ReturnToBase → 卸载 → Idle
               → No → 继续采集
```

#### 巡逻任务流程
```
Idle → 生成巡逻点 → Patrol → 移动到点 → 到达? → 
Yes → Idle → 生成新巡逻点 → Patrol
```

#### 返回基地流程
```
任何任务 → 能量不足/背包满 → ReturnToBase → 
移动到玩家 → 到达? → Yes → 卸载/充能 → Idle
```

---

## 3. 机器人AI

### 3.1 AI决策树

```
开始
  │
  ├─ 能量 <= 0?
  │   └─ Yes → ReturnToBase
  │   └─ No → 继续
  │
  ├─ 当前任务?
  │   ├─ Idle → 根据类型分配任务
  │   ├─ Harvest → 执行采集
  │   ├─ Patrol → 执行巡逻
  │   └─ ReturnToBase → 返回玩家
  │
  └─ 执行任务逻辑
```

### 3.2 任务分配规则

#### 采集机器人
- 寻找最近的成熟植物
- 检测半径：100像素
- 优先选择高产出植物

#### 侦察机器人
- 随机生成巡逻点
- 巡逻半径：200像素
- 探索未知区域

#### 建造机器人
- 待实现建造逻辑
- 当前执行基础巡逻

#### 防御机器人
- 待实现防御逻辑
- 当前执行基础巡逻

### 3.3 移动系统

```rust
fn move_towards_target(transform: &mut Transform, target: Vec2, speed: f32) {
    let current = transform.translation.truncate();
    let direction = (target - current).normalize();
    let new_pos = current + direction * speed;

    transform.translation.x = new_pos.x;
    transform.translation.y = new_pos.y;
}
```

### 3.4 夜晚影响
- **效率降低**：夜晚移动速度降至50%
- **影响范围**：所有机器人
- **计算公式**：`实际速度 = 基础速度 × 0.5`

---

## 4. 能量管理

### 4.1 能量参数

| 机器人类型 | 最大能量 | 消耗速率 | 充能时间 |
|------------|----------|----------|----------|
| Harvester | 100 | 0.5/秒 | 200秒 |
| Builder | 150 | 0.8/秒 | 187.5秒 |
| Defender | 120 | 0.6/秒 | 200秒 |
| Scout | 80 | 0.3/秒 | 266.7秒 |

### 4.2 能量消耗
- **基础消耗**：每秒固定消耗
- **任务消耗**：执行任务时额外消耗
- **移动消耗**：移动时增加消耗（待实现）

### 4.3 充能机制
- **充能条件**：返回玩家位置50像素内
- **充能速度**：瞬间充满
- **充能效果**：能量恢复到最大值

### 4.4 能量不足处理
- **触发条件**：能量 <= 0
- **处理方式**：自动返回基地
- **任务中断**：当前任务暂停

---

## 5. 资源管理

### 5.1 背包容量

| 机器人类型 | 容量 | 主要资源 |
|------------|------|----------|
| Harvester | 50 | 能源 |
| Builder | 30 | 金属、土壤 |
| Defender | 20 | - |
| Scout | 10 | - |

### 5.2 资源采集
- **采集范围**：32像素
- **采集条件**：植物成熟
- **采集速度**：瞬间完成
- **资源类型**：根据植物类型确定

### 5.3 资源卸载
- **卸载位置**：玩家位置
- **卸载条件**：返回基地
- **卸载方式**：全部卸载
- **玩家获得**：资源加入玩家背包

---

## 6. 生成系统

### 6.1 生成方式

| 按键 | 机器人类型 | 位置 |
|------|------------|------|
| 1 | Harvester | 玩家位置 |
| 2 | Builder | 玩家位置 |
| 3 | Defender | 玩家位置 |
| 4 | Scout | 玩家位置 |

### 6.2 生成参数
- **大小**：24x24像素
- **层级**：Z=2（在植物之上）
- **初始状态**：
  - 能量：满
  - 任务：Idle
  - 背包：空

---

## 7. 系统集成

### 7.1 初始化
- 无需特殊初始化
- 机器人通过玩家操作动态生成

### 7.2 更新顺序
```rust
.add_systems(Update, (
    systems::robot::spawn_robot,
    systems::robot::robot_ai_system,
).run_if(in_state(GameState::InGame)))
```

### 7.3 依赖关系
- 依赖时间系统（获取昼夜阶段）
- 依赖植物系统（采集目标）
- 依赖玩家系统（返回基地）
- 依赖地图系统（移动限制）

---

## 8. 性能优化

### 8.1 AI优化
- 使用空间分区优化目标查找
- 缓存常用计算结果
- 限制AI更新频率

### 8.2 移动优化
- 使用路径规划（待实现）
- 避免重复计算
- 批量更新位置

### 8.3 任务优化
- 任务队列管理
- 优先级调度
- 任务复用

---

## 9. 待实现功能

### 9.1 AI增强
- [ ] 路径规划算法
- [ ] 协作机制
- [ ] 任务优先级系统
- [ ] 学习能力

### 9.2 任务扩展
- [ ] 建造任务实现
- [ ] 维修任务实现
- [ ] 防御任务实现
- [ ] 运输任务

### 9.3 交互增强
- [ ] 机器人命令系统
- [ ] 机器人编队
- [ ] 机器人升级
- [ ] 机器人自定义

---

## 10. 已知问题

### 10.1 AI问题
- 机器人可能卡在障碍物
- 目标选择不够智能
- 缺少协作机制

### 10.2 任务问题
- 建造和防御任务未实现
- 任务切换生硬
- 缺少任务队列

### 10.3 性能问题
- 大量机器人时AI计算量大
- 目标查找效率低
- 缺少空间分区

---

## 11. 更新日志

### v0.1.0 (当前版本)
- ✅ 实现4种机器人类型
- ✅ 实现基础AI系统
- ✅ 实现任务系统框架
- ✅ 实现采集任务
- ✅ 实现巡逻任务
- ✅ 实现能量管理
- ✅ 实现资源管理
- ✅ 实现夜晚影响
