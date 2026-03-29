//! Built-in dictionary mapping well-known cookie names to human-readable descriptions.

pub fn lookup(name: &str) -> Option<&'static str> {
    let desc = match name {
        // Google
        "NID" => "Google: unique browser/user ID for ads & preferences",
        "SID" | "HSID" | "SSID" => "Google: account session authentication",
        "APISID" | "SAPISID" => "Google: API session identifier",
        "1P_JAR" => "Google: ad targeting & conversion tracking",
        "CONSENT" => "Google: cookie consent state",
        "OGPC" | "OGP" => "Google: Google+ session preferences",
        "AEC" => "Google: fraud prevention & ad integrity",
        "SOCS" => "Google: cookie consent / NID opt-out flag",
        "__gsas" => "Google: AdSense ad personalisation",
        "DV" => "Google: rate-limiting & regional preferences",
        "SEARCH_SAMESITE" => "Google: SameSite cross-site tracking guard",
        "ANID" => "Google: advertising cookie for non-Google sites",

        // Google Analytics & Ads
        "_ga" => "Google Analytics: unique visitor identifier",
        "_gid" => "Google Analytics: 24h visitor identifier",
        "_gat" => "Google Analytics: request throttle flag",
        "_gcl_au" => "Google Ads: conversion linker click ID",
        "__gads" => "Google Ads: ad-serving & frequency capping",

        // Meta / Facebook
        "_fbp" => "Meta Pixel: browser-level user identifier",
        "_fbc" => "Meta Pixel: click identifier from ad click",
        "fr" => "Facebook: ad delivery & measurement",
        "sb" => "Facebook: browser identification for security",
        "datr" => "Facebook: browser integrity / spam prevention",
        "xs" => "Facebook: session ID tied to account",
        "c_user" => "Facebook: logged-in user ID",

        // Microsoft
        "MUID" => "Microsoft: unique machine/user identifier",
        "ANON" => "Microsoft: anonymous user preferences",
        "NAP" => "Microsoft: authentication profile token",
        "MSFPC" | "MC1" => "Microsoft: first-party analytics identifier",
        "AI_sentBuffer" => "Azure Application Insights: telemetry buffer",

        // Cloudflare
        "__cf_bm" => "Cloudflare: bot-management challenge token",
        "cf_clearance" => "Cloudflare: visitor passed JS/CAPTCHA challenge",
        "__cfruid" => "Cloudflare: rate-limiting session identifier",
        "__cfduid" => "Cloudflare (legacy): client identification",

        // HubSpot
        "__hstc" => "HubSpot: main visitor tracking cookie",
        "hubspotutk" => "HubSpot: visitor identity for form submissions",
        "__hssc" => "HubSpot: session-level page view counter",
        "__hssrc" => "HubSpot: session reset detection flag",

        // LinkedIn
        "bcookie" => "LinkedIn: browser identifier for security",
        "li_sugr" => "LinkedIn: cross-domain visitor identifier",
        "lidc" => "LinkedIn: data-center routing cookie",
        "li_at" => "LinkedIn: member authentication token",
        "UserMatchHistory" => "LinkedIn: ad sync / audience matching",
        "AnalyticsSyncHistory" => "LinkedIn: analytics partner sync",
        "lang" => "LinkedIn: UI language preference",

        // TikTok
        "_ttp" => "TikTok Pixel: visitor attribution identifier",
        "ttwid" => "TikTok: unique session/user identifier",
        "tt_csrf_token" => "TikTok: CSRF protection token",
        "msToken" => "TikTok: anti-bot verification token",

        // Pinterest
        "_pinterest_sess" => "Pinterest: session authentication token",
        "_pin_unauth" => "Pinterest: anonymous / first-party identifier",
        "_routing_id" => "Pinterest: server routing / load-balancing",

        // Twitter / X
        "_twitter_sess" => "Twitter: session authentication token",
        "ct0" => "Twitter: CSRF / authenticity token",
        "guest_id" => "Twitter: logged-out visitor identifier",
        "personalization_id" => "Twitter: personalisation / ad tracking",

        // Workday
        "PLAY_SESSION" => "Workday: Play Framework session token",
        "wday_vps_cookie" => "Workday: virtual-proxy server routing",
        "wd-browser-id" => "Workday: browser fingerprint for session",

        // ServiceNow
        "glide_user_route" => "ServiceNow: node affinity / routing",
        "glide_session_store" => "ServiceNow: server-side session ID",

        // SAP
        "sap-usercontext" => "SAP: user locale & client context",
        "MYSAPSSO2" => "SAP: Single Sign-On v2 ticket",

        // Salesforce
        "BrowserId" => "Salesforce: browser-level tracking identifier",
        "CookieConsentPolicy" => "Salesforce: consent categories accepted",
        "sid" => "Salesforce: session authentication ID",
        "oid" => "Salesforce: organization / tenant ID",
        "inst" => "Salesforce: instance routing identifier",

        // Atlassian (Jira, Confluence)
        "cloud.session.token" => "Atlassian Cloud: session authentication",
        "atlassian.xsrf.token" => "Atlassian: CSRF protection token",
        "tenant.session.token" => "Atlassian: multi-tenant session binding",

        // Zendesk
        "_zendesk_cookie" => "Zendesk: general tracking identifier",
        "_zendesk_session" => "Zendesk: support session token",
        "_zendesk_shared_session" => "Zendesk: cross-subdomain session",

        // AWS
        "AWSALB" => "AWS ALB: sticky session for load balancer",
        "AWSALBCORS" => "AWS ALB: CORS-aware sticky session",
        "aws-waf-token" => "AWS WAF: bot-protection verification token",

        // Stripe
        "__stripe_mid" => "Stripe: merchant-level fraud-prevention ID",
        "__stripe_sid" => "Stripe: session-level fraud-prevention ID",

        // Segment
        "ajs_user_id" => "Segment: identified user analytics ID",
        "ajs_anonymous_id" => "Segment: anonymous visitor analytics ID",

        // Other analytics & A/B testing
        "optimizelyEndUserId" => "Optimizely: A/B test user identifier",
        "_dd_s" => "Datadog RUM: browser session tracking",
        "csrftoken" => "CSRF: cross-site request forgery protection token",

        // Server-framework session cookies
        "PHPSESSID" => "PHP: server-side session identifier",
        "JSESSIONID" => "Java Servlet: server-side session identifier",
        "ASP.NET_SessionId" => "ASP.NET: server-side session identifier",
        "connect.sid" => "Node.js Express: session identifier",
        "rack.session" => "Ruby Rack: server-side session identifier",
        "laravel_session" => "Laravel: encrypted session identifier",
        "_csrf" | "XSRF-TOKEN" => "CSRF: cross-site request forgery token",

        _ => return lookup_prefix(name),
    };
    Some(desc)
}

