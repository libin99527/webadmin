/*
 * SPDX-FileCopyrightText: 2020 Stalwart Labs LLC <hello@stalw.art>
 *
 * SPDX-License-Identifier: AGPL-3.0-only OR LicenseRef-SEL
 */

use crate::core::schema::*;

impl Builder<Schemas, ()> {
    pub fn build_imap(self) -> Self {
        self.new_schema("imap-settings")
            .new_field("imap.request.max-size")
            .label("请求大小")
            .help("Maximum size of an IMAP request that the server will accept")
            .default("52428800")
            .typ(Type::Size)
            .input_check([], [Validator::Required])
            .build()
            .new_field("imap.timeout.authenticated")
            .label("已认证")
            .help(concat!(
                "Time an authenticated session can remain idle before the server ",
                "terminates it"
            ))
            .default("30m")
            .typ(Type::Duration)
            .input_check([], [Validator::Required])
            .build()
            .new_field("imap.timeout.anonymous")
            .label("匿名")
            .help(concat!(
                "Time an unauthenticated session can stay inactive before being ",
                "ended by the server"
            ))
            .default("1m")
            .typ(Type::Duration)
            .input_check([], [Validator::Required])
            .build()
            .new_field("imap.timeout.idle")
            .label("空闲")
            .help(concat!(
                "Time a connection can stay idle in the IMAP IDLE state before ",
                "the server breaks the connection"
            ))
            .default("30m")
            .typ(Type::Duration)
            .input_check([], [Validator::Required])
            .build()
            // Rate limiting
            .new_field("imap.rate-limit.requests")
            .label("请求数")
            .help("The maximum number of requests per minute")
            .default("2000/1m")
            .typ(Type::Rate)
            .build()
            .new_field("imap.rate-limit.concurrent")
            .label("并发")
            .help("The maximum number of concurrent connections")
            .default("6")
            .typ(Type::Input)
            .input_check([Transformer::Trim], [Validator::MinValue(1.into())])
            .build()
            // Authentication
            .new_field("imap.auth.max-failures")
            .label("最大失败数")
            .help(concat!(
                "Number of authentication attempts a user can make before being ",
                "disconnected by the server"
            ))
            .default("3")
            .typ(Type::Input)
            .input_check(
                [Transformer::Trim],
                [Validator::Required, Validator::MinValue(1.into())],
            )
            .build()
            .new_field("imap.auth.allow-plain-text")
            .label("允许明文认证")
            .help("Whether to allow plain text authentication on unencrypted connections")
            .default("false")
            .typ(Type::Boolean)
            .build()
            .new_form_section()
            .title("认证设置")
            .fields(["imap.auth.max-failures", "imap.auth.allow-plain-text"])
            .build()
            .new_form_section()
            .title("请求限制")
            .fields(["imap.request.max-size"])
            .build()
            .new_form_section()
            .title("超时")
            .fields([
                "imap.timeout.authenticated",
                "imap.timeout.anonymous",
                "imap.timeout.idle",
            ])
            .build()
            .new_form_section()
            .title("速率限制")
            .fields(["imap.rate-limit.requests", "imap.rate-limit.concurrent"])
            .build()
            .build()
            // Folders
            .new_schema("email-folders")
            .new_field("email.folders.inbox.name")
            .label("名称")
            .help("Default name for the inbox folder")
            .default("Inbox")
            .typ(Type::Input)
            .input_check([Transformer::Trim], [])
            .build()
            .new_field("email.folders.inbox.create")
            .label("自动创建")
            .help("Whether to create the inbox folder automatically")
            .default("true")
            .typ(Type::Boolean)
            .build()
            .new_field("email.folders.inbox.subscribe")
            .label("自动订阅")
            .help("Whether to subscribe to the inbox folder automatically")
            .default("true")
            .typ(Type::Boolean)
            .build()
            .new_field("email.folders.trash.name")
            .label("名称")
            .help("Default name for the trash folder")
            .default("Deleted Items")
            .typ(Type::Input)
            .input_check([Transformer::Trim], [])
            .build()
            .new_field("email.folders.trash.create")
            .label("自动创建")
            .help("Whether to create the trash folder automatically")
            .default("true")
            .typ(Type::Boolean)
            .build()
            .new_field("email.folders.trash.subscribe")
            .label("自动订阅")
            .help("Whether to subscribe to the trash folder automatically")
            .default("true")
            .typ(Type::Boolean)
            .build()
            .new_field("email.folders.junk.name")
            .label("名称")
            .help("Default name for the junk folder")
            .default("Junk Mail")
            .typ(Type::Input)
            .input_check([Transformer::Trim], [])
            .build()
            .new_field("email.folders.junk.create")
            .label("自动创建")
            .help("Whether to create the junk folder automatically")
            .default("true")
            .typ(Type::Boolean)
            .build()
            .new_field("email.folders.junk.subscribe")
            .label("自动订阅")
            .help("Whether to subscribe to the junk folder automatically")
            .default("true")
            .typ(Type::Boolean)
            .build()
            .new_field("email.folders.drafts.name")
            .label("名称")
            .help("Default name for the drafts folder")
            .default("Drafts")
            .typ(Type::Input)
            .input_check([Transformer::Trim], [])
            .build()
            .new_field("email.folders.drafts.create")
            .label("自动创建")
            .help("Whether to create the drafts folder automatically")
            .default("true")
            .typ(Type::Boolean)
            .build()
            .new_field("email.folders.drafts.subscribe")
            .label("自动订阅")
            .help("Whether to subscribe to the drafts folder automatically")
            .default("true")
            .typ(Type::Boolean)
            .build()
            .new_field("email.folders.sent.name")
            .label("名称")
            .help("Default name for the sent folder")
            .default("Sent Items")
            .typ(Type::Input)
            .input_check([Transformer::Trim], [])
            .build()
            .new_field("email.folders.sent.create")
            .label("自动创建")
            .help("Whether to create the sent folder automatically")
            .default("true")
            .typ(Type::Boolean)
            .build()
            .new_field("email.folders.sent.subscribe")
            .label("自动订阅")
            .help("Whether to subscribe to the sent folder automatically")
            .default("true")
            .typ(Type::Boolean)
            .build()
            .new_field("email.folders.archive.name")
            .label("名称")
            .help("Default name for the archive folder")
            .default("Archive")
            .typ(Type::Input)
            .input_check([Transformer::Trim], [])
            .build()
            .new_field("email.folders.archive.create")
            .label("自动创建")
            .help("Whether to create the archive folder automatically")
            .default("true")
            .typ(Type::Boolean)
            .build()
            .new_field("email.folders.archive.subscribe")
            .label("自动订阅")
            .help("Whether to subscribe to the archive folder automatically")
            .default("true")
            .typ(Type::Boolean)
            .build()
            .new_field("email.folders.shared.name")
            .label("名称")
            .help("Default name for the shared folder")
            .default("Shared Folders")
            .typ(Type::Input)
            .input_check([Transformer::Trim], [])
            .build()
            .new_form_section()
            .title("收件箱")
            .fields([
                "email.folders.inbox.name",
                "email.folders.inbox.create",
                "email.folders.inbox.subscribe",
            ])
            .build()
            .new_form_section()
            .title("回收站")
            .fields([
                "email.folders.trash.name",
                "email.folders.trash.create",
                "email.folders.trash.subscribe",
            ])
            .build()
            .new_form_section()
            .title("垃圾邮件")
            .fields([
                "email.folders.junk.name",
                "email.folders.junk.create",
                "email.folders.junk.subscribe",
            ])
            .build()
            .new_form_section()
            .title("草稿")
            .fields([
                "email.folders.drafts.name",
                "email.folders.drafts.create",
                "email.folders.drafts.subscribe",
            ])
            .build()
            .new_form_section()
            .title("已发送")
            .fields([
                "email.folders.sent.name",
                "email.folders.sent.create",
                "email.folders.sent.subscribe",
            ])
            .build()
            .new_form_section()
            .title("归档")
            .fields([
                "email.folders.archive.name",
                "email.folders.archive.create",
                "email.folders.archive.subscribe",
            ])
            .build()
            .new_form_section()
            .title("共享文件夹")
            .fields(["email.folders.shared.name"])
            .build()
            .build()
    }
}
