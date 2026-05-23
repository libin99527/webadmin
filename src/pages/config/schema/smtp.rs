/*
 * SPDX-FileCopyrightText: 2020 Stalwart Labs LLC <hello@stalw.art>
 *
 * SPDX-License-Identifier: AGPL-3.0-only OR LicenseRef-SEL
 */

use crate::core::{form::Expression, schema::*};

use super::*;

impl Builder<Schemas, ()> {
    #![allow(clippy::useless_concat)]
    pub fn build_smtp_outbound(self) -> Self {
        const REQUIRE_OPTIONAL: &[(&str, &str)] = &[
            ("optional", "Optional"),
            ("require", "Required"),
            ("disable", "Disabled"),
        ];

        let rcpt_vars = ExpressionValidator::new(SMTP_QUEUE_RCPT_VARS, &[]);
        let host_vars = ExpressionValidator::new(SMTP_QUEUE_HOST_VARS, &[]);

        // Strategies
        self.new_schema("smtp-out-strategy")
            .new_field("queue.strategy.route")
            .label("路由")
            .help(concat!(
                "An expression that returns the route name to use ",
                "when delivering queued messages"
            ))
            .default(Expression::new(
                [("is_local_domain('*', rcpt_domain)", "'local'")],
                "'mx'",
            ))
            .typ(Type::Expression)
            .input_check(
                [],
                [Validator::Required, Validator::IsValidExpression(host_vars)],
            )
            .new_field("queue.strategy.schedule")
            .label("调度")
            .help(concat!(
                "An expression that returns the scheduling strategy to use ",
                "when queueing messages"
            ))
            .default(Expression::new(
                [
                    ("is_local_domain('*', rcpt_domain)", "'local'"),
                    ("source == 'dsn'", "'dsn'"),
                    ("source == 'report'", "'report'"),
                ],
                "'remote'",
            ))
            .input_check(
                [],
                [Validator::Required, Validator::IsValidExpression(rcpt_vars)],
            )
            .new_field("queue.strategy.connection")
            .label("连接")
            .help(concat!(
                "An expression that returns the connection strategy to use ",
                "when delivering messages to remote SMTP servers"
            ))
            .default("'default'")
            .build()
            .new_field("queue.strategy.tls")
            .label("TLS")
            .typ(Type::Expression)
            .help(concat!(
                "An expression that returns the TLS strategy to use ",
                "when delivering messages to remote SMTP servers"
            ))
            .default(Expression::new(
                [("retry_num > 0 && last_error == 'tls'", "'invalid-tls'")],
                "'default'",
            ))
            .input_check(
                [],
                [Validator::Required, Validator::IsValidExpression(rcpt_vars)],
            )
            .build()
            .new_form_section()
            .title("出站策略")
            .fields([
                "queue.strategy.route",
                "queue.strategy.schedule",
                "queue.strategy.connection",
                "queue.strategy.tls",
            ])
            .build()
            .build()
            // Resolver
            .new_schema("smtp-out-resolver")
            .new_field("resolver.type")
            .label("解析器")
            .help(concat!("Resolver to use for DNS resolution"))
            .default("system")
            .typ(Type::Select {
                typ: SelectType::Single,
                source: Source::Static(&[
                    ("system", "System Resolver"),
                    ("custom", "Custom DNS"),
                    ("cloudflare", "Cloudflare DNS "),
                    ("cloudflare-tls", "Cloudflare DNS (TLS)"),
                    ("quad9", "Quad9 DNS"),
                    ("quad9-tls", "Quad9 DNS (TLS)"),
                    ("google", "Google DNS"),
                ]),
            })
            .input_check([], [Validator::Required])
            .build()
            .new_field("resolver.custom")
            .label("DNS 服务器")
            .help(concat!(
                "List of custom DNS server URLs to use for resolution"
            ))
            .default("udp://127.0.0.1:53")
            .typ(Type::Array(ArrayType::Text))
            .input_check([], [Validator::Required])
            .display_if_eq("resolver.type", ["custom"])
            .build()
            .new_field("resolver.preserve-intermediates")
            .label("保留中间证书")
            .help(concat!(
                "Whether to preserve the intermediate name servers in the ",
                "DNS resolution results"
            ))
            .default("true")
            .typ(Type::Boolean)
            .input_check([], [Validator::Required])
            .build()
            .new_field("resolver.edns")
            .label("启用 EDNS")
            .help(concat!(
                "Whether to enable EDNS (Extension Mechanisms for DNS) support"
            ))
            .default("true")
            .typ(Type::Boolean)
            .input_check([], [Validator::Required])
            .build()
            .new_field("resolver.concurrency")
            .label("并发请求")
            .help(concat!(
                "Number of concurrent resolution requests that can be made ",
                "at the same time"
            ))
            .default("2")
            .typ(Type::Input)
            .input_check([], [Validator::Required])
            .build()
            .new_field("resolver.timeout")
            .label("超时")
            .help(concat!(
                "Time after which a resolution request will be timed out if ",
                "no response is received"
            ))
            .default("5s")
            .typ(Type::Duration)
            .input_check([], [Validator::Required])
            .build()
            .new_field("resolver.attempts")
            .label("最大尝试次数")
            .help(concat!(
                "Number of times a resolution request will be retried before ",
                "it is considered failed"
            ))
            .default("2")
            .typ(Type::Input)
            .input_check([], [Validator::Required])
            .build()
            .new_field("resolver.try-tcp-on-error")
            .label("错误时尝试 TCP")
            .help(concat!(
                "Whether to try using TCP for resolution requests if an error ",
                "occurs during a UDP resolution request"
            ))
            .default("true")
            .typ(Type::Boolean)
            .input_check([], [Validator::Required])
            .build()
            .new_form_section()
            .title("DNS 解析器设置")
            .fields([
                "resolver.type",
                "resolver.custom",
                "resolver.concurrency",
                "resolver.timeout",
                "resolver.attempts",
                "resolver.preserve-intermediates",
                "resolver.try-tcp-on-error",
                "resolver.edns",
            ])
            .build()
            .build()
            // Routing strategies
            .new_schema("smtp-out-routing")
            .prefix("queue.route")
            .suffix("type")
            .names("route", "routes")
            .new_id_field()
            .label("ID")
            .help("Unique identifier for the route")
            .build()
            .new_field("type")
            .readonly()
            .label("类型")
            .help("Route type")
            .default("mx")
            .typ(Type::Select {
                source: Source::Static(&[
                    ("local", "Local Delivery"),
                    ("mx", "Remote Delivery (MX)"),
                    ("relay", "Relay Host"),
                ]),
                typ: SelectType::Single,
            })
            .build()
            .new_field("description")
            .label("描述")
            .help(concat!(
                "A short description of the route, which can be used to ",
                "identify it in the list of routes"
            ))
            .typ(Type::Input)
            .placeholder("Route description")
            .build()
            .new_field("ip-lookup")
            .display_if_eq("type", ["mx"])
            .label("IP 解析")
            .help("IP resolution strategy for MX hosts")
            .default("ipv4_then_ipv6")
            .typ(Type::Select {
                source: Source::Static(&[
                    ("ipv4_then_ipv6", "IPv4 then IPv6"),
                    ("ipv6_then_ipv4", "IPv6 then IPv4"),
                    ("ipv4_only", "IPv4 Only"),
                    ("ipv6_only", "IPv6 Only"),
                ]),
                typ: SelectType::Single,
            })
            .build()
            .new_field("limits.mx")
            .display_if_eq("type", ["mx"])
            .label("MX 主机")
            .help(concat!(
                "Maximum number of MX hosts to try on each delivery attempt"
            ))
            .typ(Type::Input)
            .input_check([], [Validator::Required, Validator::MinValue(1i64.into())])
            .default("5")
            .build()
            .new_field("limits.multihomed")
            .display_if_eq("type", ["mx"])
            .label("多宿主 IP")
            .help(concat!(
                "For multi-homed remote servers, it is the maximum number of ",
                "IP addresses to try on each delivery attempt"
            ))
            .typ(Type::Input)
            .input_check([], [Validator::Required, Validator::MinValue(1i64.into())])
            .default("2")
            .build()
            .new_field("address")
            .display_if_eq("type", ["relay"])
            .label("地址")
            .help(concat!(
                "The address of the remote SMTP server, which can be an IP ",
                "address or a domain name"
            ))
            .typ(Type::Input)
            .input_check([], [Validator::Required, Validator::IsHost])
            .placeholder("127.0.0.1")
            .build()
            .new_field("port")
            .display_if_eq("type", ["relay"])
            .label("端口")
            .help(concat!(
                "The port number of the remote server, which is typically ",
                "25 for SMTP and 11200 for LMTP"
            ))
            .typ(Type::Input)
            .input_check([], [Validator::Required, Validator::IsPort])
            .placeholder("25")
            .build()
            .new_field("protocol")
            .display_if_eq("type", ["relay"])
            .label("协议")
            .help(concat!(
                "The protocol to use when delivering messages to the remote ",
                "server, which can be either SMTP or LMTP"
            ))
            .typ(Type::Select {
                typ: SelectType::Single,
                source: Source::Static(&[("smtp", "SMTP"), ("lmtp", "LMTP")]),
            })
            .default("smtp")
            .build()
            .new_field("tls.implicit")
            .display_if_eq("type", ["relay"])
            .label("隐式 TLS")
            .help(concat!(
                "Whether to use TLS encryption for all connections to the remote ",
                "server"
            ))
            .typ(Type::Boolean)
            .default("false")
            .build()
            .new_field("tls.allow-invalid-certs")
            .display_if_eq("type", ["relay"])
            .label("允许无效证书")
            .help(concat!(
                "Whether to allow connections to servers with invalid TLS certificates"
            ))
            .typ(Type::Boolean)
            .default("false")
            .build()
            .new_field("auth.username")
            .display_if_eq("type", ["relay"])
            .label("用户名")
            .help(concat!(
                "The username to use when authenticating with the remote server"
            ))
            .typ(Type::Input)
            .build()
            .new_field("auth.secret")
            .display_if_eq("type", ["relay"])
            .label("密钥")
            .help(concat!(
                "The secret to use when authenticating with the remote server"
            ))
            .typ(Type::Secret)
            .build()
            .new_form_section()
            .title("路由配置")
            .fields(["_id", "type", "description"])
            .build()
            .new_form_section()
            .title("MX 解析")
            .display_if_eq("type", ["mx"])
            .fields(["ip-lookup", "limits.mx", "limits.multihomed"])
            .build()
            .new_form_section()
            .title("服务器详情")
            .display_if_eq("type", ["relay"])
            .fields(["address", "port", "protocol"])
            .build()
            .new_form_section()
            .title("TLS")
            .display_if_eq("type", ["relay"])
            .fields(["tls.implicit", "tls.allow-invalid-certs"])
            .build()
            .new_form_section()
            .title("认证")
            .display_if_eq("type", ["relay"])
            .fields(["auth.username", "auth.secret"])
            .build()
            .list_title("Routes")
            .list_subtitle("Manage routes for message delivery")
            .list_fields(["_id", "type", "description"])
            .build()
            // Virtual queues
            .new_schema("smtp-out-queues")
            .prefix("queue.virtual")
            .suffix("threads-per-node")
            .names("queue", "queues")
            .new_id_field()
            .label("名称")
            .help("Unique identifier for the queue, max 8 characters")
            .input_check(
                [Transformer::Trim],
                [
                    Validator::Required,
                    Validator::IsId,
                    Validator::MaxLength(8),
                ],
            )
            .build()
            .new_field("threads-per-node")
            .label("投递线程数")
            .help(concat!(
                "Maximum number of threads to use for  delivery ",
                "on each node in the cluster"
            ))
            .typ(Type::Input)
            .input_check([], [Validator::Required, Validator::MinValue(1i64.into())])
            .default("25")
            .build()
            .new_field("description")
            .label("描述")
            .help(concat!(
                "A short description of the queue, which can be used to ",
                "identify it in the list of queues"
            ))
            .typ(Type::Input)
            .placeholder("Queue description")
            .build()
            .new_form_section()
            .title("虚拟队列")
            .fields(["_id", "description", "threads-per-node"])
            .build()
            .list_title("Virtual Queues")
            .list_subtitle("Manage virtual queues for message delivery")
            .list_fields(["_id", "threads-per-node", "description"])
            .build()
            // Scheduling
            .new_schema("smtp-out-scheduling")
            .prefix("queue.schedule")
            .suffix("queue-name")
            .names("schedule", "schedules")
            .new_id_field()
            .label("名称")
            .help("Unique identifier for the schedule")
            .build()
            .new_field("queue-name")
            .label("虚拟队列")
            .help(concat!(
                "The name of the virtual queue to use for this schedule"
            ))
            .typ(Type::Select {
                source: Source::Dynamic {
                    schema: "smtp-out-queues",
                    field: "description",
                    filter: Default::default(),
                },
                typ: SelectType::Single,
            })
            .input_check([], [Validator::Required])
            .build()
            .new_field("description")
            .label("描述")
            .help(concat!(
                "A short description of the schedule, which can be used to ",
                "identify it in the list of schedules"
            ))
            .typ(Type::Input)
            .placeholder("Schedule description")
            .build()
            .new_field("retry")
            .label("重试间隔")
            .help(concat!("List of retry intervals for message delivery"))
            .default(&["2m", "5m", "10m", "15m", "30m", "1h", "2h"][..])
            .typ(Type::Array(ArrayType::Duration))
            .input_check([], [Validator::Required])
            .build()
            .new_field("notify")
            .label("通知间隔")
            .help(concat!(
                "List of delayed delivery DSN notification intervals"
            ))
            .typ(Type::Array(ArrayType::Duration))
            .build()
            .new_field("expire-type")
            .label("过期策略")
            .help(concat!(
                "Whether to expire messages after a number of delivery ",
                "attempts or after certain time (TTL)"
            ))
            .default("ttl")
            .typ(Type::Select {
                source: Source::Static(&[
                    ("ttl", "Time To Live"),
                    ("attempts", "Delivery Attempts"),
                ]),
                typ: SelectType::Single,
            })
            .build()
            .new_field("expire")
            .display_if_eq("expire-type", ["ttl"])
            .label("生存时间")
            .help(concat!(
                "Time after which the message will be expired if it is not ",
                "delivered"
            ))
            .typ(Type::Duration)
            .input_check([], [Validator::Required])
            .default("3d")
            .build()
            .new_field("max-attempts")
            .display_if_eq("expire-type", ["attempts"])
            .label("最大尝试次数")
            .help(concat!(
                "Maximum number of delivery attempts before the message is ",
                "considered failed"
            ))
            .typ(Type::Input)
            .input_check([], [Validator::Required, Validator::MinValue(1i64.into())])
            .default("5")
            .build()
            .new_form_section()
            .title("调度详情")
            .fields(["_id", "queue-name", "description"])
            .build()
            .new_form_section()
            .title("投递重试间隔")
            .fields(["retry"])
            .build()
            .new_form_section()
            .title("延迟投递通知")
            .fields(["notify"])
            .build()
            .new_form_section()
            .title("消息过期")
            .fields(["expire-type", "expire", "max-attempts"])
            .build()
            .list_title("Schedules")
            .list_subtitle("Manage schedules for message delivery")
            .list_fields(["_id", "queue-name", "description"])
            .build()
            // TLS strategies
            .new_schema("smtp-out-tls")
            .prefix("queue.tls")
            .suffix("allow-invalid-certs")
            .names("TLS strategy", "TLS strategies")
            .new_id_field()
            .label("名称")
            .help("Unique identifier for the TLS strategy")
            .build()
            .new_field("dane")
            .label("DANE")
            .help(concat!("Whether DANE is required, optional, or disabled"))
            .default("optional")
            .typ(Type::Select {
                typ: SelectType::Single,
                source: Source::Static(REQUIRE_OPTIONAL),
            })
            .input_check([], [Validator::Required])
            .build()
            .new_field("mta-sts")
            .label("MTA-STS")
            .help(concat!(
                "Whether MTA-STS is required, optional, or disabled"
            ))
            .default("optional")
            .typ(Type::Select {
                typ: SelectType::Single,
                source: Source::Static(REQUIRE_OPTIONAL),
            })
            .input_check([], [Validator::Required])
            .build()
            .new_field("starttls")
            .label("STARTTLS")
            .help(concat!(
                "Whether TLS support is required, optional, or disabled"
            ))
            .default("optional")
            .typ(Type::Select {
                typ: SelectType::Single,
                source: Source::Static(REQUIRE_OPTIONAL),
            })
            .input_check([], [Validator::Required])
            .build()
            .new_field("allow-invalid-certs")
            .label("允许无效证书")
            .help(concat!(
                "Whether to allow connections to servers with invalid TLS certificates"
            ))
            .default("false")
            .typ(Type::Boolean)
            .build()
            .new_field("timeout.tls")
            .label("TLS")
            .help(concat!(
                "Maximum time to wait for the TLS handshake to complete"
            ))
            .default("3m")
            .typ(Type::Duration)
            .input_check([], [Validator::Required])
            .build()
            .new_field("timeout.mta-sts")
            .label("MTA-STS")
            .help(concat!(
                "Maximum time to wait for the MTA-STS policy lookup to complete"
            ))
            .default("5m")
            .typ(Type::Duration)
            .input_check([], [Validator::Required])
            .build()
            .new_field("description")
            .label("描述")
            .help(concat!(
                "A short description of the TLS strategy, which can be used to ",
                "identify it in the list of strategies"
            ))
            .typ(Type::Input)
            .placeholder("TLS Strategy description")
            .build()
            .new_form_section()
            .title("TLS 策略")
            .fields(["_id", "description"])
            .build()
            .new_form_section()
            .title("安全要求")
            .fields(["dane", "mta-sts", "starttls", "allow-invalid-certs"])
            .build()
            .new_form_section()
            .title("超时")
            .fields(["timeout.tls", "timeout.mta-sts"])
            .build()
            .list_title("TLS Strategies")
            .list_subtitle("Manage TLS strategies for message delivery")
            .list_fields(["_id", "description"])
            .build()
            // Connection strategies
            .new_schema("smtp-out-connection")
            .prefix("queue.connection")
            .suffix("timeout.connect")
            .names("Connection strategy", "Connection strategies")
            .new_id_field()
            .label("名称")
            .help("Unique identifier for the connection strategy")
            .build()
            .new_field("source-ips")
            .label("源 IP")
            .help(concat!(
                "List of local IPv4 and IPv6 addresses to use when ",
                "delivering emails to remote SMTP servers"
            ))
            .typ(Type::Array(ArrayType::Text))
            .input_check([], [Validator::IsIpOrMask])
            .build()
            .new_field("ehlo-hostname")
            .label("EHLO 主机名")
            .help(concat!(
                "Overrides the EHLO hostname that will be used when ",
                "connecting to remote SMTP servers"
            ))
            .typ(Type::Input)
            .input_check([], [Validator::IsHost])
            .placeholder("mail.example.com")
            .build()
            .new_field("timeout.connect")
            .label("连接")
            .help(concat!(
                "Maximum time to wait for the connection to be established"
            ))
            .default("5m")
            .typ(Type::Duration)
            .input_check([], [Validator::Required])
            .build()
            .new_field("timeout.greeting")
            .label("问候语")
            .help(concat!(
                "Maximum time to wait for the SMTP greeting message"
            ))
            .default("5m")
            .typ(Type::Duration)
            .input_check([], [Validator::Required])
            .build()
            .new_field("timeout.ehlo")
            .label("EHLO")
            .help(concat!(
                "Maximum time to wait for the EHLO command response"
            ))
            .default("5m")
            .typ(Type::Duration)
            .input_check([], [Validator::Required])
            .build()
            .new_field("timeout.mail-from")
            .label("MAIL-FROM")
            .help(concat!(
                "Maximum time to wait for the MAIL-FROM command response"
            ))
            .default("5m")
            .typ(Type::Duration)
            .input_check([], [Validator::Required])
            .build()
            .new_field("timeout.rcpt-to")
            .label("RCPT-TO")
            .help(concat!(
                "Maximum time to wait for the RCPT-TO command response"
            ))
            .default("5m")
            .typ(Type::Duration)
            .input_check([], [Validator::Required])
            .build()
            .new_field("timeout.data")
            .label("DATA")
            .help(concat!(
                "Maximum time to wait for the DATA command response"
            ))
            .default("10m")
            .typ(Type::Duration)
            .input_check([], [Validator::Required])
            .build()
            .new_field("description")
            .label("描述")
            .help("Short description of the connection strategy")
            .typ(Type::Input)
            .build()
            .new_form_section()
            .title("连接策略")
            .fields(["_id", "description", "ehlo-hostname"])
            .build()
            .new_form_section()
            .title("超时")
            .fields([
                "timeout.connect",
                "timeout.greeting",
                "timeout.ehlo",
                "timeout.mail-from",
                "timeout.rcpt-to",
                "timeout.data",
            ])
            .build()
            .new_form_section()
            .title("源 IP 地址")
            .fields(["source-ips"])
            .build()
            .list_title("Connection Strategies")
            .list_subtitle("Manage connection strategies for message delivery")
            .list_fields(["_id", "description"])
            .build()
            // Outbound rate limiter
            .new_schema("smtp-out-throttle")
            .prefix("queue.limiter.outbound")
            .names("throttle", "throttles")
            .suffix("enable")
            .new_id_field()
            .label("限制器 ID")
            .help("Unique identifier for the throttle")
            .build()
            .new_field("enable")
            .label("已启用")
            .help("Whether to enable this throttle")
            .typ(Type::Boolean)
            .default("true")
            .build()
            .new_field("key")
            .label("密钥")
            .help(concat!(
                "Optional list of context variables that determine ",
                "where this throttle should be applied"
            ))
            .typ(Type::Select {
                typ: SelectType::Many,
                source: Source::Static(&[
                    (V_MX, "MX Host"),
                    (V_REMOTE_IP, "Remote IP"),
                    (V_LOCAL_IP, "Local IP"),
                    (V_SENDER, "Sender"),
                    (V_SENDER_DOMAIN, "Sender Domain"),
                    (V_RECIPIENT_DOMAIN, "Recipient Domain"),
                ]),
            })
            .build()
            .new_field("match")
            .label("匹配条件")
            .help(concat!(
                "Enable the imposition of concurrency and rate limits only ",
                "when a specific condition is met"
            ))
            .typ(Type::Expression)
            .input_check(
                [],
                [
                    Validator::IsValidExpression(ExpressionValidator::new(
                        SMTP_QUEUE_HOST_VARS,
                        &[],
                    )),
                    Validator::MaxItems(1),
                ],
            )
            .build()
            .new_field("rate")
            .label("速率限制")
            .help(concat!(
                "Number of incoming requests over a period of time ",
                "that the rate limiter will allow"
            ))
            .input_check([], [Validator::Required])
            .typ(Type::Rate)
            .build()
            .new_form_section()
            .title("出站速率限制")
            .fields(["_id", "key", "rate", "match", "enable"])
            .build()
            .list_title("Outbound Rate Limits")
            .list_subtitle("Manage outbound rate limits")
            .list_fields(["_id", "rate", "enable"])
            .build()
            // Queue quotas
            .new_schema("smtp-out-quota")
            .prefix("queue.quota")
            .names("quota", "quotas")
            .suffix("enable")
            .new_id_field()
            .label("配额 ID")
            .help("Unique identifier for the quota")
            .build()
            .new_field("enable")
            .label("已启用")
            .help("Whether to enable this quota")
            .typ(Type::Boolean)
            .default("true")
            .build()
            .new_field("key")
            .label("密钥")
            .help(concat!(
                "Optional list of context variables that determine ",
                "where this quota should be applied"
            ))
            .typ(Type::Select {
                typ: SelectType::Many,
                source: Source::Static(&[
                    (V_SENDER, "Sender"),
                    (V_SENDER_DOMAIN, "Sender Domain"),
                    (V_RECIPIENT, "Recipient"),
                    (V_RECIPIENT_DOMAIN, "Recipient Domain"),
                ]),
            })
            .build()
            .new_field("match")
            .label("匹配条件")
            .help(concat!(
                "Enable the imposition of concurrency and rate limits only ",
                "when a specific condition is met"
            ))
            .typ(Type::Expression)
            .input_check(
                [],
                [
                    Validator::IsValidExpression(ExpressionValidator::new(
                        SMTP_QUEUE_HOST_VARS,
                        &[],
                    )),
                    Validator::MaxItems(1),
                ],
            )
            .build()
            .new_field("messages")
            .label("最大消息数")
            .help(concat!(
                "Maximum number of messages in the queue that ",
                "this quota will allow"
            ))
            .typ(Type::Input)
            .input_check([Transformer::Trim], [Validator::MinValue(1.into())])
            .build()
            .new_field("size")
            .label("最大大小")
            .help(concat!(
                "Maximum total size of messages in the queue that ",
                "this quota will allow"
            ))
            .typ(Type::Size)
            .build()
            .new_form_section()
            .title("配额")
            .fields(["_id", "key", "messages", "size", "match", "enable"])
            .build()
            .list_title("Quota Queues")
            .list_subtitle("Manage quotas on message queues")
            .list_fields(["_id", "messages", "size", "enable"])
            .build()
    }

