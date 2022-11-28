use lazy_static::lazy_static;
use veloren_serverbrowser_api::GameServerList;

const OFFICIAL_AUTH: &str = "https://auth.veloren.net";
const OFFICIAL_CHANNEL: &str = "weekly";

lazy_static! {
    pub static ref SERVERS: GameServerList =
        ron::de::from_reader::<_, GameServerList>(&include_bytes!("../servers.ron")[..],).unwrap();
    pub static ref SERVERS_LIMITED: GameServerList = GameServerList {
        servers: SERVERS
            .servers
            .iter()
            .cloned()
            .filter(|f| f.auth_server == *OFFICIAL_AUTH)
            .filter(|f| f.channel == Some(OFFICIAL_CHANNEL.to_string()))
            .collect()
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use regex::Regex;
    use url::{Host, Origin, Url};
    use veloren_serverbrowser_api::FieldContent;

    const EXTRA_ID_REGEX: &str = r"^[a-z][a-z_]*$";

    #[test]
    fn check_server_list_valid() {
        ron::de::from_reader::<_, GameServerList>(&include_bytes!("../servers.ron")[..]).unwrap();
    }

    #[test]
    fn check_official_servers() {
        SERVERS.servers.iter().filter(|s| s.official).for_each(|s| {
            assert!(s.address.ends_with("veloren.net"));
            assert_eq!(s.auth_server, OFFICIAL_AUTH.to_string());
        });
    }

    #[test]
    fn verify_list_is_pretty() {
        let pretty =
            ron::ser::to_string_pretty(&*SERVERS, ron::ser::PrettyConfig::default()).unwrap();
        let original = std::str::from_utf8(include_bytes!("../servers.ron")).unwrap();
        assert_eq!(original.trim(), pretty);
    }

    #[test]
    fn verify_filter_works() {
        assert_eq!(
            SERVERS_LIMITED
                .servers
                .iter()
                .find(|x| x.auth_server != *OFFICIAL_AUTH),
            None
        );
        assert_eq!(
            SERVERS_LIMITED
                .servers
                .iter()
                .find(|x| x.channel != Some(OFFICIAL_CHANNEL.to_string())),
            None
        )
    }

    #[test]
    fn verify_field_id_valid() {
        let re = Regex::new(EXTRA_ID_REGEX).unwrap();
        assert_eq!(
            SERVERS_LIMITED
                .servers
                .iter()
                .find(|x| x.extra.iter().any(|(id, _)| !re.is_match(id))),
            None
        );
    }

    #[test]
    fn verify_field_content_no_unknown() {
        assert_eq!(
            SERVERS_LIMITED.servers.iter().find(|x| x
                .extra
                .iter()
                .any(|(_, field)| matches!(field.content, FieldContent::Unknown))),
            None
        );
    }

    #[test]
    fn verify_field_content_urls_valid() {
        assert_eq!(
            SERVERS_LIMITED
                .servers
                .iter()
                .find(|x| x.extra.iter().any(|(_, field)| match &field.content {
                    FieldContent::Url(u) => Url::parse(u).is_err(),
                    _ => false,
                })),
            None
        );
    }

    /// in order to not trick people we should only provide discord URLs starting with `https://discord.gg/`
    #[test]
    fn verify_discord_urls() {
        let discord_origin: Origin = Origin::Tuple(
            String::from("https"),
            Host::Domain(String::from("discord.gg")),
            443,
        );
        assert_eq!(
            SERVERS_LIMITED
                .servers
                .iter()
                .find(|x| x.extra.iter().any(|(id, field)| match &field.content {
                    FieldContent::Url(u) if *id == "discord" =>
                        Url::parse(u).unwrap().origin() != discord_origin,
                    _ => false,
                })),
            None
        );
    }
}
