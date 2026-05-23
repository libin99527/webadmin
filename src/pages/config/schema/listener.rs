/*
 * SPDX-FileCopyrightText: 2020 Stalwart Labs LLC <hello@stalw.art>
 *
 * SPDX-License-Identifier: AGPL-3.0-only OR LicenseRef-SEL
 */

use crate::core::schema::*;

impl Builder<Schemas, ()> {
    pub fn build_listener(self) -> Self {
        self.new_schema("listener")
            .names("listener", "listeners")
            .prefix("server.listener")
            .suffix("protocol")
            // Id
            .new_id_field()
            .label("监听器 ID")
            .help("Unique identifier for the listener")
            .build()
            // Type
            .new_field("protocol")
            .typ(Type::Select {
                typ: SelectType::Single,
                source: Source::Static(&[
                    ("smtp", "SMTP"),
                    ("lmtp", "LMTP"),
                    ("http", "HTTP"),
                    ("imap", "IMAP4"),
                    ("pop3", "POP3"),
                    ("managesieve", "ManageSieve"),
                ]),
            })
            .label("协议")
            .help("The protocol used by the listener")
            .input_check([], [Validator::Required])
            .default("smtp")
            .build()
            // Bind addresses
            .new_field("bind")
            .label("绑定地址")
            .help("The addresses the listener will bind to")
            .typ(Type::Array(ArrayType::Text))
            .input_check(
                [Transformer::Trim],
                [Validator::Required, Validator::IsSocketAddr],
            )
            .build()
            // Override proxy protocol
            .new_field("proxy.override")
            .label("覆盖代理网络")
            .help("Override the default proxy protocol networks")
            .typ(Type::Boolean)
            .default("false")
            .build()
            // Override socket options
            .new_field("socket.override")
            .label("覆盖套接字选项")
            .help("Override the default socket options")
            .typ(Type::Boolean)
            .default("false")
            .build()
            // Override TLS options
            .new_field("tls.override")
            .label("覆盖 TLS 选项")
            .help("Override the default TLS options")
            .typ(Type::Boolean)
            .default("false")
            .build()
            .new_field("tls.implicit")
            .label("隐式 TLS")
            .help("Whether to use implicit TLS")
            .typ(Type::Boolean)
            .default("false")
            .build()
            // Add common fields
            .add_network_fields(true)
            .add_tls_fields(true)
            // Forms
            .new_form_section()
            .title("监听器设置")
            .fields(["_id", "protocol", "bind"])
            .build()
            .new_form_section()
            .title("TLS 选项")
            .fields([
                "tls.implicit",
                "tls.override",
                "tls.disable-protocols",
                "tls.disable-ciphers",
                "tls.timeout",
                "tls.ignore-client-order",
            ])
            .build()
            .new_form_section()
            .title("代理协议")
            .fields(["proxy.override", "proxy.trusted-networks"])
            .build()
            .new_form_section()
            .title("套接字选项")
            .fields([
                "socket.override",
                "socket.backlog",
                "socket.ttl",
                "socket.linger",
                "socket.tos",
                "socket.send-buffer-size",
                "socket.recv-buffer-size",
                "socket.nodelay",
                "socket.reuse-addr",
                "socket.reuse-port",
            ])
            .build()
            .list_title("Listeners")
            .list_subtitle("Manage SMTP, IMAP, HTTP, and other listeners")
            .list_fields(["_id", "protocol", "bind", "tls.implicit"])
            .build()
    }
}
