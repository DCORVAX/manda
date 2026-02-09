#!/bin/bash
# Kaku 配置版本检查和更新系统

set -euo pipefail

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
BOLD='\033[1m'
DIM='\033[2m'
NC='\033[0m'

# 当前配置版本（每次更新配置时递增）
CURRENT_CONFIG_VERSION=2

# 版本文件路径
VERSION_FILE="$HOME/.config/kaku/.kaku_config_version"

# 获取用户当前的配置版本
get_user_version() {
    if [[ -f "$VERSION_FILE" ]]; then
        cat "$VERSION_FILE"
    else
        # 全新用户
        echo "0"
    fi
}

# 保存版本号
save_version() {
    mkdir -p "$(dirname "$VERSION_FILE")"
    echo "$1" > "$VERSION_FILE"
}

# 获取版本更新日志
get_changelog() {
    local from_version=$1
    local to_version=$2

    echo -e "${BOLD}配置更新内容：${NC}"
    echo ""

    # v0 -> v1
    if [[ $from_version -lt 1 && $to_version -ge 1 ]]; then
        echo -e "${GREEN}✓${NC} v1 - 初始配置"
        echo "  • Starship 提示符"
        echo "  • zsh-z 智能跳转"
        echo "  • zsh-autosuggestions 自动建议"
        echo "  • zsh-syntax-highlighting 语法高亮"
        echo ""
    fi

    # v1 -> v2
    if [[ $from_version -lt 2 && $to_version -ge 2 ]]; then
        echo -e "${GREEN}✓${NC} v2 - 性能优化 + Delta"
        echo "  • ZSH 启动速度优化（减少 40% 启动时间）"
        echo "  • compinit 缓存优化"
        echo "  • 延迟加载语法高亮"
        echo "  • Delta - 美化 git diff 输出"
        echo "  • 优化别名（ll 不再显示隐藏文件）"
        echo ""
    fi

    # 未来版本示例
    # v2 -> v3
    # if [[ $from_version -lt 3 && $to_version -ge 3 ]]; then
    #     echo -e "${GREEN}✓${NC} v3 - 新功能"
    #     echo "  • XXX"
    # fi
}

# 应用配置更新
apply_updates() {
    local from_version=$1
    local to_version=$2
    local resource_dir=$3

    echo -e "${BLUE}正在应用配置更新...${NC}"
    echo ""

    # v1 -> v2 更新（ZSH 优化 + Delta）
    if [[ $from_version -lt 2 && $to_version -ge 2 ]]; then
        # 重新生成 kaku.zsh（包含性能优化）
        if [[ -f "$resource_dir/setup_zsh.sh" ]]; then
            echo -e "  ${DIM}• 更新 ZSH 配置（性能优化）${NC}"
            bash "$resource_dir/setup_zsh.sh" --update-only
        fi

        # 询问是否安装 delta
        echo ""
        echo -e "${BOLD}Delta (Git Diff 美化工具)${NC}"
        echo -e "  ${DIM}• 语法高亮的 git diff 输出${NC}"
        echo -e "  ${DIM}• 更好的代码审查体验${NC}"
        echo -e "  ${DIM}• 对 AI Coding 工作流友好${NC}"
        echo ""
        echo -en "是否安装 Delta？ (Y/n) "
        read -r response
        if [[ -z "$response" || "$response" =~ ^[Yy]$ ]]; then
            if [[ -f "$resource_dir/install_delta.sh" ]]; then
                bash "$resource_dir/install_delta.sh"
            else
                echo -e "  ${YELLOW}⚠${NC} 安装脚本未找到，请手动安装：brew install git-delta"
            fi
        fi
    fi

    echo ""
    echo -e "${GREEN}✓ 配置更新完成！${NC}"
}

# 主逻辑
main() {
    local user_version=$(get_user_version)

    # 如果是全新用户（版本 0），不显示更新提示
    if [[ $user_version -eq 0 ]]; then
        # 首次启动会由 first_run.sh 处理
        return 0
    fi

    # 检查是否需要更新
    if [[ $user_version -lt $CURRENT_CONFIG_VERSION ]]; then
        clear
        echo -e "${BOLD}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
        echo -e "${BOLD}  🎉 Kaku 配置有更新！${NC}"
        echo -e "${BOLD}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
        echo ""
        echo -e "  当前版本: ${DIM}v$user_version${NC}"
        echo -e "  最新版本: ${GREEN}${BOLD}v$CURRENT_CONFIG_VERSION${NC}"
        echo ""

        # 显示更新日志
        get_changelog "$user_version" "$CURRENT_CONFIG_VERSION"

        echo -e "${BOLD}是否现在更新配置？${NC} (Y/n/later)"
        echo -e "  ${DIM}Y     - 立即更新（推荐）${NC}"
        echo -e "  ${DIM}n     - 跳过此版本${NC}"
        echo -e "  ${DIM}later - 下次启动再提醒${NC}"
        echo ""
        echo -en "请选择: "
        read -r response

        case "$response" in
            [Nn])
                # 跳过，但标记为当前版本（不再提示）
                save_version "$CURRENT_CONFIG_VERSION"
                echo -e "${YELLOW}已跳过配置更新${NC}"
                ;;
            [Ll]*)
                # 下次再提醒（不保存版本）
                echo -e "${YELLOW}下次启动会再次提醒${NC}"
                ;;
            *)
                # 默认 Yes，应用更新
                echo ""

                # 确定 resource_dir
                local resource_dir
                if [[ -d "/Applications/Kaku.app/Contents/Resources" ]]; then
                    resource_dir="/Applications/Kaku.app/Contents/Resources"
                else
                    # 开发环境
                    resource_dir="$(dirname "$0")"
                fi

                apply_updates "$user_version" "$CURRENT_CONFIG_VERSION" "$resource_dir"
                save_version "$CURRENT_CONFIG_VERSION"

                echo ""
                echo -e "${GREEN}${BOLD}配置已更新到 v$CURRENT_CONFIG_VERSION！${NC}"
                echo ""
                echo "请重启终端以应用所有更改。"
                ;;
        esac

        echo ""
        echo "按任意键继续..."
        read -n 1 -s
    fi
}

# 如果直接运行（非 source）
if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
    main "$@"
fi
