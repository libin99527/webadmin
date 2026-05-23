/*
 * SPDX-FileCopyrightText: 2020 Stalwart Labs LLC <hello@stalw.art>
 *
 * SPDX-License-Identifier: AGPL-3.0-only OR LicenseRef-SEL
 */

use crate::core::schema::*;

impl Builder<Schemas, ()> {
    #![allow(clippy::useless_concat)]
    pub fn build_webdav(self) -> Self {
        // WebDAV
        self.new_schema("webdav")
            .new_field("dav.request.max-size")
            .label("最大请求大小")
            .help(concat!(
                "Determines the maximum XML size of a WebDAV ",
                "request that the server will accept"
            ))
            .default("26214400")
            .typ(Type::Size)
            .input_check([], [Validator::Required])
            .new_field("file-storage.max-size")
            .label("最大文件大小")
            .help(concat!(
                "Specifies the maximum size of a file that ",
                "can be uploaded to the server"
            ))
            .default("26214400")
            .new_field("dav.property.max-size.live")
            .label("活动属性")
            .help(concat!(
                "Specifies the maximum size of a WebDAV live ",
                "property value that the server will accept"
            ))
            .default("250")
            .new_field("dav.property.max-size.dead")
            .label("死属性")
            .default("1024")
            .help(concat!(
                "Specifies the maximum size of a WebDAV dead ",
                "property value that the server will accept"
            ))
            .input_check([], [])
            .build()
            .new_field("dav.lock.max-timeout")
            .label("最大锁超时")
            .default("1h")
            .help(concat!(
                "Specifies the maximum duration for which a ",
                "lock can be held on a resource"
            ))
            .typ(Type::Duration)
            .input_check([], [Validator::Required])
            .build()
            .new_field("dav.locks.max-per-user")
            .label("每用户最大锁数")
            .help(concat!(
                "Specifies the maximum number of locks that ",
                "a user can create on a resource"
            ))
            .default("10")
            .typ(Type::Input)
            .input_check([Transformer::Trim], [Validator::Required])
            .new_field("dav.response.max-results")
            .label("最大结果数")
            .help(concat!(
                "Specifies the maximum number of results ",
                "that a WebDAV query can return"
            ))
            .default("2000")
            .build()
            .new_field("dav.collection.assisted-discovery")
            .label("辅助发现")
            .help(concat!(
                "Enables assisted discovery of WebDAV shared collections by ",
                "modifying PROPFIND requests to the root collection. Requests ",
                "with depth 1 are automatically changed to depth 2, which may ",
                "cause compatibility issues with some clients that expect the ",
                "original behavior."
            ))
            .default("false")
            .typ(Type::Boolean)
            .input_check([], [Validator::Required])
            .build()
            .new_form_section()
            .title("WebDAV 设置")
            .fields([
                "dav.request.max-size",
                "dav.response.max-results",
                "dav.collection.assisted-discovery",
            ])
            .build()
            .new_form_section()
            .title("属性限制")
            .fields(["dav.property.max-size.live", "dav.property.max-size.dead"])
            .build()
            .new_form_section()
            .title("锁定")
            .fields(["dav.lock.max-timeout", "dav.locks.max-per-user"])
            .build()
            .new_form_section()
            .title("文件存储")
            .fields(["file-storage.max-size"])
            .build()
            .build()
            // Calendar
            .new_schema("calendar")
            .new_field("calendar.max-size")
            .label("最大 iCal 大小")
            .help(concat!(
                "Specifies the maximum size of an iCalendar ",
                "file that can be uploaded to the server"
            ))
            .default("524288")
            .typ(Type::Size)
            .input_check([], [Validator::Required])
            .new_field("calendar.max-recurrence-expansions")
            .label("最大 iCal 实例数")
            .help(concat!(
                "Specifies the maximum number of instances that ",
                "can be generated from a recurring iCalendar event"
            ))
            .default("3000")
            .typ(Type::Input)
            .input_check([Transformer::Trim], [Validator::Required])
            .new_field("calendar.max-attendees-per-instance")
            .label("最大 iCal 参与者数")
            .help(concat!(
                "Specifies the maximum number of attendees that ",
                "can be included in a single iCalendar instance"
            ))
            .default("20")
            .typ(Type::Input)
            .new_field("calendar.default.href-name")
            .label("默认链接名称")
            .help(concat!(
                "Specifies the default href name for a calendar when it is created"
            ))
            .default("default")
            .typ(Type::Input)
            .input_check([Transformer::Trim], [])
            .new_field("calendar.default.display-name")
            .label("默认显示名称")
            .help(concat!(
                "Specifies the default display name for a calendar when it is created"
            ))
            .default("Stalwart Calendar")
            .typ(Type::Input)
            .input_check([Transformer::Trim], [Validator::Required])
            .build()
            .new_form_section()
            .title("日历设置")
            .fields([
                "calendar.max-size",
                "calendar.max-recurrence-expansions",
                "calendar.max-attendees-per-instance",
            ])
            .build()
            .new_form_section()
            .title("默认名称")
            .fields([
                "calendar.default.href-name",
                "calendar.default.display-name",
            ])
            .build()
            .build()
            // Contacts
            .new_schema("contacts")
            .new_field("contacts.max-size")
            .label("最大 vCard 大小")
            .help(concat!(
                "Specifies the maximum size of a vCard file ",
                "that can be uploaded to the server"
            ))
            .default("524288")
            .typ(Type::Size)
            .input_check([], [Validator::Required])
            .new_field("contacts.default.href-name")
            .label("默认链接名称")
            .help(concat!(
                "Specifies the default href name for a contact when it is created"
            ))
            .default("default")
            .typ(Type::Input)
            .input_check([Transformer::Trim], [])
            .new_field("contacts.default.display-name")
            .label("默认显示名称")
            .help(concat!(
                "Specifies the default display name for a contact when it is created"
            ))
            .default("Stalwart Address Book")
            .typ(Type::Input)
            .input_check([Transformer::Trim], [Validator::Required])
            .build()
            .new_form_section()
            .title("联系人设置")
            .fields([
                "contacts.max-size",
                "contacts.default.href-name",
                "contacts.default.display-name",
            ])
            .build()
            .build()
            // Scheduling
            .new_schema("scheduling")
            .new_field("calendar.scheduling.enable")
            .label("启用调度")
            .help(concat!(
                "Enables the scheduling features for calendar events, ",
                "allowing users to send and receive invitations"
            ))
            .default("true")
            .typ(Type::Boolean)
            .input_check([], [Validator::Required])
            .build()
            .new_field("calendar.scheduling.inbound.auto-add")
            .label("自动添加邀请")
            .help(concat!(
                "Automatically adds incoming ",
                "invitations to the user's calendar."
            ))
            .default("false")
            .typ(Type::Boolean)
            .input_check([], [Validator::Required])
            .build()
            .new_field("calendar.scheduling.inbound.max-size")
            .label("最大 iTIP 大小")
            .help(concat!(
                "Sets the maximum iCalendar ",
                "object size for ",
                "incoming iTIP messages."
            ))
            .default("512000")
            .typ(Type::Size)
            .input_check([], [Validator::Required, Validator::MinValue(100.into())])
            .build()
            .new_field("calendar.scheduling.outbound.max-recipients")
            .label("最大收件人数")
            .help(concat!(
                "Sets the maximum number of ",
                "recipients for outbound iTIP messages."
            ))
            .default("100")
            .typ(Type::Input)
            .input_check(
                [Transformer::Trim],
                [Validator::Required, Validator::MinValue(1.into())],
            )
            .build()
            .new_field("calendar.scheduling.inbox.auto-expunge")
            .label("收件箱自动清除")
            .help(concat!(
                "Sets the duration after which the iTIP inbox ",
                "will automatically expunge old messages."
            ))
            .default("30d")
            .typ(Type::Duration)
            .input_check([], [Validator::Required])
            .build()
            .new_field("calendar.scheduling.http-rsvp.enable")
            .label("启用 HTTP RSVP")
            .help(concat!(
                "Enables the HTTP RSVP feature for calendar invitations, ",
                "allowing users to respond via a web interface."
            ))
            .default("true")
            .typ(Type::Boolean)
            .input_check([], [Validator::Required])
            .build()
            .new_field("calendar.scheduling.http-rsvp.url")
            .label("HTTP RSVP URL")
            .help(concat!(
                "Specifies a custom URL for the HTTP RSVP endpoint, ",
                "where users can respond to calendar invitations."
            ))
            .typ(Type::Input)
            .input_check([Transformer::Trim], [Validator::IsUrl])
            .build()
            .new_field("calendar.scheduling.http-rsvp.expiration")
            .label("HTTP RSVP 过期时间")
            .help(concat!(
                "Sets the expiration duration for HTTP RSVP links, ",
                "after which they will no longer be valid."
            ))
            .default("90d")
            .typ(Type::Duration)
            .input_check([], [Validator::Required])
            .build()
            .new_field("calendar.scheduling.template.email")
            .label("iMIP 模板")
            .help(concat!(
                "Specifies the HTML template used ",
                "for rendering iMIP invitations."
            ))
            .enterprise_feature()
            .typ(Type::Text)
            .build()
            .new_field("calendar.scheduling.template.web")
            .label("RSVP 模板")
            .help(concat!(
                "Specifies the HTML template used ",
                "for rendering HTTP RSVP confirmations."
            ))
            .enterprise_feature()
            .typ(Type::Text)
            .build()
            .new_form_section()
            .title("日历调度")
            .fields([
                "calendar.scheduling.inbound.max-size",
                "calendar.scheduling.inbox.auto-expunge",
                "calendar.scheduling.inbound.auto-add",
                "calendar.scheduling.enable",
            ])
            .build()
            .new_form_section()
            .title("出站 iMIP")
            .fields([
                "calendar.scheduling.outbound.max-recipients",
                "calendar.scheduling.template.email",
            ])
            .build()
            .new_form_section()
            .title("HTTP RSVP")
            .fields([
                "calendar.scheduling.http-rsvp.url",
                "calendar.scheduling.http-rsvp.expiration",
                "calendar.scheduling.template.web",
                "calendar.scheduling.http-rsvp.enable",
            ])
            .build()
            .build()
            // Sharing
            .new_schema("sharing")
            .new_field("sharing.max-shares-per-item")
            .label("最大共享数")
            .help(concat!(
                "Specifies the maximum number of sharees that can be added to a single ",
                "shared item (calendar, address book or file)"
            ))
            .default("10")
            .typ(Type::Input)
            .input_check([], [Validator::Required, Validator::MinValue(1.into())])
            .build()
            .new_field("sharing.max-history")
            .label("共享历史")
            .help(concat!(
                "Specifies the duration for which the JMAP share notification history is retained ",
                "before it is automatically purged."
            ))
            .default("30d")
            .typ(Type::Duration)
            .input_check([Transformer::Trim], [])
            .build()
            .new_field("sharing.allow-directory-query")
            .label("允许目录查询")
            .help("Whether authenticated users can query the directory via WebDAV and JMAP")
            .typ(Type::Boolean)
            .default("false")
            .build()
            .new_form_section()
            .title("共享设置")
            .fields([
                "sharing.max-shares-per-item",
                "sharing.max-history",
                "sharing.allow-directory-query",
            ])
            .build()
            .build()
            // Alarms
            .new_schema("alarms")
            .new_field("calendar.alarms.enabled")
            .label("启用邮件提醒")
            .help(concat!(
                "Enables the calendar alarms feature, allowing users to set alarms for events ",
                "and receive notifications via e-mail"
            ))
            .default("true")
            .typ(Type::Boolean)
            .input_check([], [Validator::Required])
            .build()
            .new_field("calendar.alarms.minimum-interval")
            .label("最小提醒间隔")
            .help(concat!(
                "Specifies the minimum interval for calendar alarms, ",
                "ensuring that alarms are not triggered too frequently"
            ))
            .default("1h")
            .typ(Type::Duration)
            .input_check([], [Validator::Required])
            .build()
            .new_field("calendar.alarms.allow-external-recipients")
            .label("允许外部收件人")
            .help(concat!(
                "Allows calendar alarms to be sent to external recipients, ",
                "enabling notifications to users outside the server"
            ))
            .default("false")
            .typ(Type::Boolean)
            .input_check([], [Validator::Required])
            .build()
            .new_field("calendar.alarms.from.name")
            .label("发件人名称")
            .help(concat!(
                "Specifies the name that will appear in the 'From' field of ",
                "calendar alarm e-mails, ",
                "providing a recognizable sender name for users"
            ))
            .default("Stalwart Calendar")
            .typ(Type::Input)
            .input_check([Transformer::Trim], [])
            .build()
            .new_field("calendar.alarms.from.email")
            .label("发件邮箱")
            .help(concat!(
                "Specifies the e-mail address that will appear in the 'From' ",
                "field of calendar alarm e-mails, ",
                "ensuring that users can reply to or contact the sender"
            ))
            .typ(Type::Input)
            .input_check([Transformer::Trim], [Validator::IsEmail])
            .build()
            .new_field("calendar.alarms.template")
            .label("提醒模板")
            .help(concat!(
                "Specifies the HTML template used for rendering calendar alarm e-mails, ",
                "allowing customization of the alarm notification format"
            ))
            .enterprise_feature()
            .typ(Type::Text)
            .input_check([Transformer::Trim], [])
            .build()
            .new_form_section()
            .title("日历提醒")
            .fields([
                "calendar.alarms.minimum-interval",
                "calendar.alarms.allow-external-recipients",
                "calendar.alarms.enabled",
            ])
            .build()
            .new_form_section()
            .title("通知邮件")
            .fields([
                "calendar.alarms.from.name",
                "calendar.alarms.from.email",
                "calendar.alarms.template",
            ])
            .build()
            .build()
    }
}