    pub fn build_smtp_inbound(self) -> Self {
        let has_conn_vars = ExpressionValidator::new(CONNECTION_VARS, &[]);
        let has_ehlo_hars = ExpressionValidator::new(SMTP_EHLO_VARS, &[]);
        let has_sender_vars = ExpressionValidator::new(SMTP_MAIL_FROM_VARS, &[]);
        let has_rcpt_vars = ExpressionValidator::new(SMTP_RCPT_TO_VARS, &[]);

        // Connect
        self.new_schema("smtp-in-connect")
            .new_field("session.connect.script")
            .typ(Type::Expression)
            .label("运行脚本")
            .help("Which Sieve script to run when a client connects")
            .input_check([], [Validator::IsValidExpression(has_conn_vars)])
            .new_field("session.connect.greeting")
            .label("SMTP 问候语")
            .help("The greeting message sent by the SMTP/LMTP server")
            .default("config_get('server.hostname') + ' Stalwart ESMTP at your service'")
            .new_field("session.connect.hostname")
            .label("服务器主机名")
            .help("The SMTP server hostname")
            .input_check(
                [],
                [
                    Validator::Required,
                    Validator::IsValidExpression(has_conn_vars),
                ],
            )
            .default("config_get('server.hostname')")
            .build()
            .new_field("auth.iprev.verify")
            .typ(Type::Expression)
            .label("IPRev 验证")
            .help("How strict to be when verifying the reverse DNS of the client IP")
            .input_check(
                [],
                [
                    Validator::Required,
                    Validator::IsValidExpression(has_conn_vars.constants(VERIFY_CONSTANTS)),
                ],
            )
            .default(Expression::new(
                [("local_port == 25", "relaxed")],
                "disable",
            ))
            .build()
            .new_form_section()
            .title("连接阶段")
            .fields([
                "session.connect.hostname",
                "session.connect.greeting",
                "session.connect.script",
                "auth.iprev.verify",
            ])
            .build()
            .build()
            // EHLO stage
            .new_schema("smtp-in-ehlo")
            .new_field("session.ehlo.require")
            .label("要求 EHLO")
            .typ(Type::Expression)
            .input_check(
                [],
                [
                    Validator::Required,
                    Validator::IsValidExpression(has_conn_vars),
                ],
            )
            .default("true")
            .help(concat!(
                "Whether the remote client must send an EHLO command ",
                "before starting an SMTP transaction"
            ))
            .build()
            .new_field("session.ehlo.reject-non-fqdn")
            .label("拒绝非 FQDN")
            .typ(Type::Expression)
            .input_check(
                [],
                [
                    Validator::Required,
                    Validator::IsValidExpression(has_conn_vars),
                ],
            )
            .help(concat!(
                "Whether to reject EHLO commands that do not include a ",
                "fully-qualified domain name as a parameter"
            ))
            .default(Expression::new([("local_port == 25", "true")], "false"))
            .build()
            .new_field("session.ehlo.script")
            .label("运行脚本")
            .typ(Type::Expression)
            .input_check([], [Validator::IsValidExpression(has_conn_vars)])
            .help("Which Sieve script to run after the client sends an EHLO command")
            .build()
            .new_form_section()
            .title("EHLO 阶段")
            .fields([
                "session.ehlo.require",
                "session.ehlo.reject-non-fqdn",
                "session.ehlo.script",
            ])
            .build()
            .build()
            // Limits
            .new_schema("smtp-in-limits")
            .new_field("session.timeout")
            .label("超时")
            .typ(Type::Expression)
            .input_check(
                [],
                [
                    Validator::Required,
                    Validator::IsValidExpression(has_conn_vars),
                ],
            )
            .default("5m")
            .help("How long to wait for a client to send a command before timing out")
            .new_field("session.transfer-limit")
            .label("字节限制")
            .default("262144000")
            .help("The maximum number of bytes that can be transferred per session")
            .new_field("session.duration")
            .label("持续时间")
            .default("10m")
            .help("The maximum duration of a session")
            .build()
            .new_form_section()
            .title("SMTP 会话限制")
            .fields([
                "session.timeout",
                "session.transfer-limit",
                "session.duration",
            ])
            .build()
            .build()
            // Extensions
            .new_schema("smtp-in-extensions")
            .new_field("session.extensions.pipelining")
            .label("管道化")
            .typ(Type::Expression)
            .input_check(
                [],
                [
                    Validator::Required,
                    Validator::IsValidExpression(has_sender_vars),
                ],
            )
            .default("true")
            .help(concat!(
                "Enables SMTP pipelining (RFC 2920), which enables multiple ",
                "commands to be sent in a single request to speed up communication ",
                "between the client and server"
            ))
            .new_field("session.extensions.chunking")
            .label("分块")
            .help(concat!(
                "Enables chunking (RFC 1830), an extension that allows large ",
                "messages to be transferred in chunks which may reduce the load ",
                "on the network and server."
            ))
            .default("true")
            .new_field("session.extensions.requiretls")
            .label("要求 TLS")
            .help(concat!(
                "Enables require TLS (RFC 8689), an extension that allows",
                " clients to require TLS encryption for the SMTP session"
            ))
            .default("true")
            .new_field("session.extensions.no-soliciting")
            .label("禁止推销")
            .help(concat!(
                "Specifies the text to include in the NOSOLICITING (RFC 3865) ",
                "message, which indicates that the server does not accept unsolicited ",
                "commercial email (UCE or spam)"
            ))
            .default("\"\"")
            .new_field("session.extensions.dsn")
            .label("DSN")
            .help(concat!(
                "Enables delivery status notifications (RFC 3461), which allows ",
                "the sender to request a delivery status notification (DSN) from ",
                "the recipient's mail server"
            ))
            .default(Expression::new(
                [("!is_empty(authenticated_as)", "true")],
                "false",
            ))
            .new_field("session.extensions.expn")
            .label("EXPN")
            .help(concat!(
                "Specifies whether to enable the EXPN command, which allows ",
                "the sender to request the membership of a mailing list. It is ",
                "recommended to disable this command to prevent spammers ",
                "from harvesting email addresses"
            ))
            .default(Expression::new(
                [("!is_empty(authenticated_as)", "true")],
                "false",
            ))
            .new_field("session.extensions.vrfy")
            .label("VRFY")
            .help(concat!(
                "Specifies whether to enable the VRFY command, which allows ",
                "the sender to verify the existence of a mailbox. It is recommended ",
                "to disable this command to prevent spammers from ",
                "harvesting email addresses"
            ))
            .default(Expression::new(
                [("!is_empty(authenticated_as)", "true")],
                "false",
            ))
            .new_field("session.extensions.future-release")
            .label("延迟发送")
            .help(concat!(
                "Specifies the maximum time that a message can be held for ",
                "delivery using the FUTURERELEASE (RFC 4865) extension"
            ))
            .default(Expression::new(
                [("!is_empty(authenticated_as)", "7d")],
                "false",
            ))
            .new_field("session.extensions.deliver-by")
            .label("投递期限")
            .help(concat!(
                "Specifies the maximum delivery time for a message using the ",
                "DELIVERBY (RFC 2852) extension, which allows the sender to request ",
                "a specific delivery time for a message"
            ))
            .default(Expression::new(
                [("!is_empty(authenticated_as)", "15d")],
                "false",
            ))
            .new_field("session.extensions.mt-priority")
            .label("MT 优先级")
            .help(concat!(
                "Specifies the priority assignment policy to advertise on the ",
                "MT-PRIORITY (RFC 6710) extension, which allows the sender to specify ",
                "a priority for a message. Available policies are mixer, stanag4406 a",
                "nd nsep, or false to disable this extension"
            ))
            .input_check(
                [],
                [
                    Validator::Required,
                    Validator::IsValidExpression(has_sender_vars.constants(&[
                        "mixer",
                        "stanag4406",
                        "nsep",
                    ])),
                ],
            )
            .default(Expression::new(
                [("!is_empty(authenticated_as)", "mixer")],
                "false",
            ))
            .build()
            .new_form_section()
            .title("SMTP 扩展")
            .fields([
                "session.extensions.pipelining",
                "session.extensions.chunking",
                "session.extensions.requiretls",
                "session.extensions.no-soliciting",
                "session.extensions.dsn",
                "session.extensions.expn",
                "session.extensions.vrfy",
                "session.extensions.future-release",
                "session.extensions.deliver-by",
                "session.extensions.mt-priority",
            ])
            .build()
            .build()
            // AUTH stage
            .new_schema("smtp-in-auth")
            .new_field("session.auth.require")
            .label("要求认证")
            .help(concat!(
                "Specifies whether authentication is necessary to send email messages"
            ))
            .typ(Type::Expression)
            .input_check(
                [],
                [
                    Validator::Required,
                    Validator::IsValidExpression(has_ehlo_hars),
                ],
            )
            .default(Expression::new([("local_port != 25", "true")], "false"))
            .new_field("session.auth.must-match-sender")
            .label("必须匹配发件人")
            .help(concat!(
                "Specifies whether the authenticated user or any of their associated ",
                "e-mail addresses must match the sender of the email message"
            ))
            .default("true")
            .typ(Type::Expression)
            .input_check(
                [],
                [
                    Validator::Required,
                    Validator::IsValidExpression(has_sender_vars),
                ],
            )
            .new_field("session.auth.directory")
            .label("目录")
            .help("Specifies the directory to use for authentication")
            .default(Expression::new([("local_port != 25", "'*'")], "false"))
            .input_check(
                [],
                [
                    Validator::Required,
                    Validator::IsValidExpression(has_ehlo_hars),
                ],
            )
            .new_field("session.auth.errors.total")
            .label("最大错误数")
            .help(concat!(
                "Maximum number of authentication errors allowed before the session ",
                "is disconnected"
            ))
            .default("3")
            .new_field("session.auth.errors.wait")
            .label("错误等待")
            .help("Time interval to wait after an authentication failure")
            .default("5s")
            .new_field("session.auth.mechanisms")
            .label("允许的认证机制")
            .help(concat!(
                "A list of SASL authentication mechanisms offered to clients, or an ",
                "empty list to disable authentication. Stalwart SMTP currently supports PLAIN, ",
                "LOGIN, and OAUTHBEARER mechanisms"
            ))
            .default(Expression::new(
                [
                    (
                        "local_port != 25 && is_tls",
                        "[plain, login, oauthbearer, xoauth2]",
                    ),
                    ("local_port != 25", "[oauthbearer, xoauth2]"),
                ],
                "false",
            ))
            .input_check(
                [],
                [Validator::IsValidExpression(
                    has_conn_vars.constants(AUTH_CONSTANTS),
                )],
            )
            .build()
            .new_form_section()
            .title("AUTH 阶段")
            .fields([
                "session.auth.directory",
                "session.auth.require",
                "session.auth.must-match-sender",
                "session.auth.mechanisms",
            ])
            .build()
            .new_form_section()
            .title("认证错误")
            .fields(["session.auth.errors.total", "session.auth.errors.wait"])
            .build()
            .build()
            // MAIL stage
            .new_schema("smtp-in-mail")
            .new_field("session.mail.rewrite")
            .label("发件人重写")
            .help("Expression to rewrite the sender address")
            .typ(Type::Expression)
            .input_check(
                [],
                [
                    Validator::Required,
                    Validator::IsValidExpression(has_sender_vars),
                ],
            )
            .default("false")
            .new_field("session.mail.script")
            .label("运行脚本")
            .help("Which Sieve script to run after the client sends a MAIL command")
            .input_check([], [Validator::IsValidExpression(has_sender_vars)])
            .new_field("session.mail.is-allowed")
            .label("允许的发件人")
            .help("Expression that returns true when the sender is allowed to send")
            .input_check([], [Validator::IsValidExpression(has_sender_vars)])
            .default(Expression::new(
                [],
                "!is_empty(authenticated_as) || !key_exists('spam-block', sender_domain)",
            ))
            .build()
            .new_form_section()
            .title("MAIL FROM 阶段")
            .fields([
                "session.mail.rewrite",
                "session.mail.is-allowed",
                "session.mail.script",
            ])
            .build()
            .build()
            // RCPT stage
            .new_schema("smtp-in-rcpt")
            .new_field("session.rcpt.directory")
            .label("目录")
            .help("Directory to use to validate local recipients")
            .default("\"*\"")
            .typ(Type::Expression)
            .input_check(
                [],
                [
                    Validator::Required,
                    Validator::IsValidExpression(has_rcpt_vars),
                ],
            )
            .new_field("session.rcpt.relay")
            .label("允许中继")
            .help("Whether to allow relaying for non-local recipients")
            .default(Expression::new(
                [("!is_empty(authenticated_as)", "true")],
                "false",
            ))
            .new_field("session.rcpt.max-recipients")
            .label("最大收件人数")
            .help("Maximum number of recipients per message")
            .default("100")
            .new_field("session.rcpt.rewrite")
            .label("收件人重写")
            .help("Expression to rewrite the recipient address")
            .default("false")
            .new_field("session.rcpt.errors.total")
            .label("最大错误数")
            .help(concat!(
                "Maximum number of recipient errors before ",
                "the session is disconnected"
            ))
            .default("5")
            .new_field("session.rcpt.errors.wait")
            .label("错误等待")
            .help("Amount of time to wait after a recipient error")
            .default("5s")
            .new_field("session.rcpt.script")
            .label("运行脚本")
            .help("Which Sieve script to run after the client sends a RCPT command")
            .input_check([], [Validator::IsValidExpression(has_rcpt_vars)])
            .build()
            .new_field("session.rcpt.catch-all")
            .label("全部捕获")
            .help("Expression to enable catch-all address")
            .typ(Type::Expression)
            .input_check([], [Validator::IsValidExpression(has_rcpt_vars)])
            .default("true")
            .new_field("session.rcpt.sub-addressing")
            .label("子地址")
            .help("Expression to enable sub-addressing")
            .default("true")
            .build()
            .new_form_section()
            .title("RCPT TO 阶段")
            .fields([
                "session.rcpt.directory",
                "session.rcpt.relay",
                "session.rcpt.max-recipients",
                "session.rcpt.script",
            ])
            .build()
            .new_form_section()
            .title("地址处理")
            .fields([
                "session.rcpt.rewrite",
                "session.rcpt.catch-all",
                "session.rcpt.sub-addressing",
            ])
            .build()
            .new_form_section()
            .title("收件人错误")
            .fields(["session.rcpt.errors.total", "session.rcpt.errors.wait"])
            .build()
            .build()
            // DATA stage
            .new_schema("smtp-in-data")
            .new_field("session.data.script")
            .label("运行脚本")
            .help("Which Sieve script to run after the client sends a DATA command")
            .typ(Type::Expression)
            .input_check([], [Validator::IsValidExpression(has_rcpt_vars)])
            .new_field("session.data.spam-filter")
            .label("垃圾邮件过滤")
            .help("Whether to enable the spam filter for incoming messages")
            .default(Expression::new([], "true"))
            .typ(Type::Expression)
            .input_check([], [Validator::IsValidExpression(has_rcpt_vars)])
            .new_field("session.data.limits.messages")
            .label("消息")
            .help("Maximum number of messages that can be submitted per SMTP session")
            .default("10")
            .input_check(
                [],
                [
                    Validator::Required,
                    Validator::IsValidExpression(has_rcpt_vars),
                ],
            )
            .new_field("session.data.limits.size")
            .label("大小")
            .help("Maximum size of a message in bytes")
            .default("104857600")
            .new_field("session.data.limits.received-headers")
            .label("接收头部")
            .help(concat!(
                "Maximum limit on the number of Received headers, ",
                "which helps to prevent message loops"
            ))
            .default("50")
            .new_field("session.data.add-headers.received")
            .label("已接收")
            .help("Whether to add a Received header to the message")
            .default(Expression::new([("local_port == 25", "true")], "false"))
            .new_field("session.data.add-headers.received-spf")
            .label("Received-SPF")
            .help("Whether to add a Received-SPF header to the message")
            .default(Expression::new([("local_port == 25", "true")], "false"))
            .new_field("session.data.add-headers.auth-results")
            .label("认证结果")
            .help("Whether to add an Authentication-Results header to the message")
            .default(Expression::new([("local_port == 25", "true")], "false"))
            .new_field("session.data.add-headers.message-id")
            .label("消息 ID")
            .help("Whether to add a Message-Id header to the message")
            .default(Expression::new([("local_port == 25", "true")], "false"))
            .new_field("session.data.add-headers.date")
            .label("日期")
            .help("Whether to add a Date header to the message")
            .default(Expression::new([("local_port == 25", "true")], "false"))
            .new_field("session.data.add-headers.return-path")
            .label("Return-Path")
            .help("Whether to add a Return-Path header to the message")
            .default(Expression::new([("local_port == 25", "true")], "false"))
            .new_field("session.data.add-headers.delivered-to")
            .label("投递至")
            .help("Whether to add a Delivered-To header to the message")
            .default(Expression::new([], "true"))
            .build()
            .new_form_section()
            .title("DATA 阶段")
            .fields(["session.data.spam-filter", "session.data.script"])
            .build()
            .new_form_section()
            .title("限制")
            .fields([
                "session.data.limits.messages",
                "session.data.limits.size",
                "session.data.limits.received-headers",
            ])
            .build()
            .new_form_section()
            .title("添加头部")
            .fields([
                "session.data.add-headers.received",
                "session.data.add-headers.received-spf",
                "session.data.add-headers.auth-results",
                "session.data.add-headers.message-id",
                "session.data.add-headers.date",
                "session.data.add-headers.return-path",
                "session.data.add-headers.delivered-to",
            ])
            .build()
            .build()
            // Inbound rate limiter
            .new_schema("smtp-in-throttle")
            .prefix("queue.limiter.inbound")
            .names("throttle", "throttles")
            .suffix("enable")
            .new_id_field()
            .label("限制器 ID")
            .help("Unique identifier for the throttle")
            .build()
            .new_field("enable")
            .label("已启用")
            .help("Whether to enable this throttle")
            .typ(Type::Boolean)
            .default("true")
            .build()
            .new_field("key")
            .label("密钥")
            .help(concat!(
                "Optional list of context variables that determine ",
                "where this throttle should be applied"
            ))
            .typ(Type::Select {
                typ: SelectType::Many,
                source: Source::Static(&[
                    (V_LISTENER, "Listener"),
                    (V_REMOTE_IP, "Remote IP"),
                    (V_LOCAL_IP, "Local IP"),
                    (V_AUTHENTICATED_AS, "Authenticated As"),
                    (V_HELO_DOMAIN, "EHLO Domain"),
                    (V_SENDER, "Sender"),
                    (V_SENDER_DOMAIN, "Sender Domain"),
                    (V_RECIPIENT, "Recipient"),
                    (V_RECIPIENT_DOMAIN, "Recipient Domain"),
                ]),
            })
            .build()
            .new_field("match")
            .label("匹配条件")
            .help(concat!(
                "Enable the imposition of concurrency and rate limits only ",
                "when a specific condition is met"
            ))
            .typ(Type::Expression)
            .input_check(
                [],
                [
                    Validator::IsValidExpression(has_rcpt_vars),
                    Validator::MaxItems(1),
                ],
            )
            .build()
            .new_field("rate")
            .label("速率限制")
            .help(concat!(
                "Number of incoming requests over a period of time ",
                "that the rate limiter will allow"
            ))
            .typ(Type::Rate)
            .input_check([], [Validator::Required])
            .build()
            .new_form_section()
            .title("入站速率限制")
            .fields(["_id", "key", "rate", "match", "enable"])
            .build()
            .list_title("Inbound Rate Limits")
            .list_subtitle("Manage inbound rate limits")
            .list_fields(["_id", "rate", "enable"])
            .build()
            // Milter
            .new_schema("milter")
            .prefix("session.milter")
            .suffix("hostname")
            .names("milter", "milters")
            .new_id_field()
            .label("Milter ID")
            .help("Unique identifier for this milter")
            .build()
            .new_field("enable")
            .label("启用")
            .help("Expression that determines whether to enable this milter")
            .default("true")
            .typ(Type::Expression)
            .input_check(
                [],
                [
                    Validator::Required,
                    Validator::IsValidExpression(has_rcpt_vars),
                ],
            )
            .build()
            .new_field("hostname")
            .label("主机名")
            .help(concat!(
                "Hostname or IP address of the server where the Milter ",
                "filter is running"
            ))
            .placeholder("127.0.0.1")
            .typ(Type::Input)
            .input_check(
                [Transformer::Trim],
                [Validator::Required, Validator::IsHost],
            )
            .build()
            .new_field("port")
            .label("端口")
            .help("Network port on the Milter filter host server")
            .placeholder("11332")
            .typ(Type::Input)
            .input_check(
                [Transformer::Trim],
                [Validator::Required, Validator::IsPort],
            )
            .build()
            .new_field("tls")
            .label("启用 TLS")
            .help(concat!(
                "Whether to use Transport Layer Security (TLS) for the connection ",
                "between Stalwart SMTP and the Milter filter"
            ))
            .default("false")
            .typ(Type::Boolean)
            .input_check([], [Validator::Required])
            .build()
            .new_field("allow-invalid-certs")
            .label("允许无效证书")
            .help(concat!(
                "Whether Stalwart SMTP should connect to a Milter filter ",
                "server that has an invalid TLS certificate"
            ))
            .default("false")
            .typ(Type::Boolean)
            .input_check([], [Validator::Required])
            .build()
            .new_field("timeout.connect")
            .label("连接")
            .help(concat!(
                "Maximum amount of time that Stalwart SMTP will wait to establish ",
                "a connection with this Milter server"
            ))
            .default("30s")
            .typ(Type::Duration)
            .input_check([], [Validator::Required])
            .build()
            .new_field("timeout.command")
            .label("命令")
            .help(concat!(
                "How long Stalwart SMTP will wait to send a command to the ",
                "Milter server"
            ))
            .default("30s")
            .typ(Type::Duration)
            .input_check([], [Validator::Required])
            .build()
            .new_field("timeout.data")
            .label("数据")
            .help(concat!(
                "Maximum amount of time Stalwart SMTP will wait for a response",
                " from the Milter server"
            ))
            .default("60s")
            .typ(Type::Duration)
            .input_check([], [Validator::Required])
            .build()
            .new_field("options.tempfail-on-error")
            .label("错误时临时失败")
            .help(concat!(
                "Whether to respond with a temporary failure (typically a 4xx ",
                "SMTP status code) when Stalwart encounters an error while ",
                "communicating with this Milter server"
            ))
            .default("true")
            .typ(Type::Boolean)
            .input_check([], [Validator::Required])
            .build()
            .new_field("options.max-response-size")
            .label("最大响应")
            .help(concat!(
                "Maximum size, in bytes, of a response that Stalwart will accept",
                " from this Milter server"
            ))
            .default("52428800")
            .typ(Type::Size)
            .input_check([], [Validator::Required])
            .build()
            .new_field("options.version")
            .label("协议版本")
            .help(concat!(
                "Version of the Milter protocol that Stalwart SMTP should use when",
                " communicating with the Milter server"
            ))
            .default("6")
            .typ(Type::Select {
                typ: SelectType::Single,
                source: Source::Static(&[("2", "Version 2"), ("6", "Version 6")]),
            })
            .input_check([], [Validator::Required])
            .build()
            .new_field("stages")
            .label("运行阶段")
            .help("Which SMTP stages to run the milter on")
            .typ(Type::Select {
                typ: SelectType::Many,
                source: Source::Static(SMTP_STAGES),
            })
            .default("data")
            .build()
            .new_form_section()
            .title("Milter 设置")
            .fields([
                "_id",
                "hostname",
                "port",
                "enable",
                "tls",
                "allow-invalid-certs",
            ])
            .build()
            .new_form_section()
            .title("选项")
            .fields([
                "stages",
                "options.max-response-size",
                "options.version",
                "options.tempfail-on-error",
            ])
            .build()
            .new_form_section()
            .title("超时")
            .fields(["timeout.connect", "timeout.command", "timeout.data"])
            .build()
            .list_title("Milter filters")
            .list_subtitle("Manage Milter filters")
            .list_fields(["_id", "hostname", "port"])
            .build()
            // MTA Hooks
            .new_schema("mta-hooks")
            .prefix("session.hook")
            .suffix("url")
            .names("hook", "hooks")
            .new_id_field()
            .label("Hook ID")
            .help("Unique identifier for this hook")
            .build()
            .new_field("enable")
            .label("启用")
            .help("Expression that determines whether to enable this hook")
            .default("true")
            .typ(Type::Expression)
            .input_check(
                [],
                [
                    Validator::Required,
                    Validator::IsValidExpression(has_rcpt_vars),
                ],
            )
            .build()
            .new_field("url")
            .label("端点 URL")
            .help(concat!("URL of the hook endpoint"))
            .placeholder("https://127.0.0.1/filter")
            .typ(Type::Input)
            .input_check([Transformer::Trim], [Validator::Required, Validator::IsUrl])
            .build()
            .new_field("allow-invalid-certs")
            .label("允许无效证书")
            .help(concat!(
                "Whether Stalwart SMTP should connect to a hook ",
                "server that has an invalid TLS certificate"
            ))
            .default("false")
            .typ(Type::Boolean)
            .input_check([], [Validator::Required])
            .build()
            .new_field("timeout")
            .label("超时")
            .help(concat!(
                "Maximum amount of time that Stalwart SMTP will wait for a response ",
                "from this hook server"
            ))
            .default("30s")
            .typ(Type::Duration)
            .input_check([], [Validator::Required])
            .build()
            .new_field("options.tempfail-on-error")
            .label("错误时临时失败")
            .help(concat!(
                "Whether to respond with a temporary failure (typically a 4xx ",
                "SMTP status code) when Stalwart encounters an error while ",
                "communicating with this MTA Hook server"
            ))
            .default("true")
            .typ(Type::Boolean)
            .input_check([], [Validator::Required])
            .build()
            .new_field("options.max-response-size")
            .label("最大大小")
            .help(concat!(
                "Maximum size, in bytes, of a response that Stalwart will accept",
                " from this MTA Hook server"
            ))
            .default("52428800")
            .typ(Type::Size)
            .input_check([], [Validator::Required])
            .build()
            .new_field("headers")
            .typ(Type::Array(ArrayType::Text))
            .label("HTTP 头部")
            .help("The headers to be sent with hook requests")
            .build()
            .new_field("auth.username")
            .label("用户名")
            .help(concat!(
                "The username to use when authenticating with the hook server"
            ))
            .typ(Type::Input)
            .input_check([Transformer::Trim], [])
            .build()
            .new_field("auth.secret")
            .label("密钥")
            .help(concat!(
                "The secret to use when authenticating with the hook server"
            ))
            .typ(Type::Secret)
            .input_check([Transformer::Trim], [])
            .build()
            .new_field("stages")
            .label("运行阶段")
            .help("Which SMTP stages to run this hook on")
            .typ(Type::Select {
                typ: SelectType::Many,
                source: Source::Static(SMTP_STAGES),
            })
            .default("data")
            .build()
            .new_form_section()
            .title("MTA Hook 设置")
            .fields(["_id", "url", "enable", "allow-invalid-certs"])
            .build()
            .new_form_section()
            .title("认证")
            .fields(["auth.username", "auth.secret"])
            .build()
            .new_form_section()
            .title("选项")
            .fields(["stages", "headers"])
            .build()
            .new_form_section()
            .title("响应")
            .fields([
                "options.max-response-size",
                "timeout",
                "options.tempfail-on-error",
            ])
            .build()
            .list_title("MTA Hooks")
            .list_subtitle("Manage MTA Hooks")
            .list_fields(["_id", "url"])
            .build()
            // MTA-STS
            .new_schema("smtp-in-mta-sts")
            .new_field("session.mta-sts.mode")
            .typ(Type::Select {
                typ: SelectType::Single,
                source: Source::Static(&[
                    ("enforce", "Enforce"),
                    ("testing", "Testing"),
                    ("disable", "Disabled"),
                ]),
            })
            .input_check([], [Validator::Required])
            .label("策略应用")
            .help("Whether to enforce, test, or disable the MTA-STS policy")
            .default("testing")
            .build()
            .new_field("session.mta-sts.max-age")
            .label("最大生命周期")
            .typ(Type::Duration)
            .help("Maximum time to cache the MTA-STS policy")
            .default("7d")
            .input_check([], [Validator::Required])
            .build()
            .new_field("session.mta-sts.mx")
            .label("MX 模式（覆盖）")
            .help(concat!(
                "Override the allowed MX hosts for the MTA-STS policy domain. ",
                "If empty, the MX hosts are determined from the available TLS certificates"
            ))
            .typ(Type::Array(ArrayType::Text))
            .input_check([Transformer::Trim], [])
            .build()
            .new_form_section()
            .title("MTA-STS 策略")
            .fields([
                "session.mta-sts.mode",
                "session.mta-sts.max-age",
                "session.mta-sts.mx",
            ])
            .build()
            .build()
            // ASN & GeoIP
            .new_schema("smtp-in-asn")
            .new_field("asn.type")
            .typ(Type::Select {
                typ: SelectType::Single,
                source: Source::Static(&[
                    ("resource", "URL Resource"),
                    ("dns", "DNS Lookup"),
                    ("disable", "Disabled"),
                ]),
            })
            .input_check([], [Validator::Required])
            .label("ASN/地理位置来源")
            .help("Whether to obtain ASN and geolocation data from a URL or DNS lookup")
            .default("disable")
            .build()
            .new_field("asn.urls.asn")
            .label("ASN URL")
            .help(concat!(
                "URLs to fetch CSV file containing the IP to ASN mappings.",
            ))
            .typ(Type::Array(ArrayType::Text))
            .input_check([Transformer::Trim], [Validator::Required])
            .display_if_eq("asn.type", ["resource"])
            .build()
            .new_field("asn.urls.geo")
            .label("地理位置 URL")
            .help(concat!(
                "URLs to fetch CSV file containing the IP to country code mappings.",
            ))
            .typ(Type::Array(ArrayType::Text))
            .input_check([Transformer::Trim], [Validator::Required])
            .display_if_eq("asn.type", ["resource"])
            .build()
            .new_field("asn.timeout")
            .label("超时")
            .help(concat!(
                "Time after which the ASN/Geo resource fetch is considered failed.",
            ))
            .default("5m")
            .typ(Type::Duration)
            .input_check([], [Validator::Required])
            .display_if_eq("asn.type", ["resource"])
            .build()
            .new_field("asn.expires")
            .label("过期")
            .help(concat!("How often to refresh the ASN/Geo data.",))
            .default("1d")
            .typ(Type::Duration)
            .input_check([], [Validator::Required])
            .display_if_eq("asn.type", ["resource"])
            .build()
            .new_field("asn.max-size")
            .label("最大大小")
            .help(concat!("Maximum size of the ASN/Geo data file.",))
            .typ(Type::Size)
            .input_check([], [Validator::Required])
            .default("104857600")
            .display_if_eq("asn.type", ["resource"])
            .build()
            .new_field("asn.headers")
            .typ(Type::Array(ArrayType::Text))
            .label("HTTP 头部")
            .help(concat!(
                "Headers to send with the ASN/Geo resource fetch request.",
            ))
            .display_if_eq("asn.type", ["resource"])
            .build()
            .new_field("asn.zone.ipv4")
            .label("IPv4 区域")
            .help(concat!(
                "The DNS zone to query for IPv4 ASN and geolocation data.",
            ))
            .typ(Type::Input)
            .input_check([Transformer::Trim], [Validator::Required])
            .display_if_eq("asn.type", ["dns"])
            .build()
            .new_field("asn.zone.ipv6")
            .label("IPv6 区域")
            .help(concat!(
                "The DNS zone to query for IPv6 ASN and geolocation data.",
            ))
            .typ(Type::Input)
            .input_check([Transformer::Trim], [Validator::Required])
            .display_if_eq("asn.type", ["dns"])
            .build()
            .new_field("asn.separator")
            .label("分隔符")
            .help(concat!(
                "The separator character used in the DNS TXT record.",
            ))
            .typ(Type::Input)
            .input_check([Transformer::Trim], [Validator::Required])
            .display_if_eq("asn.type", ["dns"])
            .default("|")
            .build()
            .new_field("asn.index.asn")
            .label("ASN 索引")
            .help(concat!("The position of the ASN in the DNS TXT record.",))
            .typ(Type::Input)
            .input_check([Transformer::Trim], [Validator::Required])
            .display_if_eq("asn.type", ["dns"])
            .default("0")
            .build()
            .new_field("asn.index.asn-name")
            .label("ASN 名称索引")
            .help(concat!(
                "The position of the ASN Name in the DNS TXT record.",
            ))
            .typ(Type::Input)
            .input_check([Transformer::Trim], [])
            .display_if_eq("asn.type", ["dns"])
            .build()
            .new_field("asn.index.country")
            .label("国家索引")
            .help(concat!(
                "The position of the country code in the DNS TXT record.",
            ))
            .typ(Type::Input)
            .input_check([Transformer::Trim], [])
            .display_if_eq("asn.type", ["dns"])
            .build()
            .new_form_section()
            .title("ASN 和 GeoIP 设置")
            .fields(["asn.type"])
            .build()
            .new_form_section()
            .title("URL 资源")
            .fields(["asn.urls.asn", "asn.urls.geo"])
            .display_if_eq("asn.type", ["resource"])
            .build()
            .new_form_section()
            .title("检索")
            .fields(["asn.expires", "asn.timeout", "asn.max-size"])
            .display_if_eq("asn.type", ["resource"])
            .build()
            .new_form_section()
            .title("认证")
            .fields(["asn.headers"])
            .display_if_eq("asn.type", ["resource"])
            .build()
            .new_form_section()
            .title("DNS 区域")
            .fields(["asn.zone.ipv4", "asn.zone.ipv6"])
            .display_if_eq("asn.type", ["dns"])
            .build()
            .new_form_section()
            .title("TXT 记录格式")
            .fields([
                "asn.separator",
                "asn.index.asn",
                "asn.index.asn-name",
                "asn.index.country",
            ])
            .display_if_eq("asn.type", ["dns"])
            .build()
            .build()
    }
}

pub const VERIFY_CONSTANTS: &[&str] =
    &["relaxed", "strict", "disable", "disabled", "never", "none"];
pub const AUTH_CONSTANTS: &[&str] = &["plain", "login", "xoauth2", "oauthbearer"];
pub const IP_STRATEGY_CONSTANTS: &[&str] =
    &["ipv4_only", "ipv6_only", "ipv6_then_ipv4", "ipv4_then_ipv6"];
pub const REQUIRE_OPTIONAL_CONSTANTS: &[&str] = &[
    "optional", "require", "required", "disable", "disabled", "none", "false",
];
pub const AGGREGATE_FREQ_CONSTANTS: &[&str] = &[
    "daily", "day", "hourly", "hour", "weekly", "week", "never", "disable", "false",
];

pub static SMTP_STAGES: &[(&str, &str)] = &[
    ("connect", "Connect"),
    ("ehlo", "EHLO"),
    ("mail", "MAIL FROM"),
    ("rcpt", "RCPT TO"),
    ("data", "DATA"),
];
