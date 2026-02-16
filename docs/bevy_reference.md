# Bevy 0.18.1 快速参考指南

## 1. 安装与设置 (Installation & Setup)

### 添加依赖
在 `Cargo.toml` 中添加 Bevy 0.18.1（使用本地源码）：

```toml
[dependencies]
bevy = { path = "../bevy" } # 使用本地 Bevy 0.18.1 源码
```

**重要**: 本项目基于 Bevy 0.18.1 本地源码开发，确保使用 Required Components 和原生 Sprite，避免使用已废弃的 SpriteBundle。

或者使用命令：
```bash
cargo add bevy
```

### 基础应用结构 (Basic App Structure)
```rust
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins) // 添加默认插件组（窗口、渲染、输入等）
        .add_systems(Startup, setup) // 添加启动系统
        .add_systems(Update, game_logic) // 添加每帧运行的逻辑系统
        .run();
}

fn setup(mut commands: Commands) {
    // 初始化实体，使用 Required Components 而非 Bundle
    commands.spawn((
        Camera2d,
        Transform::default(),
        GlobalTransform::default(),
    ));
}

fn game_logic() {
    // 游戏逻辑
}
```

## 2. ECS 核心概念 (ECS Core Concepts)

### 实体 (Entity)
- 唯一的 ID，组件的容器。
- 创建：`commands.spawn((Component1, Component2))` # 使用 Required Components 元组
- 销毁：`commands.entity(entity_id).despawn()`
- **注意**: 避免使用已废弃的 Bundle，改用 Required Components 元组

### 组件 (Component)
- 数据结构，必须派生 `Component` trait。
```rust
#[derive(Component)]
struct Position { x: f32, y: f32 }
```

### 系统 (System)
- 普通 Rust 函数，处理逻辑。
- 参数可以是 `Query`, `Res`, `ResMut`, `Commands`, `EventReader`, `EventWriter` 等。

#### 查询 (Query)
- 访问组件数据。
```rust
fn move_player(mut query: Query<(&mut Position, &Velocity)>) {
    for (mut pos, vel) in &mut query {
        pos.x += vel.x;
        pos.y += vel.y;
    }
}
```

#### 资源 (Resource)
- 全局唯一数据。
- 注册：`app.insert_resource(MyResource)`
- 访问：`Res<MyResource>` (只读), `ResMut<MyResource>` (可写)

## 3. 常用插件与功能 (Common Plugins & Features)

### 窗口配置 (Window Configuration)
```rust
.add_plugins(DefaultPlugins.set(WindowPlugin {
    primary_window: Some(Window {
        title: "Dark Forest".into(),
        resolution: (1280.0, 720.0).into(),
        ..default()
    }),
    ..default()
}))
```

### 时间与计时器 (Time & Timers)
- `Res<Time>`: 获取 `time.delta()` 或 `time.elapsed()`。
- `Timer`:用于定时的结构体。

### 状态管理 (States)
- 定义状态枚举并派生 `States`。
- `app.init_state::<GameState>()`
- 系统调度：`.add_systems(Update, system.run_if(in_state(GameState::InGame)))`

## 4. 调试与开发工具
- **动态链接** (为了快速编译):
  在 `.cargo/config.toml` 中:
  ```toml
  [target.x86_64-unknown-linux-gnu]
  linker = "clang"
  rustflags = ["-C", "link-arg=-fuse-ld=lld"]
  ```
- **Bevy Inspector Egui**: 推荐用于调试 ECS 数据（需单独添加依赖）。

## 5. 精灵渲染 (Sprite Rendering)

### 使用原生 Sprite 组件（推荐）
```rust
fn spawn_sprite(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Sprite {
            image: asset_server.load("player.png"),
            custom_size: Some(Vec2::new(64.0, 64.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
        GlobalTransform::default(),
    ));
}
```

### Tilemap 渲染
- 使用原生 Bevy Tilemap 功能，暂不使用外部 crate
- 通过 Required Components 组合实现瓦片地图

## 6. 参考链接
- [Bevy 0.18.1 官方文档](https://bevyengine.org/learn/book/introduction/)
- [Bevy 示例](https://github.com/bevyengine/bevy/tree/main/examples)
- [Bevy Cheat Sheet](https://bevy-cheatbook.github.io/)
- [Required Components 指南](https://bevyengine.org/learn/book/migration-guides/0.18-0.19/#required-components)
