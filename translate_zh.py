import os
import re

# Comprehensive English -> Chinese translation dictionary
translations = {
    # Menu items (.create calls in main.rs)
    "Dashboard": "仪表盘",
    "Overview": "概览",
    "Network": "网络",
    "Security": "安全",
    "Delivery": "投递",
    "Performance": "性能",
    "Directory": "目录",
    "Accounts": "账户",
    "Groups": "群组",
    "Lists": "列表",
    "Domains": "域名",
    "Roles": "角色",
    "Tenants": "租户",
    "API Keys": "API 密钥",
    "OAuth Clients": "OAuth 客户端",
    "Queues": "队列",
    "Messages": "消息",
    "Reports": "报告",
    "DMARC Aggregate": "DMARC 聚合",
    "TLS Aggregate": "TLS 聚合",
    "Failures": "失败",
    "History": "历史",
    "Received Messages": "已接收消息",
    "Delivery Attempts": "投递尝试",
    "Telemetry": "遥测",
    "Logs": "日志",
    "Live tracing": "实时追踪",
    "Spam filter": "垃圾邮件过滤",
    "Upload samples": "上传样本",
    "Test filter": "测试过滤",
    "Troubleshoot": "故障排查",
    "E-mail Delivery": "邮件投递",
    "DMARC": "DMARC",
    "Settings": "设置",
    "Maintenance": "维护",

    # Common labels
    "Name": "名称",
    "Description": "描述",
    "Type": "类型",
    "Enabled": "已启用",
    "Enable": "启用",
    "Enable TLS": "启用 TLS",
    "Timeout": "超时",
    "Username": "用户名",
    "Password": "密码",
    "Secret": "密钥",
    "Port": "端口",
    "Hostname": "主机名",
    "URL": "URL",
    "Store": "存储",
    "Protocol": "协议",
    "Method": "方法",
    "Endpoint": "端点",
    "Endpoint URL": "端点 URL",
    "Model": "模型",
    "Temperature": "温度",
    "Size": "大小",
    "Max Size": "最大大小",
    "Max Recipients": "最大收件人数",
    "Max Attempts": "最大尝试次数",
    "Frequency": "频率",
    "Keys": "密钥",
    "Rate limit": "速率限制",
    "Send rate": "发送速率",
    "Match condition": "匹配条件",
    "Separator": "分隔符",
    "Strategy": "策略",
    "Signature": "签名",
    "Signature Key": "签名密钥",
    "From Name": "发件人名称",
    "From Address": "发件人地址",
    "Subject": "主题",
    "Domain Name": "域名",
    "Selector": "选择器",
    "Headers": "头部",
    "Expiration": "过期时间",
    "Algorithm": "算法",
    "Private Key": "私钥",
    "Organization": "组织",
    "Contact": "联系方式",
    "Throttle": "限流",
    "Run Script": "运行脚本",
    "HTTP Headers": "HTTP 头部",
    "Allow Invalid Certs": "允许无效证书",
    "Subscribe automatically": "自动订阅",
    "Create automatically": "自动创建",
    "TLS": "TLS",
    "TTL": "TTL",
    "Transport": "传输",
    "Retries": "重试次数",
    "Retry limit": "重试限制",
    "Retention period": "保留期限",
    "Rule": "规则",
    "Rule ID": "规则 ID",
    "Scope": "范围",
    "Script Id": "脚本 ID",
    "TempFail on Error": "错误时临时失败",
    "Received Headers": "接收头部",
    "E-mail": "电子邮件",
    "Emails": "邮箱",
    "Calendars": "日历",
    "Id": "ID",
    "Value": "值",
    "Signature ID": "签名 ID",
    "Canonicalization": "规范化",
    "Authorized Party": "授权方",
    "Hash Algorithm": "哈希算法",
    "Agent User ID": "代理用户 ID",
    "Request Reports": "请求报告",
    "Ignore insecure DKIM signatures": "忽略不安全的 DKIM 签名",
    "EHLO": "EHLO",
    "MAIL FROM": "MAIL FROM",
    "Report Addresses": "报告地址",
    "Forward": "转发",
    "Store duration": "存储时长",
    "Default Domain": "默认域名",
    "Submitter": "提交者",
    "Max Report Size": "最大报告大小",
    "Run on stages": "运行阶段",
    "Bucket": "存储桶",
    "Certificate": "证书",
    "Cluster Ids": "集群 ID",
    "Cluster Settings": "集群设置",
    "Caching": "缓存",
    "Cleanup": "清理",
    "Classification": "分类",
    "Calendar Alarms": "日历提醒",
    "Calendar Scheduling": "日历调度",
    "Calendar Settings": "日历设置",
    "Branding": "品牌",
    "Blob Store": "Blob 存储",
    "Automatic banning": "自动封禁",
    "Auto-learn": "自动学习",
    "Authorized Third-Party Signatures": "授权第三方签名",
    "Authorization Cache": "授权缓存",
    "Authentication": "认证",
    "Authentication settings": "认证设置",
    "Authentication Errors": "认证错误",
    "Archive": "归档",
    "Add Headers": "添加头部",
    "Address Handling": "地址处理",
    "Alert configuration": "告警配置",
    "AI Endpoint settings": "AI 端点设置",
    "ACME provider": "ACME 提供商",
    "ASN & GeoIP Settings": "ASN 和 GeoIP 设置",
    "ARC Sealing": "ARC 签封",
    "ARC Verification": "ARC 验证",
    "AUTH Stage": "AUTH 阶段",
    "CSV Parsing": "CSV 解析",

    # Alert messages
    "Network error": "网络错误",
    "Failed to deserialize response": "反序列化响应失败",
    "Not found": "未找到",
    "Forbidden": "禁止访问",
    "Unauthorized": "未授权",
    "OAuth failure": "OAuth 失败",
    "Invalid server response": "无效的服务器响应",
    "Unsupported server version": "不支持的服务器版本",
    "Device authenticated": "设备已认证",
    "Encryption-at-rest disabled": "静态加密已禁用",
    "Encryption-at-rest enabled": "静态加密已启用",
    "Password changed": "密码已更改",
    "Settings successfully reloaded": "设置已成功重新加载",
    "Successfully requested cancellation.": "已成功请求取消。",
    "Successfully requested immediate delivery.": "已成功请求立即投递。",
    "Upload successful": "上传成功",
    "Device authentication failed": "设备认证失败",
    "Incorrect password": "密码错误",
    "2FA Settings Updated": "双因素认证设置已更新",
    "Failed to reload settings": "重新加载设置失败",

    # Alert details
    "The OTP code you entered is invalid": "您输入的 OTP 验证码无效",
    "The code you entered is invalid or has expired": "您输入的验证码无效或已过期",
    "The password you entered is incorrect": "您输入的密码不正确",
    "You are not authorized to perform this action.": "您无权执行此操作。",
    "You have successfully authenticated your device": "您已成功认证设备",
    "Your 2FA settings has been updated successfully": "您的双因素认证设置已成功更新",
    "Your password has been changed successfully": "您的密码已成功更改",

    # Login/Auth page
    "You are not authorized to access this service.": "您无权访问此服务。",
    "Invalid redirect_uri parameter, must be a valid HTTPS URL": "无效的 redirect_uri 参数，必须是有效的 HTTPS URL",
    "Missing redirect_uri in query parameters": "查询参数中缺少 redirect_uri",

    # 404 page
    "Oops, something went wrong.": "出错了。",
    "Sorry, we could not find your page.": "抱歉，找不到您请求的页面。",
    "Back to manage": "返回管理",

    # HTTP error context strings
    "Field already exists": "字段已存在",
    "Missing required field": "缺少必填字段",
    "Operation not allowed": "不允许此操作",
    "Enterprise feature": "企业版功能",
    "Record already exists": "记录已存在",

    # Form-related
    "This feature is only available in the enterprise version of the software. ": "此功能仅在企业版中可用。",
    "Request trial.": "申请试用。",

    # Common UI words
    "Save": "保存",
    "Delete": "删除",
    "Cancel": "取消",
    "Edit": "编辑",
    "Create": "创建",
    "Search": "搜索",
    "Back": "返回",
    "Submit": "提交",
    "Close": "关闭",
    "Confirm": "确认",
    "Reset": "重置",
    "Reload": "重新加载",
    "Download": "下载",
    "Upload": "上传",
    "Export": "导出",
    "Import": "导入",
    "Add": "添加",
    "Remove": "移除",
    "Update": "更新",
    "Refresh": "刷新",
    "Apply": "应用",
    "Dismiss": "关闭",
    "Yes": "是",
    "No": "否",
    "None": "无",
    "All": "全部",
    "Select": "选择",
    "Filter": "过滤",
    "Clear": "清除",
    "Copy": "复制",
    "Paste": "粘贴",
    "Next": "下一步",
    "Previous": "上一步",
    "First": "第一页",
    "Last": "最后一页",
    "Loading": "加载中",
    "Saving": "保存中",
    "Deleting": "删除中",
    "Error": "错误",
    "Warning": "警告",
    "Success": "成功",
    "Info": "信息",
    "Details": "详情",
    "Actions": "操作",
    "Status": "状态",
    "Active": "活跃",
    "Inactive": "不活跃",
    "Pending": "待处理",
    "Required": "必填",
    "Optional": "可选",

    # More titles (section headers)
    "Spam Filter": "垃圾邮件过滤",
    "Inbound": "入站",
    "Outbound": "出站",
    "Listener": "监听器",
    "Listeners": "监听器",
    "Connection": "连接",
    "Connections": "连接",
    "Session": "会话",
    "Sessions": "会话",
    "Limits": "限制",
    "General": "通用",
    "Advanced": "高级",
    "Notifications": "通知",
    "Encryption": "加密",
    "Storage": "存储",
    "Server": "服务器",
    "Servers": "服务器",
    "Logging": "日志记录",
    "Tracing": "追踪",
    "Metrics": "指标",
    "Webhooks": "Webhooks",
    "Web Hooks": "Webhooks",
    "Push Notifications": "推送通知",
    "DNS": "DNS",
    "SMTP": "SMTP",
    "IMAP": "IMAP",
    "POP3": "POP3",
    "JMAP": "JMAP",
    "ManageSieve": "ManageSieve",
    "HTTP": "HTTP",
    "Relay": "中继",
    "Relay Host": "中继主机",
    "Queue": "队列",
    "Routing": "路由",
    "Remote Delivery": "远程投递",
    "Local Delivery": "本地投递",
    "Data": "数据",
    "Extensions": "扩展",
    "Milter": "Milter",
    "Milters": "Milter",
    "MTA-STS": "MTA-STS",
    "DANE": "DANE",
    "TLS Reporting": "TLS 报告",
    "DMARC Reporting": "DMARC 报告",
    "Sieve": "Sieve",
    "Sieve Scripts": "Sieve 脚本",
    "Trusted Senders": "可信发件人",
    "Blocked Senders": "已阻止发件人",
    "Spam Scores": "垃圾邮件评分",
    "Spam Lists": "垃圾邮件列表",
    "Bayes": "贝叶斯",
    "Reputation": "信誉",
    "Phishing": "钓鱼",
    "LLM": "LLM",
    "Pyzor": "Pyzor",
}
# Special translations for view! macro text (login/authorize pages)
view_text_translations = {
    "Host": "主机",
    "Login": "登录",
    "Password": "密码",
    "TOTP Token": "TOTP 验证码",
    "Remember me": "记住我",
    "Sign in": "登录",
    "Authorize": "授权",
    "Code": "验证码",
    "Back to manage": "返回管理",
}