fn lookup_prefix(name: &str) -> Option<&'static str> {
    if name.starts_with("__Secure-") {
        return Some("Google: secure cross-site authentication");
    }
    if name.starts_with("_ga_") {
        return Some("Google Analytics: measurement stream session");
    }
    if name.starts_with("SAP_SESSIONID_") {
        return Some("SAP: application server session ID");
    }
    if name.starts_with("ORA_") {
        return Some("Oracle: application tracking / session");
    }
    if name.starts_with("BIGipServer") {
        return Some("F5 / ServiceNow: server-pool persistence");
    }
    if name.starts_with("_hjSessionUser_") {
        return Some("Hotjar: returning visitor identifier");
    }
    if name.starts_with("_hjSession_") {
        return Some("Hotjar: active session data");
    }
    if name.starts_with("intercom-id-") {
        return Some("Intercom: visitor identity token");
    }
    if name.starts_with("intercom-session-") {
        return Some("Intercom: session state token");
    }
    if name.starts_with("wp-settings-") {
        return Some("WordPress: admin UI preferences");
    }
    if name.starts_with("wordpress_logged_in_") {
        return Some("WordPress: authenticated login state");
    }
    if name.starts_with("ajs_") {
        return Some("Segment: analytics event metadata");
    }
    None
}
