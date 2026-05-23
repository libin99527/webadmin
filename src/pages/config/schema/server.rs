/*
 * SPDX-FileCopyrightText: 2020 Stalwart Labs LLC <hello@stalw.art>
 *
 * SPDX-License-Identifier: AGPL-3.0-only OR LicenseRef-SEL
 */

use crate::core::schema::*;

use super::tracing::EVENT_NAMES;

impl Builder<Schemas, ()> {
    #![allow(clippy::useless_concat)]
    pub fn build_server(self) -> Self {
        self.new_schema("network")
            // Default hostname
            .new_field("server.hostname")
            .label("主机名")
            .help("The default fully-qualified system hostname")
            .placeholder("mail.example.com")
            .typ(Type::Input)
            .input_check(
                [Transformer::Trim],
                [Validator::Required, Validator::IsHost],
            )
            // Max connections
            .new_field("server.max-connections")
            .label("最大连接数")
            .help("The maximum number of concurrent connections the server will accept")
            .typ(Type::Input)
            .input_check(
                [Transformer::Trim],
                [Validator::Required, Validator::MinValue(1.into())],
            )
            .default("8192")
            .build()
            // Network fields
            .add_network_fields(false)
            // Forms
            .new_form_section()
            .title("网络设置")
            .fields([
                "server.hostname",
                "server.max-connections",
                "server.proxy.trusted-networks",
            ])
            .build()
            .new_form_section()
            .title("套接字选项")
            .fields([
                "server.socket.backlog",
                "server.socket.ttl",
                "server.socket.linger",
                "server.socket.tos",
                "server.socket.send-buffer-size",
                "server.socket.recv-buffer-size",
                "server.socket.nodelay",
                "server.socket.reuse-addr",
                "server.socket.reuse-port",
            ])
            .build()
            .build()
            // Common settings
            .new_schema("system")
            // Local keys
            .new_field("config.local-keys")
            .label("本地设置")
            .help(concat!(
                "List of glob expressions for local configuration keys",
                " that should be stored locally in the configuration file.",
                "All other keys will be stored in the database. If left blank ",
                "the default settings will be used (check the documentation for more info)"
            ))
            .typ(Type::Array(ArrayType::Text))
            .input_check([Transformer::Trim], [Validator::Required])
            .default(
                &[
                    "store.*",
                    "directory.*",
                    "tracer.*",
                    "!server.blocked-ip.*",
                    "!server.allowed-ip.*",
                    "server.*",
                    "config.local-keys.*",
                    "certificate.*",
                    "cluster.*",
                    "storage.data",
                    "storage.blob",
                    "storage.lookup",
                    "storage.fts",
                    "storage.directory",
                    "authentication.fallback-admin.*",
                    "enterprise.license-key",
                ][..],
            )
            .build()
            // Thread pool
            .new_field("global.thread-pool")
            .label("连接池大小")
            .help(concat!(
                "The number of threads in the global thread pool for ",
                "CPU intensive tasks. Defaults to the number ",
                "of CPU cores"
            ))
            .typ(Type::Input)
            .input_check([Transformer::Trim], [Validator::MinValue(1.into())])
            .placeholder("8")
            .build()
            .new_form_section()
            .title("本地配置键")
            .fields(["config.local-keys"])
            .build()
            .new_form_section()
            .title("线程池")
            .fields(["global.thread-pool"])
            .build()
            .build()
            // Caching
            .new_schema("cache")
            .new_field("cache.dns.txt.size")
            .label("TXT 记录")
            .help(concat!("Maximum size of the TXT record cache"))
            .default("5242880")
            .typ(Type::Size)
            .input_check([], [Validator::Required, Validator::MinValue(2048.into())])
            .new_field("cache.dns.mx.size")
            .label("MX 记录")
            .help(concat!("Maximum size of the MX record cache"))
            .default("5242880")
            .new_field("cache.dns.ipv4.size")
            .label("IPv4 记录")
            .help(concat!("Maximum size of the IPv4 record cache"))
            .default("5242880")
            .new_field("cache.dns.ipv6.size")
            .label("IPv6 记录")
            .help(concat!("Maximum size of the IPv6 record cache"))
            .default("5242880")
            .new_field("cache.dns.ptr.size")
            .label("PTR 记录")
            .help(concat!("Maximum size of the PTR record cache"))
            .default("1048576")
            .new_field("cache.dns.tlsa.size")
            .label("TLSA 记录")
            .help(concat!("Maximum size of the TLSA record cache"))
            .default("1048576")
            .new_field("cache.dns.mta-sts.size")
            .label("MTA-STS 记录")
            .help(concat!("Maximum size of the MTA-STS record cache"))
            .default("1048576")
            .new_field("cache.dns.rbl.size")
            .label("RBL 记录")
            .help(concat!("Maximum size of the DNSBl record cache"))
            .default("5242880")
            .new_field("cache.access-token.size")
            .label("访问令牌")
            .help(concat!("Maximum size of the access tokens cache"))
            .default("10485760")
            .new_field("cache.http-auth.size")
            .label("HTTP 授权")
            .help(concat!(
                "Maximum size of the HTTP authorization headers cache"
            ))
            .default("1048576")
            .new_field("cache.permission.size")
            .label("权限")
            .help(concat!("Maximum size of the effective permissions cache"))
            .default("5242880")
            .new_field("cache.message.size")
            .label("邮箱")
            .help(concat!("Maximum size of the e-mail data cache"))
            .default("52428800")
            .new_field("cache.files.size")
            .label("文件")
            .help(concat!("Maximum size of the file storage data cache"))
            .default("10485760")
            .new_field("cache.events.size")
            .label("日历")
            .help(concat!("Maximum size of the calendar and events cache"))
            .default("10485760")
            .new_field("cache.contacts.size")
            .label("联系人")
            .help(concat!(
                "Maximum size of the address books and contacts cache"
            ))
            .default("10485760")
            .build()
            .new_form_section()
            .title("数据缓存")
            .fields([
                "cache.message.size",
                "cache.events.size",
                "cache.contacts.size",
                "cache.files.size",
            ])
            .build()
            .new_form_section()
            .title("授权缓存")
            .fields([
                "cache.access-token.size",
                "cache.http-auth.size",
                "cache.permission.size",
            ])
            .build()
            .new_form_section()
            .title("垃圾邮件过滤缓存")
            .fields(["cache.dns.rbl.size"])
            .build()
            .new_form_section()
            .title("DNS 记录缓存")
            .fields([
                "cache.dns.txt.size",
                "cache.dns.mx.size",
                "cache.dns.ipv4.size",
                "cache.dns.ipv6.size",
                "cache.dns.ptr.size",
                "cache.dns.tlsa.size",
                "cache.dns.mta-sts.size",
            ])
            .build()
            .build()
            // Blocked IP addresses
            .new_schema("blocked-ip")
            .reload_prefix("server.blocked-ip")
            .names("address", "addresses")
            .prefix("server.blocked-ip")
            .new_id_field()
            .label("IP 地址")
            .help("The IP address or mask to block")
            .input_check(
                [Transformer::Trim],
                [Validator::Required, Validator::IsIpOrMask],
            )
            .build()
            .new_form_section()
            .field("_id")
            .build()
            .list_title("Blocked IP addresses")
            .list_subtitle("Manage blocked IP addresses")
            .list_fields(["_id"])
            .no_list_action(Action::Modify)
            .build()
            // Allowed IP addresses
            .new_schema("allowed-ip")
            .reload_prefix("server.allowed-ip")
            .names("address", "addresses")
            .prefix("server.allowed-ip")
            .new_id_field()
            .label("IP 地址")
            .help("The IP address or mask to allow")
            .input_check(
                [Transformer::Trim],
                [Validator::Required, Validator::IsIpOrMask],
            )
            .build()
            .new_form_section()
            .field("_id")
            .build()
            .list_title("Allowed IP addresses")
            .list_subtitle("Manage allowed IP addresses")
            .list_fields(["_id"])
            .no_list_action(Action::Modify)
            .build()
            // Auto-ban settings
            .new_schema("auto-ban")
            .new_field("server.auto-ban.auth.rate")
            .label("认证失败")
            .help("The maximum number of failed login attempts before the IP is banned")
            .typ(Type::Rate)
            .default("100/1d")
            .build()
            .new_field("server.auto-ban.scan.rate")
            .label("扫描尝试")
            .help(concat!(
                "The maximum number of port scanning attempts before the IP is banned"
            ))
            .typ(Type::Rate)
            .default("30/1d")
            .build()
            .new_field("server.auto-ban.abuse.rate")
            .label("滥用尝试")
            .help(concat!(
                "The maximum number of abuse attempts (relaying or failed ",
                "RCPT TO attempts) before the IP is banned"
            ))
            .typ(Type::Rate)
            .default("35/1d")
            .build()
            .new_field("server.auto-ban.loiter.rate")
            .label("逗留")
            .help("The maximum number of loitering disconnections before the IP is banned")
            .typ(Type::Rate)
            .default("150/1d")
            .build()
            .new_field("server.auto-ban.scan.paths")
            .label("HTTP 禁止路径")
            .help(concat!(
                "The paths that will trigger an immediate ban if accessed. ",
                "Each path should be a glob expression"
            ))
            .typ(Type::Array(ArrayType::Text))
            .input_check([Transformer::Trim], [])
            .default(
                &[
                    "*.php*",
                    "*.cgi*",
                    "*.asp*",
                    "*/wp-*",
                    "*/php*",
                    "*/cgi-bin*",
                    "*xmlrpc*",
                    "*../*",
                    "*/..*",
                    "*joomla*",
                    "*wordpress*",
                    "*drupal*",
                ][..],
            )
            .build()
            .new_form_section()
            .title("自动封禁")
            .fields([
                "server.auto-ban.auth.rate",
                "server.auto-ban.abuse.rate",
                "server.auto-ban.loiter.rate",
            ])
            .build()
            .new_form_section()
            .title("端口扫描封禁")
            .fields(["server.auto-ban.scan.rate", "server.auto-ban.scan.paths"])
            .build()
            .build()
            // Clustering
            .new_schema("cluster")
            // Cluster node ID
            .new_field("cluster.node-id")
            .label("节点 ID")
            .help(concat!("Unique identifier for this node in the cluster"))
            .default("1")
            .typ(Type::Input)
            .input_check(
                [Transformer::Trim],
                [Validator::Required, Validator::MinValue(0.into())],
            )
            .build()
            // Pubsub
            .new_field("cluster.coordinator")
            .label("协调器")
            .help(concat!(
                "The id of the coordinator backend to use for distributing events",
                " in the cluster. Leave blank to disable."
            ))
            .typ(Type::Select {
                source: Source::Dynamic {
                    schema: "store",
                    field: "type",
                    filter: Default::default(),
                },
                typ: SelectType::Single,
            })
            .source_filter(&["redis", "nats"])
            .build()
            // Roles
            .new_field("cluster.roles.purge.stores")
            .label("清除存储")
            .help(concat!(
                "List of node ids that are responsible for purging stores"
            ))
            .typ(Type::Array(ArrayType::Text))
            .input_check([Transformer::Trim], [])
            .new_field("cluster.roles.purge.accounts")
            .label("清除账户")
            .help(concat!(
                "List of node ids that are responsible for purging accounts"
            ))
            .new_field("cluster.roles.acme.renew")
            .label("续期 ACME")
            .help(concat!(
                "List of node ids that are responsible for renewing ACME certificates"
            ))
            .new_field("cluster.roles.metrics.calculate")
            .label("计算指标")
            .help(concat!(
                "List of node ids that are responsible for calculating metrics"
            ))
            .new_field("cluster.roles.metrics.push")
            .label("推送指标")
            .help(concat!(
                "List of node ids that are responsible for pushing metrics"
            ))
            .new_field("cluster.roles.push-notifications")
            .label("推送通知")
            .help(concat!(
                "List of node ids that are responsible for sending push notifications"
            ))
            .new_field("cluster.roles.fts-indexing")
            .label("全文搜索索引")
            .help(concat!(
                "List of node ids that are responsible for full-text search indexing"
            ))
            .new_field("cluster.roles.spam-training")
            .label("垃圾邮件分类器训练")
            .help(concat!(
                "The node id that is responsible for training the spam classifier model"
            ))
            .new_field("cluster.roles.imip-processing")
            .label("iMIP 处理")
            .help(concat!(
                "List of node ids that are responsible for processing iMIP calendar messages"
            ))
            .new_field("cluster.roles.calendar-alerts")
            .label("日历提醒")
            .help(concat!(
                "List of node ids that are responsible for sending calendar alerts"
            ))
            .build()
            // Forms
            .new_form_section()
            .title("集群设置")
            .fields(["cluster.node-id", "cluster.coordinator"])
            .build()
            .new_form_section()
            .title("节点角色")
            .fields([
                "cluster.roles.purge.stores",
                "cluster.roles.purge.accounts",
                "cluster.roles.acme.renew",
                "cluster.roles.metrics.calculate",
                "cluster.roles.metrics.push",
                "cluster.roles.push-notifications",
                "cluster.roles.fts-indexing",
                "cluster.roles.spam-training",
                "cluster.roles.imip-processing",
                "cluster.roles.calendar-alerts",
            ])
            .build()
            .build()
            // Web hooks
            .new_schema("web-hooks")
            .prefix("webhook")
            .suffix("url")
            .names("webhook", "webhooks")
            .new_id_field()
            .label("Webhook ID")
            .help("Unique identifier for this webhook")
            .build()
            .new_field("url")
            .label("端点 URL")
            .help(concat!("URL of the webhook endpoint"))
            .placeholder("https://127.0.0.1/webhook")
            .typ(Type::Input)
            .input_check([Transformer::Trim], [Validator::Required, Validator::IsUrl])
            .build()
            .new_field("allow-invalid-certs")
            .label("允许无效证书")
            .help(concat!(
                "Whether Stalwart should connect to a webhook ",
                "endpoint that has an invalid TLS certificate"
            ))
            .default("false")
            .typ(Type::Boolean)
            .input_check([], [Validator::Required])
            .build()
            .new_field("timeout")
            .label("超时")
            .help(concat!(
                "Maximum amount of time that Stalwart will wait for a response ",
                "from this webhook"
            ))
            .default("30s")
            .typ(Type::Duration)
            .input_check([], [Validator::Required])
            .build()
            .new_field("throttle")
            .label("限流")
            .help(concat!(
                "The minimum amount of time that must pass between ",
                "each request to the webhook endpoint"
            ))
            .default("1s")
            .typ(Type::Duration)
            .input_check([], [Validator::Required])
            .build()
            .new_field("signature-key")
            .label("签名密钥")
            .help(concat!(
                "The HMAC key used to sign the webhook request body ",
                "to prevent tampering"
            ))
            .typ(Type::Secret)
            .build()
            .new_field("headers")
            .typ(Type::Array(ArrayType::Text))
            .label("HTTP 头部")
            .help("The headers to be sent with webhook requests")
            .build()
            .new_field("auth.username")
            .label("用户名")
            .help(concat!(
                "The username to use when authenticating with the webhook endpoint"
            ))
            .typ(Type::Input)
            .input_check([Transformer::Trim], [])
            .build()
            .new_field("auth.secret")
            .label("密钥")
            .help(concat!(
                "The secret to use when authenticating with the webhook endpoint"
            ))
            .typ(Type::Secret)
            .build()
            .new_field("events")
            .label("事件")
            .help("Which events should trigger this webhook")
            .typ(Type::Select {
                typ: SelectType::ManyWithSearch,
                source: Source::StaticId(EVENT_NAMES),
            })
            .build()
            .new_form_section()
            .title("Webhook 设置")
            .fields(["_id", "url", "signature-key", "allow-invalid-certs"])
            .build()
            .new_form_section()
            .title("认证")
            .fields(["auth.username", "auth.secret"])
            .build()
            .new_form_section()
            .title("触发器")
            .fields(["events"])
            .build()
            .new_form_section()
            .title("选项")
            .fields(["throttle", "timeout", "headers"])
            .build()
            .list_title("Webhooks")
            .list_subtitle("Manage Webhooks")
            .list_fields(["_id", "url"])
            .build()
            // Enterprise settings
            .new_schema("enterprise")
            // License key
            .new_field("enterprise.license-key")
            .label("许可证密钥")
            .help(concat!(
                "Upgrade to the enterprise version of Stalwart by ",
                "entering your license key here. Obtain your license at ",
                "https://license.stalw.art/buy"
            ))
            .typ(Type::Secret)
            .input_check([Transformer::Trim], [])
            .build()
            .new_field("enterprise.api-key")
            .label("API 密钥")
            .help(concat!(
                "API key for license retrieval and automatic renewals. ",
                "Obtain your API key at https://license.stalw.art.",
            ))
            .typ(Type::Secret)
            .input_check([Transformer::Trim], [])
            .build()
            .new_field("enterprise.logo-url")
            .label("默认 Logo URL")
            .help(concat!(
                "URL to the default logo to use in the Webadmin interface. ",
                "(Enterprise feature)"
            ))
            .typ(Type::Input)
            .input_check([Transformer::Trim], [Validator::IsUrl])
            .enterprise_feature()
            .build()
            .new_form_section()
            .title("许可证")
            .fields(["enterprise.license-key", "enterprise.api-key"])
            .build()
            .new_form_section()
            .title("品牌")
            .fields(["enterprise.logo-url"])
            .build()
            .build()
            // AI models
            .new_schema("ai-models")
            .prefix("enterprise.ai")
            .suffix("url")
            .names("model", "models")
            .new_id_field()
            .label("模型 ID")
            .help("Unique identifier for this AI model")
            .enterprise_feature()
            .build()
            .new_field("url")
            .label("端点 URL")
            .help(concat!("URL of the OpenAI compatible endpoint"))
            .placeholder("https://api.openai.com/v1/chat/completions")
            .typ(Type::Input)
            .input_check([Transformer::Trim], [Validator::Required, Validator::IsUrl])
            .enterprise_feature()
            .build()
            .new_field("allow-invalid-certs")
            .label("允许无效证书")
            .help(concat!(
                "Whether Stalwart should connect to an ",
                "endpoint that has an invalid TLS certificate"
            ))
            .default("false")
            .typ(Type::Boolean)
            .input_check([], [Validator::Required])
            .enterprise_feature()
            .build()
            .new_field("timeout")
            .label("超时")
            .help(concat!(
                "Maximum amount of time that Stalwart will wait for a response ",
                "from this endpoint"
            ))
            .default("2m")
            .typ(Type::Duration)
            .input_check([], [Validator::Required])
            .enterprise_feature()
            .build()
            .new_field("auth.token")
            .label("API 令牌")
            .help(concat!(
                "The API token used to authenticate with the AI model endpoint"
            ))
            .typ(Type::Secret)
            .enterprise_feature()
            .build()
            .new_field("headers")
            .typ(Type::Array(ArrayType::Text))
            .label("HTTP 头部")
            .help("The headers to be sent with requests")
            .enterprise_feature()
            .build()
            .new_field("default-temperature")
            .label("温度")
            .help(concat!(
                "The temperature of the AI model, which controls the randomness ",
                "of the output. A higher temperature will produce more random output."
            ))
            .typ(Type::Input)
            .default("0.7")
            .input_check(
                [Transformer::Trim],
                [
                    Validator::MinValue(NumberType::Float(0.0)),
                    Validator::MaxValue(NumberType::Float(1.0)),
                ],
            )
            .enterprise_feature()
            .build()
            .new_field("model")
            .label("模型")
            .help(concat!("The name of the AI model to use.",))
            .typ(Type::Input)
            .placeholder("gpt-4")
            .input_check([Transformer::Trim], [Validator::Required])
            .enterprise_feature()
            .build()
            .new_field("type")
            .label("类型")
            .help("API type")
            .typ(Type::Select {
                typ: SelectType::Single,
                source: Source::Static(&[("chat", "Chat Completion"), ("text", "Text Generation")]),
            })
            .default("chat")
            .enterprise_feature()
            .build()
            .new_form_section()
            .title("AI 端点设置")
            .fields(["_id", "url", "allow-invalid-certs"])
            .build()
            .new_form_section()
            .title("模型")
            .fields(["type", "model"])
            .build()
            .new_form_section()
            .title("认证")
            .fields(["auth.token"])
            .build()
            .new_form_section()
            .title("选项")
            .fields(["timeout", "headers"])
            .build()
            .list_title("AI Models")
            .list_subtitle("Manage AI Models")
            .list_fields(["_id", "model", "type"])
            .build()
    }
}

