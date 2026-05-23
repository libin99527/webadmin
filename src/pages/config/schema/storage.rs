/*
 * SPDX-FileCopyrightText: 2020 Stalwart Labs LLC <hello@stalw.art>
 *
 * SPDX-License-Identifier: AGPL-3.0-only OR LicenseRef-SEL
 */

use super::*;

impl Builder<Schemas, ()> {
    #![allow(clippy::useless_concat)]
    pub fn build_storage(self) -> Self {
        self.new_schema("storage")
            .new_field("storage.data")
            .label("存储")
            .help(concat!(
                "Core storage unit where email metadata, folders, and various settings ",
                "are stored. Essentially, it contains all the data except for ",
                "large binary objects (blobs)"
            ))
            .typ(Type::Select {
                source: Source::Dynamic {
                    schema: "store",
                    field: "type",
                    filter: Default::default(),
                },
                typ: SelectType::Single,
            })
            .source_filter(&[
                "foundationdb",
                "mysql",
                "postgresql",
                "sqlite",
                "rocksdb",
                "sql-read-replica",
            ])
            .input_check([], [Validator::Required])
            .build()
            .new_field("storage.blob")
            .label("存储")
            .help(concat!(
                "Used for storing large binary objects such as emails, sieve scripts, ",
                "and other files"
            ))
            .typ(Type::Select {
                source: Source::Dynamic {
                    schema: "store",
                    field: "type",
                    filter: Default::default(),
                },
                typ: SelectType::Single,
            })
            .source_filter(&[
                "foundationdb",
                "mysql",
                "postgresql",
                "sqlite",
                "rocksdb",
                "s3",
                "azure",
                "fs",
                "sql-read-replica",
                "sharded-blob",
            ])
            .input_check([], [Validator::Required])
            .build()
            .new_field("storage.fts")
            .label("存储")
            .help(concat!(
                "Dedicated to indexing for full-text search, enhancing the speed and ",
                "efficiency of text-based queries"
            ))
            .typ(Type::Select {
                source: Source::Dynamic {
                    schema: "store",
                    field: "type",
                    filter: Default::default(),
                },
                typ: SelectType::Single,
            })
            .source_filter(&[
                "foundationdb",
                "mysql",
                "postgresql",
                "sqlite",
                "rocksdb",
                "elasticsearch",
                "meilisearch",
                "sql-read-replica",
            ])
            .input_check([], [Validator::Required])
            .build()
            .new_field("storage.lookup")
            .label("存储")
            .help(concat!(
                "Key-value storage used primarily by the SMTP server and anti-spam ",
                "components"
            ))
            .typ(Type::Select {
                source: Source::Dynamic {
                    schema: "store",
                    field: "type",
                    filter: Default::default(),
                },
                typ: SelectType::Single,
            })
            .source_filter(&[
                "foundationdb",
                "mysql",
                "postgresql",
                "sqlite",
                "rocksdb",
                "redis",
                "sql-read-replica",
                "sharded-in-memory",
            ])
            .input_check([], [Validator::Required])
            .build()
            .new_field("email.encryption.enable")
            .label("启用静态加密")
            .help(concat!(
                "Allow users to configure encryption at rest for their data"
            ))
            .default("true")
            .typ(Type::Boolean)
            .build()
            .new_field("email.encryption.append")
            .label("追加时加密")
            .help(concat!(
                "Encrypt messages that are manually appended by the user using ",
                "JMAP or IMAP"
            ))
            .default("false")
            .typ(Type::Boolean)
            .build()
            .new_field("storage.search-index.default-language")
            .label("默认语言")
            .help(concat!(
                "Default language to use when language detection is not possible"
            ))
            .typ(Type::Input)
            .default("en")
            .input_check([Transformer::Trim], [Validator::Required])
            .build()
            .new_field("storage.search-index.batch-size")
            .label("索引批量大小")
            .help(concat!(
                "Number of items to process in each batch during indexing operations"
            ))
            .typ(Type::Input)
            .default("100")
            .input_check(
                [Transformer::Trim],
                [Validator::Required, Validator::MinValue(1.into())],
            )
            .build()
            .new_field("storage.search-index.email.enable")
            .label("启用邮件搜索")
            .help(concat!(
                "Enable full-text search indexing for email content and metadata"
            ))
            .typ(Type::Input)
            .typ(Type::Boolean)
            .default("true")
            .new_field("storage.search-index.calendar.enable")
            .label("启用日历搜索")
            .help(concat!(
                "Enable full-text search indexing for calendar data"
            ))
            .default("true")
            .new_field("storage.search-index.contacts.enable")
            .label("启用联系人搜索")
            .help(concat!(
                "Enable full-text search indexing for contacts data"
            ))
            .default("true")
            .new_field("storage.search-index.tracing.enable")
            .label("启用追踪搜索")
            .help(concat!("Enable full-text search indexing for tracing data"))
            .default("true")
            .enterprise_feature()
            .build()
            .new_field("account.purge.frequency")
            .label("频率")
            .help(concat!(
                "Specifies how often tombstoned messages are deleted ",
                "from the database"
            ))
            .default("0 0 *")
            .typ(Type::Cron)
            .input_check([], [Validator::Required])
            .build()
            .new_field("changes.max-history")
            .label("变更历史")
            .help(concat!(
                "How many changes to keep in the history for each account. ",
                "This is used to determine the changes that have occurred ",
                "since the last time the client requested changes."
            ))
            .default("10000")
            .typ(Type::Input)
            .build()
            .new_field("email.auto-expunge")
            .label("回收站自动清除")
            .help(concat!(
                "How long to keep messages in the Trash and Junk Mail folders ",
                "before auto-expunging"
            ))
            .default("30d")
            .typ(Type::Duration)
            .build()
            .new_field("storage.undelete.retention")
            .label("恢复期限")
            .help(concat!(
                "How long to keep deleted emails before they are permanently ",
                "removed from the system. (Enterprise feature)"
            ))
            .default("false")
            .typ(Type::Duration)
            .enterprise_feature()
            .build()
            .new_form_section()
            .title("数据存储")
            .fields([
                "storage.data",
                "email.encryption.enable",
                "email.encryption.append",
            ])
            .build()
            .new_form_section()
            .title("Blob 存储")
            .fields(["storage.blob", "storage.undelete.retention"])
            .build()
            .new_form_section()
            .title("搜索存储")
            .fields([
                "storage.fts",
                "storage.search-index.default-language",
                "storage.search-index.batch-size",
                "storage.search-index.email.enable",
                "storage.search-index.calendar.enable",
                "storage.search-index.contacts.enable",
                "storage.search-index.tracing.enable",
            ])
            .build()
            .new_form_section()
            .title("内存存储")
            .fields(["storage.lookup"])
            .build()
            .new_form_section()
            .title("清理")
            .fields([
                "account.purge.frequency",
                "changes.max-history",
                "email.auto-expunge",
            ])
            .build()
            .build()
            // E-mail Storage Quotas
            .new_schema("email-storage-quota")
            .new_field("object-quota.push-subscription")
            .label("推送订阅")
            .help("The default maximum number of push subscriptions a user can create")
            .default("15")
            .typ(Type::Input)
            .input_check([Transformer::Trim], [Validator::MinValue(1.into())])
            .build()
            .new_field("object-quota.email")
            .label("邮箱")
            .help("The default maximum number of emails a user can create")
            .typ(Type::Input)
            .input_check([Transformer::Trim], [Validator::MinValue(1.into())])
            .build()
            .new_field("object-quota.mailbox")
            .label("邮箱")
            .help("The default maximum number of mailboxes a user can create")
            .default("250")
            .typ(Type::Input)
            .input_check([Transformer::Trim], [Validator::MinValue(1.into())])
            .build()
            .new_field("object-quota.identity")
            .label("邮件身份")
            .help("The default maximum number of identities a user can create")
            .default("20")
            .typ(Type::Input)
            .input_check([Transformer::Trim], [Validator::MinValue(1.into())])
            .build()
            .new_field("object-quota.email-submission")
            .label("邮件提交")
            .help("The default maximum number of email submissions a user can create")
            .default("500")
            .typ(Type::Input)
            .input_check([Transformer::Trim], [Validator::MinValue(1.into())])
            .build()
            .new_field("object-quota.sieve-script")
            .label("Sieve 脚本")
            .help("The default maximum number of sieve scripts a user can create")
            .default("100")
            .typ(Type::Input)
            .input_check([Transformer::Trim], [Validator::MinValue(1.into())])
            .build()
            .new_form_section()
            .title("默认对象配额")
            .fields([
                "object-quota.mailbox",
                "object-quota.email",
                "object-quota.sieve-script",
                "object-quota.push-subscription",
                "object-quota.identity",
                "object-quota.email-submission",
            ])
            .build()
            .build()
            // E-mail Storage Quotas
            .new_schema("groupware-storage-quota")
            .new_field("object-quota.calendar")
            .label("日历")
            .help("The default maximum number of calendars a user can create")
            .default("250")
            .typ(Type::Input)
            .input_check([Transformer::Trim], [Validator::MinValue(1.into())])
            .build()
            .new_field("object-quota.calendar-event")
            .label("日历事件")
            .help("The default maximum number of calendar events a user can create")
            .typ(Type::Input)
            .input_check([Transformer::Trim], [Validator::MinValue(1.into())])
            .build()
            .new_field("object-quota.address-book")
            .label("通讯录")
            .help("The default maximum number of address books a user can create")
            .default("250")
            .typ(Type::Input)
            .input_check([Transformer::Trim], [Validator::MinValue(1.into())])
            .build()
            .new_field("object-quota.contact-card")
            .label("联系人卡片")
            .help("The default maximum number of contact cards a user can create")
            .typ(Type::Input)
            .input_check([Transformer::Trim], [Validator::MinValue(1.into())])
            .build()
            .new_field("object-quota.file-node")
            .label("文件节点")
            .help("The default maximum number of file nodes a user can create")
            .typ(Type::Input)
            .input_check([Transformer::Trim], [Validator::MinValue(1.into())])
            .build()
            .new_form_section()
            .title("默认对象配额")
            .fields([
                "object-quota.calendar",
                "object-quota.calendar-event",
                "object-quota.address-book",
                "object-quota.contact-card",
                "object-quota.file-node",
            ])
            .build()
            .build()
    }
}
