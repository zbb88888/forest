.PHONY: run run-headless build test clean run-sys0 run-sys1 run-sys2 run-sys3 run-sys4 run-sys50 run-sys999

# Default run (all systems)
run:
	cargo run

# Headless mode
run-headless:
	HEADLESS=1 cargo run

# Build release
build:
	cargo build --release

# Run tests
test:
	cargo test

# Clean build artifacts
clean:
	cargo clean

# ============================================================
# Sys0: 纯净地图 (基础渲染 + UI)
# 包含: map, player, time, lighting, menu, hud
# 依赖: 无
# ============================================================
run-sys0:
	LAYER=0 cargo run

# ============================================================
# Sys1: 实体生成
# 包含: Sys0 + enemy_spawn, enemy_base, plant
# 依赖: Sys0
# ============================================================
run-sys1:
	LAYER=1 cargo run

# ============================================================
# Sys2: 实体行为
# 包含: Sys1 + enemy, robot, equipment
# 依赖: Sys1
# ============================================================
run-sys2:
	LAYER=2 cargo run

# ============================================================
# Sys3: 战斗系统
# 包含: Sys2 + enemy_attack, player_combat, combat, combat_effects,
#       defense_tower, defense_wall, defense_range
# 依赖: Sys2
# ============================================================
run-sys3:
	LAYER=3 cargo run

# ============================================================
# Sys4: 生产建造
# 包含: Sys3 + plant_upgrade, crafting, building (含UI)
# 依赖: Sys3
# ============================================================
run-sys4:
	LAYER=4 cargo run

# ============================================================
# Sys50: 任务成就
# 包含: Sys4 + quest_manager, quest_events, quest_generator,
#       achievement_manager, achievement_events, achievement_generator
# 依赖: Sys4
# ============================================================
run-sys50:
	LAYER=50 cargo run

# ============================================================
# Sys999: 存档系统 (完整版)
# 包含: Sys50 + save_manager, save_ui
# 依赖: Sys50
# ============================================================
run-sys999:
	LAYER=999 cargo run