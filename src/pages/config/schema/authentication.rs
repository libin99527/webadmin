/*
 * SPDX-FileCopyrightText: 2020 Stalwart Labs LLC <hello@stalw.art>
 *
 * SPDX-License-Identifier: AGPL-3.0-only OR LicenseRef-SEL
 */

use super::*;

impl Builder<Schemas, ()> {
    pub fn build_authentication(self) -> Self {
        // Authentication
        self.new_schema("authentication")
            .new_field("storage.directory")
            .label("目录")
            .help("The directory to use for authentication and authorization")
            .typ(Type::Select {
                source: Source::Dynamic {
                    schema: "directory",
                    field: "type",
                    filter: Default::default(),
                },
                typ: SelectType::Single,
            })
            .input_check([], [Validator::Required])
            .build()
            // Fallback admin
            .new_field("authentication.fallback-admin.user")
            .label("用户名")
            .help(concat!(
                "A rescue admin user can access the server in case the ",
                "directory becomes unavailable"
            ))
            .typ(Type::Input)
            .input_check([Transformer::Trim], [])
            .build()
            .new_field("authentication.fallback-admin.secret")
            .label("密码")
            .help(concat!(
                "A rescue admin secret that can access the server ",
                "in case the directory becomes unavailable"
            ))
            .typ(Type::Secret)
            .input_check([Transformer::Trim, Transformer::HashSecret], [])
            .build()
            // Master user
            .new_field("authentication.master.user")
            .label("用户名")
            .help(concat!(
                "The master user can access any user account ",
                "using 'user-login%master-user' as the login name. ",
                "Leave blank to disable"
            ))
            .typ(Type::Input)
            .input_check([Transformer::Trim], [])
            .build()
            .new_field("authentication.master.secret")
            .label("密码")
            .help("The master user secret to access any user account ")
            .typ(Type::Secret)
            .input_check([Transformer::Trim, Transformer::HashSecret], [])
            .build()
            .new_form_section()
            .title("认证")
            .fields(["storage.directory"])
            .build()
            .new_form_section()
            .title("备用管理员")
            .fields([
                "authentication.fallback-admin.user",
                "authentication.fallback-admin.secret",
            ])
            .build()
            .new_form_section()
            .title("主用户")
            .fields(["authentication.master.user", "authentication.master.secret"])
            .build()
            .build()
            // OAuth
            .new_schema("oauth")
            .new_field("oauth.key")
            .label("密钥")
            .help("Encryption key to use for OAuth")
            .typ(Type::Secret)
            .input_check([], [Validator::Required])
            .build()
            .new_field("oauth.auth.max-attempts")
            .label("最大尝试次数")
            .help("Number of failed login attempts before an authorization code is invalidated")
            .typ(Type::Input)
            .default("3")
            .input_check([], [Validator::Required, Validator::MinValue(1.into())])
            .build()
            .new_field("oauth.expiry.user-code")
            .label("用户代码")
            .help("Expiration time of a user code issued by the device authentication flow")
            .typ(Type::Duration)
            .input_check([], [Validator::Required])
            .default("30m")
            .build()
            .new_field("oauth.expiry.auth-code")
            .label("授权码")
            .help("Expiration time of an authorization code issued by the authorization code flow")
            .typ(Type::Duration)
            .input_check([], [Validator::Required])
            .default("10m")
            .build()
            .new_field("oauth.expiry.token")
            .label("令牌")
            .help("Expiration time of an OAuth access token")
            .typ(Type::Duration)
            .input_check([], [Validator::Required])
            .default("1h")
            .build()
            .new_field("oauth.expiry.refresh-token")
            .label("刷新令牌")
            .help("Expiration time of an OAuth refresh token")
            .typ(Type::Duration)
            .input_check([], [Validator::Required])
            .default("30d")
            .build()
            .new_field("oauth.expiry.refresh-token-renew")
            .label("刷新令牌续期")
            .help("Remaining time in a refresh token before a new one is issued to the client")
            .typ(Type::Duration)
            .input_check([], [Validator::Required])
            .default("4d")
            .build()
            .new_field("oauth.client-registration.require")
            .label("要求客户端注册")
            .help("Whether to require OAuth client_ids to be registered before they can be used")
            .typ(Type::Boolean)
            .default("false")
            .build()
            .new_field("oauth.client-registration.anonymous")
            .label("允许匿名注册")
            .help("Whether to allow OAuth clients to register without authentication")
            .typ(Type::Boolean)
            .default("false")
            .build()
            .new_form_section()
            .title("OAuth 设置")
            .fields(["oauth.key"])
            .fields(["oauth.auth.max-attempts"])
            .build()
            .new_form_section()
            .title("令牌过期")
            .fields([
                "oauth.expiry.user-code",
                "oauth.expiry.auth-code",
                "oauth.expiry.token",
                "oauth.expiry.refresh-token",
                "oauth.expiry.refresh-token-renew",
            ])
            .build()
            .new_form_section()
            .title("动态客户端注册")
            .fields([
                "oauth.client-registration.require",
                "oauth.client-registration.anonymous",
            ])
            .build()
            .build()
            // OpenID
            .new_schema("openid")
            .new_field("oauth.oidc.signature-algorithm")
            .label("签名算法")
            .help("JWT signature algorithm to use for OpenID Connect.")
            .default("relaxed/relaxed")
            .typ(Type::Select {
                typ: SelectType::Single,
                source: Source::StaticId(&[
                    "ES256", "ES384", "PS256", "PS384", "PS512", "RS256", "RS384", "RS512",
                    "HS256", "HS384", "HS512",
                ]),
            })
            .default("HS256")
            .input_check([], [Validator::Required])
            .build()
            .new_field("oauth.oidc.signature-key")
            .label("签名密钥")
            .help("Contents of the private key PEM used to sign JWTs for OpenID Connect.")
            .typ(Type::Text)
            .input_check([], [Validator::Required])
            .build()
            .new_form_section()
            .title("OpenID Connect")
            .fields(["oauth.oidc.signature-algorithm", "oauth.oidc.signature-key"])
            .build()
            .build()
    }
}
