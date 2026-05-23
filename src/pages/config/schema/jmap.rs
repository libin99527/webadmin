/*
 * SPDX-FileCopyrightText: 2020 Stalwart Labs LLC <hello@stalw.art>
 *
 * SPDX-License-Identifier: AGPL-3.0-only OR LicenseRef-SEL
 */

use crate::core::schema::*;

impl Builder<Schemas, ()> {
    #![allow(clippy::useless_concat)]
    pub fn build_jmap(self) -> Self {
        self.new_schema("jmap-limits")
            .new_field("jmap.protocol.get.max-objects")
            .label("获取")
            .help(concat!(
                "Determines the maximum number of objects that can be fetched in a ",
                "single method call"
            ))
            .default("500")
            .typ(Type::Input)
            .input_check(
                [Transformer::Trim],
                [Validator::Required, Validator::MinValue(1.into())],
            )
            .new_field("jmap.protocol.set.max-objects")
            .label("设置")
            .help(concat!(
                "Establishes the maximum number of objects that can be modified in ",
                "a single method call"
            ))
            .default("500")
            .new_field("jmap.protocol.request.max-concurrent")
            .label("并发")
            .help(concat!(
                "Restricts the number of concurrent requests a user can make to the ",
                "JMAP server"
            ))
            .default("4")
            .new_field("jmap.protocol.request.max-size")
            .label("大小")
            .help(concat!(
                "Defines the maximum size of a single request, in bytes, that the ",
                "server will accept"
            ))
            .default("10000000")
            .new_field("jmap.protocol.request.max-calls")
            .label("方法调用")
            .help(concat!(
                "Limits the maximum number of method calls that can be included in",
                " a single request"
            ))
            .default("16")
            .new_field("jmap.protocol.query.max-results")
            .label("查询")
            .help(concat!(
                "Sets the maximum number of results that a Query method can return"
            ))
            .default("5000")
            .new_field("jmap.protocol.upload.max-size")
            .label("最大大小")
            .help(concat!(
                "Defines the maximum file size for file uploads to the server"
            ))
            .default("50000000")
            .new_field("jmap.protocol.upload.max-concurrent")
            .label("最大并发")
            .help(concat!(
                "Restricts the number of concurrent file uploads a user can perform"
            ))
            .default("4")
            .new_field("jmap.protocol.upload.quota.files")
            .label("文件总数")
            .help(concat!(
                "Specifies the maximum number of files that a user can upload within ",
                "a certain period"
            ))
            .default("1000")
            .new_field("jmap.protocol.upload.quota.size")
            .label("总大小")
            .default("50000000")
            .help(concat!(
                "Defines the total size of files that a user can upload within a ",
                "certain period"
            ))
            .new_field("jmap.protocol.changes.max-results")
            .label("变更")
            .help(concat!(
                "Determines the maximum number of change objects that a Changes",
                " method can return"
            ))
            .default("5000")
            .new_field("jmap.mailbox.max-depth")
            .label("最大深度")
            .help(concat!(
                "Restricts the maximum depth of nested mailboxes a user can ",
                "create"
            ))
            .default("10")
            .new_field("jmap.mailbox.max-name-length")
            .label("名称长度")
            .help(concat!("Establishes the maximum length of a mailbox name"))
            .default("255")
            .new_field("jmap.email.max-attachment-size")
            .label("附件大小")
            .help(concat!(
                "Specifies the maximum size for an email attachment"
            ))
            .default("50000000")
            .new_field("jmap.email.max-size")
            .label("邮件大小")
            .help(concat!("Determines the maximum size for an email message"))
            .default("75000000")
            .new_field("jmap.email.parse.max-items")
            .label("邮箱")
            .help(concat!(
                "Limits the maximum number of e-mail message that can be parsed in ",
                "a single request"
            ))
            .default("10")
            .new_field("jmap.calendar.parse.max-items")
            .label("日历")
            .help(concat!(
                "Limits the maximum number of iCalendar items that can be parsed in ",
                "a single request"
            ))
            .default("10")
            .new_field("jmap.contact.parse.max-items")
            .label("联系人")
            .help(concat!(
                "Limits the maximum number of vCard items that can be parsed in ",
                "a single request"
            ))
            .default("10")
            .build()
            .new_field("jmap.protocol.upload.ttl")
            .label("过期时间")
            .help(concat!(
                "Specifies the Time-To-Live (TTL) for each uploaded file, after ",
                "which the file is deleted from temporary storage"
            ))
            .default("1h")
            .typ(Type::Duration)
            .input_check([], [Validator::Required])
            .build()
            .new_form_section()
            .title("请求限制")
            .fields([
                "jmap.protocol.request.max-concurrent",
                "jmap.protocol.request.max-size",
                "jmap.protocol.request.max-calls",
            ])
            .build()
            .new_form_section()
            .title("最大对象数")
            .fields([
                "jmap.protocol.get.max-objects",
                "jmap.protocol.set.max-objects",
            ])
            .build()
            .new_form_section()
            .title("最大结果数")
            .fields([
                "jmap.protocol.query.max-results",
                "jmap.protocol.changes.max-results",
            ])
            .build()
            .new_form_section()
            .title("上传限制")
            .fields([
                "jmap.protocol.upload.max-size",
                "jmap.protocol.upload.max-concurrent",
                "jmap.protocol.upload.quota.files",
                "jmap.protocol.upload.quota.size",
                "jmap.protocol.upload.ttl",
            ])
            .build()
            .new_form_section()
            .title("邮箱限制")
            .fields(["jmap.mailbox.max-depth", "jmap.mailbox.max-name-length"])
            .build()
            .new_form_section()
            .title("邮件限制")
            .fields(["jmap.email.max-attachment-size", "jmap.email.max-size"])
            .build()
            .new_form_section()
            .title("解析限制")
            .fields([
                "jmap.email.parse.max-items",
                "jmap.calendar.parse.max-items",
                "jmap.contact.parse.max-items",
            ])
            .build()
            .build()
            // Push & EventSource
            .new_schema("jmap-push")
            .new_field("jmap.push.throttle")
            .label("限流")
            .help("Time to wait before sending a new request to the push service")
            .default("1ms")
            .typ(Type::Duration)
            .input_check([], [Validator::Required])
            .build()
            .new_field("jmap.push.attempts.interval")
            .label("尝试间隔")
            .help("Time to wait between push attempts")
            .default("1m")
            .typ(Type::Duration)
            .input_check([], [Validator::Required])
            .build()
            .new_field("jmap.push.attempts.max")
            .label("最大尝试次数")
            .help("Maximum number of push attempts before a notification is discarded")
            .default("3")
            .typ(Type::Input)
            .input_check(
                [Transformer::Trim],
                [Validator::Required, Validator::MinValue(1.into())],
            )
            .build()
            .new_field("jmap.push.retry.interval")
            .label("重试间隔")
            .help("Time to wait between retry attempts")
            .default("1s")
            .typ(Type::Duration)
            .input_check([], [Validator::Required])
            .build()
            .new_field("jmap.push.timeout.request")
            .label("请求")
            .help("Time before a connection with a push service URL times out")
            .default("10s")
            .typ(Type::Duration)
            .input_check([], [Validator::Required])
            .build()
            .new_field("jmap.push.timeout.verify")
            .label("验证")
            .help("Time to wait for the push service to verify a subscription")
            .default("1s")
            .typ(Type::Duration)
            .input_check([], [Validator::Required])
            .build()
            .new_field("jmap.event-source.throttle")
            .label("限流")
            .help("Specifies the minimum time between two event source notifications")
            .default("1s")
            .typ(Type::Duration)
            .input_check([], [Validator::Required])
            .build()
            .new_form_section()
            .title("推送订阅")
            .fields([
                "jmap.push.throttle",
                "jmap.push.attempts.interval",
                "jmap.push.attempts.max",
                "jmap.push.retry.interval",
            ])
            .build()
            .new_form_section()
            .title("推送超时")
            .fields(["jmap.push.timeout.request", "jmap.push.timeout.verify"])
            .build()
            .new_form_section()
            .title("事件源")
            .fields(["jmap.event-source.throttle"])
            .build()
            .build()
            // Web Sockets
            .new_schema("jmap-web-sockets")
            .new_field("jmap.web-sockets.throttle")
            .label("限流")
            .help("Amount of time to wait before sending a batch of notifications to a WS client")
            .default("1s")
            .typ(Type::Duration)
            .input_check([], [Validator::Required])
            .build()
            .new_field("jmap.web-sockets.timeout")
            .label("超时")
            .help("Time before an inactive WebSocket connection times out")
            .default("10m")
            .typ(Type::Duration)
            .input_check([], [Validator::Required])
            .build()
            .new_field("jmap.web-sockets.heartbeat")
            .label("心跳")
            .help("Time to wait before sending a new heartbeat to the WebSocket client")
            .default("1m")
            .typ(Type::Duration)
            .input_check([], [Validator::Required])
            .build()
            .new_form_section()
            .title("JMAP WebSocket")
            .fields([
                "jmap.web-sockets.throttle",
                "jmap.web-sockets.timeout",
                "jmap.web-sockets.heartbeat",
            ])
            .build()
            .build()
    }
}