impl Builder<Schemas, Schema> {
    pub fn add_network_fields(self, is_listener: bool) -> Self {
        let do_override: &'static [&'static str] =
            if is_listener { &["true"][..] } else { &[][..] };

        // Proxy networks
        self.new_field(if is_listener {
            "proxy.trusted-networks"
        } else {
            "server.proxy.trusted-networks"
        })
        .label("代理网络")
        .help("Enable proxy protocol for connections from these networks")
        .typ(Type::Array(ArrayType::Text))
        .input_check([Transformer::Trim], [Validator::IsIpOrMask])
        .display_if_eq("proxy.override", do_override.iter().copied())
        .build()
        // Socket options
        // Backlog
        .new_field(if is_listener {
            "socket.backlog"
        } else {
            "server.socket.backlog"
        })
        .label("积压")
        .help(concat!(
            "The maximum number of incoming connections ",
            "that can be pending in the backlog queue"
        ))
        .default("1024")
        .typ(Type::Input)
        .input_check([Transformer::Trim], vec![Validator::MinValue(1.into())])
        .display_if_eq("socket.override", do_override.iter().copied())
        .build()
        // TTL
        .new_field(if is_listener {
            "socket.ttl"
        } else {
            "server.socket.ttl"
        })
        .label("TTL")
        .help(concat!(
            "Time-to-live (TTL) value for the socket, which determines how ",
            "many hops a packet can make before it is discarded"
        ))
        .typ(Type::Input)
        .input_check([Transformer::Trim], vec![Validator::MinValue(1.into())])
        .display_if_eq("socket.override", do_override.iter().copied())
        .build()
        // Linger
        .new_field(if is_listener {
            "socket.linger"
        } else {
            "server.socket.linger"
        })
        .label("延迟关闭")
        .help(concat!(
            "The time to wait before closing a socket when ",
            "there is still unsent data"
        ))
        .typ(Type::Duration)
        .display_if_eq("socket.override", do_override.iter().copied())
        .build()
        // ToS
        .new_field(if is_listener {
            "socket.tos"
        } else {
            "server.socket.tos"
        })
        .label("服务类型")
        .help(concat!(
            "The type of service (TOS) value for the socket, ",
            "which determines the priority of the traffic sent through the socket"
        ))
        .typ(Type::Input)
        .input_check([Transformer::Trim], [Validator::MinValue(1.into())])
        .display_if_eq("socket.override", do_override.iter().copied())
        .build()
        // Send buf size
        .new_field(if is_listener {
            "socket.send-buffer-size"
        } else {
            "server.socket.send-buffer-size"
        })
        .label("发送缓冲区")
        .help("The size of the buffer used for sending data")
        .typ(Type::Input)
        .input_check([Transformer::Trim], [Validator::MinValue(1.into())])
        .display_if_eq("socket.override", do_override.iter().copied())
        .build()
        // Receive buf size
        .new_field(if is_listener {
            "socket.recv-buffer-size"
        } else {
            "server.socket.recv-buffer-size"
        })
        .label("接收缓冲区")
        .help("The size of the buffer used for receiving data")
        .default("")
        .typ(Type::Input)
        .input_check([Transformer::Trim], [Validator::MinValue(1.into())])
        .display_if_eq("socket.override", do_override.iter().copied())
        .build()
        // No delay
        .new_field(if is_listener {
            "socket.nodelay"
        } else {
            "server.socket.nodelay"
        })
        .label("无延迟")
        .help("Whether the Nagle algorithm should be disabled for the socket")
        .default("true")
        .typ(Type::Boolean)
        .input_check([], [Validator::Required])
        .display_if_eq("socket.override", do_override.iter().copied())
        .build()
        // Reuse addr
        .new_field(if is_listener {
            "socket.reuse-addr"
        } else {
            "server.socket.reuse-addr"
        })
        .label("地址复用")
        .help(concat!(
            "Whether the socket can be bound to an address that ",
            "is already in use by another socket"
        ))
        .default("true")
        .typ(Type::Boolean)
        .input_check([], [Validator::Required])
        .display_if_eq("socket.override", do_override.iter().copied())
        .build()
        // Reuse port
        .new_field(if is_listener {
            "socket.reuse-port"
        } else {
            "server.socket.reuse-port"
        })
        .label("端口复用")
        .help("Whether multiple sockets can be bound to the same address and port")
        .default("true")
        .typ(Type::Boolean)
        .input_check([], [Validator::Required])
        .display_if_eq("socket.override", do_override.iter().copied())
        .build()
    }
}
