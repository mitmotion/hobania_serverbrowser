use isocountry::alpha2;
use lazy_static::lazy_static;

pub use v1::GameServer;

lazy_static! {
    pub static ref SERVERS: Vec<GameServer> = vec!(
        // Officail Servers
        GameServer::with_full("Official Server", "veloren.net", 14004, alpha2::ISO_A2_DEU, "discord:  https://discord.gg/ecUxc9N", "https://auth.veloren.net", "weekly", true),
        // KIRO♡#7113
        GameServer::new("VELOREN LATAM 🔥", "server.velorenlatam.net", alpha2::ISO_A2_CHL, "discord: https://discord.gg/uU5ATbvCjv website: https://velorenlatam.net/"),
        // Samuellm#5373
        GameServer::new("South America Server", "velorenbr.com", alpha2::ISO_A2_BRA, "discord: https://discord.gg/X39tWScpYQ reddit: https://www.reddit.com/r/Veloren/comments/vbbtx2/south_america_server_velorensamuellmdev/"),
        // Kalgator#0001
        GameServer::new("Veloren ES", "server.veloren.es", alpha2::ISO_A2_FRA, "discord: discord.veloren.es - 24/7 dedicated server hosted on France OVH for Spanish community"),
        // Zodurus#0001
        GameServer::new("Endcube Veloren Server (Asia-Hosted)", "veloren.endcube.net", alpha2::ISO_A2_SGP, ""),
        // HereticErik#1079
        GameServer::new("Heretic.Network Community Server", "gaming.heretic.network", alpha2::ISO_A2_USA, "hosted (East Coast GMT-5) - 24/7 fiber internet for the Heretic.Network community, however everyone is welcome to freely play on it without membership to our community"),
        // Konstantin_#4377
        GameServer::new("[SWAMP.LOL] Community Gameserver", "swamp.lol", alpha2::ISO_A2_RUS, "discord: https://discord.gg/pmeYZaaqc3 telegram: https://t.me/swamplol - Friendship community safespace server - Ламповый сервер для анона"),
        // HostEZ_Admin#4504
        GameServer::new("HostEZ Official Veloren (Vanilla)", "51.222.146.193", alpha2::ISO_A2_CAN, "discord: https://hostez.io/discord - Join us in Veloren for a fun Vanilla experience with low latency and high performance. Enjoy! - The server automatically updates every night at 0:00 Pacific Time and restarts at 0:05 with the latest version"),
        // Sand Kingston#1307
        GameServer::new("VLRN", "vlrn.duckdns.org", alpha2::ISO_A2_BRA, "O servidor caiu por 2 dias porque esqueci de pagar a conta de luz. Caso algo mais aconteça, vou colocar os updates em: https://discord.gg/9Uexb5zsn8 "),
        // Pretzelise#8782
        GameServer::new("Elise.GG :: Veloren", "veloren.elise.gg", alpha2::ISO_A2_AUS, "Currently a basic Veloren server very close to stock. Going to try to add some stuff while I learn more about the game. Happy to take suggestions for improvements! "),
        // EvanMeek#2724
        GameServer::new("WECW", "sadj.tpddns.net", alpha2::ISO_A2_CHN, "DownloadLink: https://github.com/EvanMeek/veloren-wecw/releases - Large-scale custom mod veloren. WECW mainly shows the world of Warcraft style multiplayer team experience, as well as a new currency system, equipment system and skill development. - 大型自定义模组版本的Veloren. WECW主要展示魔兽世界风格的大型多人团队体验, 其相比原版有全新的货币，装备，技能系统。"),
        // milan-ihl.de
        GameServer::with_full("tchncs.de Community Gameserver", "veloren.tchncs.de", 14004, alpha2::ISO_A2_DEU, "discord: https://discord.gg/yEpgxCKb matrix: #veloren-server:tchncs.de irc ##veloren-tchncs on Libera.Chat (Info & Signup) tchncs.de/veloren - This server is mainly and heavily promoted on the tchncs Mastodon and Matrix instance in an attempt of bringing users closer together. As of the time of writing, we've just declared saturday to Veloren-day. Of course the server is not meant to be exclusive to tchncs users and everybody is welcome.", "https://auth.veloren.tchncs.de", "weekly", false),
        // Nodge#5537
        GameServer::new("[BR/SA] No name, no admins, same settings as main", "veloren.wisp.run", alpha2::ISO_A2_BRA, "24/7"),
        // cupanoodle#7543
        GameServer::new("[No name - Suggest one!", "joeisthebest.mooo.com", alpha2::ISO_A2_USA, "US West - I'm hoping to make the server a bit more lively, hop on and have some fun. "),
        // zesterer#3131
        GameServer::new("Zesterer's experimental build server", "168.119.172.252", alpha2::ISO_A2_GBR, "Hopefully Airshipper compatible. If you get a hard error when connecting, ping me and I will update the server!"),
        // freezee#4242
        GameServer::new("Currently unnamed", "lbenard.dev", alpha2::ISO_A2_FRA, "West EU - I haven't properly set up the server yet - It's hosted on a basic VPS but can be scaled if deemed useful. Ping-wise I'm getting at most 40ms when travelling (Paris)"),
    );
}

mod v1 {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct GameServer {
        server_name: String,
        hostname_ip: String,
        port: u16,
        location: String,
        description: String,
        auth_server: String,
        channel: String,
        official: bool,
    }

    impl GameServer {
        pub fn new(name: &str, hostname: &str, location: &str, desc: &str) -> Self {
            Self::with_full(
                name,
                hostname,
                14004,
                location,
                desc,
                "https://auth.veloren.net",
                "weekly",
                false,
            )
        }

        pub fn with_full(
            name: &str,
            hostname: &str,
            port: u16,
            location: &str,
            desc: &str,
            auth: &str,
            channel: &str,
            official: bool,
        ) -> Self {
            Self {
                server_name: name.to_string(),
                hostname_ip: hostname.to_string(),
                port,
                location: location.to_string(),
                description: desc.to_string(),
                auth_server: auth.to_string(),
                channel: channel.to_string(),
                official,
            }
        }
    }
}
