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
            .filter(|f| f.auth_server == Some(OFFICIAL_AUTH.to_string()))
            .filter(|f| f.channel == Some(OFFICIAL_CHANNEL.to_string()))
            .collect()
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_server_list_valid() {
        ron::de::from_reader::<_, GameServerList>(&include_bytes!("../servers.ron")[..]).unwrap();
    }

    #[test]
    fn check_official_server() {
        let servers = SERVERS
            .servers
            .iter()
            .filter(|s| s.official)
            .collect::<Vec<_>>();
        assert_eq!(servers.len(), 1, "MORE THAN 1 OFFICIAL SERVER");
        assert_eq!(servers[0].address, "veloren.net");
        assert_eq!(servers[0].auth_server, Some(OFFICIAL_AUTH.to_string()));
    }

    #[test]
    fn verify_list_is_pretty() {
        let pretty =
            ron::ser::to_string_pretty(&*SERVERS, ron::ser::PrettyConfig::default()).unwrap();
        let original = std::str::from_utf8(include_bytes!("../servers.ron")).unwrap();
        assert_eq!(original, pretty);
    }

    #[test]
    fn verify_filter_works() {
        assert_eq!(
            SERVERS_LIMITED
                .servers
                .iter()
                .find(|x| x.auth_server != Some(OFFICIAL_AUTH.to_string())),
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
}
