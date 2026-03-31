# 将明文 SVG 替换为 Icon 库 Spec

## Why
项目中多处直接使用内联 SVG，导致代码冗余、难以维护。使用统一的 icon 库可以简化代码，提高可维护性。

## What Changes
- 安装 @primer/octicons-react 或类似 icon 库
- 替换所有 Vue 文件中的内联 SVG 为 icon 组件
- 保持原有样式和交互不变

## Impact
- Affected files:
  - src/components/Sidebar.vue (多处 SVG)
  - src/views/Home.vue (多处 SVG)
  - src/views/Auth.vue (Logo SVG)

## ADDED Requirements
### Requirement: Icon 库集成
The system SHALL 安装并使用 @primer/octicons 作为图标库

#### Scenario: 安装依赖
- **WHEN** 执行 npm install
- **THEN** @primer/octicons 被正确安装

#### Scenario: Sidebar 图标替换
- **WHEN** 渲染 Sidebar 组件
- **THEN** 所有导航图标使用 Octicon 组件替代内联 SVG
- **AND** 保持原有样式

#### Scenario: Home 页面图标替换
- **WHEN** 渲染 Home 页面
- **THEN** 空状态、查看按钮、删除按钮的图标使用 Octicon 组件
- **AND** 保持原有交互

#### Scenario: Auth 页面图标替换
- **WHEN** 渲染 Auth 页面
- **THEN** Logo 使用 Octicon 组件或保持 SVG（因是自定义图形）

## MODIFIED Requirements
无

## REMOVED Requirements
无