# Walk all .rs files in src/
src_dir = os.path.join(os.path.dirname(os.path.abspath(__file__)), "src")
modified_count = 0

for root, dirs, files in os.walk(src_dir):
    for fname in files:
        if not fname.endswith(".rs"):
            continue
        filepath = os.path.join(root, fname)
        with open(filepath, "r", encoding="utf-8") as f:
            content = f.read()

        original = content

        # Sort translations by length (longest first) to avoid partial replacements
        sorted_translations = sorted(translations.items(), key=lambda x: len(x[0]), reverse=True)

        for eng, zh in sorted_translations:
            # Replace in .label("...") calls
            content = content.replace(f'.label("{eng}")', f'.label("{zh}")')
            # Replace in .title("...") calls
            content = content.replace(f'.title("{eng}")', f'.title("{zh}")')
            # Replace in .create("...") calls (menu items)
            content = content.replace(f'.create("{eng}")', f'.create("{zh}")')
            # Replace in Alert::error("...") calls
            content = content.replace(f'Alert::error("{eng}")', f'Alert::error("{zh}")')
            # Replace in Alert::success("...") calls
            content = content.replace(f'Alert::success("{eng}")', f'Alert::success("{zh}")')
            # Replace in Alert::warning("...") calls
            content = content.replace(f'Alert::warning("{eng}")', f'Alert::warning("{zh}")')
            # Replace in .with_details("...") calls
            content = content.replace(f'.with_details("{eng}")', f'.with_details("{zh}")')
            # Replace in .button("...") calls
            content = content.replace(f'.button("{eng}")', f'.button("{zh}")')
            # Replace in .placeholder("...") calls - skip, these are often technical
            # Replace in .help("...") calls if present
            content = content.replace(f'.help("{eng}")', f'.help("{zh}")')
            # Replace in .prefix("...") calls if present
            content = content.replace(f'.prefix("{eng}")', f'.prefix("{zh}")')
            # Replace in .suffix("...") calls if present
            content = content.replace(f'.suffix("{eng}")', f'.suffix("{zh}")')

        # Handle view! macro text for login/authorize/notfound pages
        # Pattern: standalone text between HTML tags in view! macros
        # e.g., <label ...>\n                            Host\n                        </label>
        if fname in ("login.rs", "authorize.rs", "notfound.rs"):
            for eng, zh in sorted(view_text_translations.items(), key=lambda x: len(x[0]), reverse=True):
                # Match text that appears as standalone content in view! macros
                # Pattern: whitespace + "English text" + whitespace (between tags)
                pattern = re.compile(
                    r'(>\s*\n\s+)' + re.escape(eng) + r'(\s*\n)',
                    re.MULTILINE
                )
                content = pattern.sub(r'\g<1>' + zh + r'\2', content)

        # Also handle the 404 page text patterns
        if fname == "notfound.rs":
            content = content.replace(
                ">Oops, something went wrong.</p>",
                ">出错了。</p>"
            )
            content = content.replace(
                ">Sorry, we could not find your page.</p>",
                ">抱歉，找不到您请求的页面。</p>"
            )

        if content != original:
            with open(filepath, "w", encoding="utf-8") as f:
                f.write(content)
            modified_count += 1
            print(f"Modified: {filepath}")

print(f"\nDone! Modified {modified_count} files.")
