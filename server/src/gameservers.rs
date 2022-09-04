use isocountry::alpha2;
use lazy_static::lazy_static;
use veloren_serverbrowser_api::{GameServer, GameServerList};

lazy_static! {
    pub static ref SERVERS: GameServerList = GameServerList {
        servers: vec!(
            // Officail Servers
            GameServer::new("Official Server", "veloren.net", 14004, "discord:  https://discord.gg/ecUxc9N", country_parser::parse(alpha2::ISO_A2_DEU), Some("https://auth.veloren.net"), Some("weekly"), true),
            // KIRO♡#7113
            small("VELOREN LATAM 🔥", "server.velorenlatam.net", "discord: https://discord.gg/uU5ATbvCjv website: https://velorenlatam.net/", alpha2::ISO_A2_CHL),
            // Samuellm#5373
            small("South America Server", "velorenbr.com", "discord: https://discord.gg/X39tWScpYQ reddit: https://www.reddit.com/r/Veloren/comments/vbbtx2/south_america_server_velorensamuellmdev/", alpha2::ISO_A2_BRA),
            // Kalgator#0001
            small("Veloren ES", "server.veloren.es", "discord: discord.veloren.es - 24/7 dedicated server hosted on France OVH for Spanish community", alpha2::ISO_A2_FRA),
            // Zodurus#0001
            small("Endcube Veloren Server (Asia-Hosted)", "veloren.endcube.net", "", "alpha2::ISO_A2_SGP"),
            // HereticErik#1079
            small("Heretic.Network Community Server", "gaming.heretic.network", "hosted (East Coast GMT-5) - 24/7 fiber internet for the Heretic.Network community, however everyone is welcome to freely play on it without membership to our community", alpha2::ISO_A2_USA),
            // Konstantin_#4377
            small("[SWAMP.LOL] Community Gameserver", "swamp.lol", "discord: https://discord.gg/pmeYZaaqc3 telegram: https://t.me/swamplol - Friendship community safespace server - Ламповый сервер для анона", alpha2::ISO_A2_RUS),
            // HostEZ_Admin#4504
            small("HostEZ Official Veloren (Vanilla)", "51.222.146.193", "discord: https://hostez.io/discord - Join us in Veloren for a fun Vanilla experience with low latency and high performance. Enjoy! - The server automatically updates every night at 0:00 Pacific Time and restarts at 0:05 with the latest version", alpha2::ISO_A2_CAN),
            // Sand Kingston#1307
            small("VLRN", "vlrn.duckdns.org", "O servidor caiu por 2 dias porque esqueci de pagar a conta de luz. Caso algo mais aconteça, vou colocar os updates em: https://discord.gg/9Uexb5zsn8 ", alpha2::ISO_A2_BRA),
            // Pretzelise#8782
            small("Elise.GG :: Veloren", "veloren.elise.gg", "Currently a basic Veloren server very close to stock. Going to try to add some stuff while I learn more about the game. Happy to take suggestions for improvements! ", alpha2::ISO_A2_AUS),
            // EvanMeek#2724
            small("WECW", "sadj.tpddns.net", "DownloadLink: https://github.com/EvanMeek/veloren-wecw/releases - Large-scale custom mod veloren. WECW mainly shows the world of Warcraft style multiplayer team experience, as well as a new currency system, equipment system and skill development. - 大型自定义模组版本的Veloren. WECW主要展示魔兽世界风格的大型多人团队体验, 其相比原版有全新的货币，装备，技能系统。", alpha2::ISO_A2_CHN),
            // milan-ihl.de
            GameServer::new("tchncs.de Community Gameserver", "veloren.tchncs.de", 14004, "discord: https://discord.gg/yEpgxCKb matrix: #veloren-server:tchncs.de irc ##veloren-tchncs on Libera.Chat (Info & Signup) tchncs.de/veloren - This server is mainly and heavily promoted on the tchncs Mastodon and Matrix instance in an attempt of bringing users closer together. As of the time of writing, we've just declared saturday to Veloren-day. Of course the server is not meant to be exclusive to tchncs users and everybody is welcome.", country_parser::parse(alpha2::ISO_A2_DEU), Some("https://auth.veloren.tchncs.de"), Some("weekly"), false),
            // Nodge#5537
            small("[BR/SA] No name, no admins, same settings as main", "veloren.wisp.run", "24/7", alpha2::ISO_A2_BRA),
            // cupanoodle#7543
            small("[No name - Suggest one!", "joeisthebest.mooo.com", "US West - I'm hoping to make the server a bit more lively, hop on and have some fun. ", alpha2::ISO_A2_USA),
            // zesterer#3131
            small("Zesterer's experimental build server", "168.119.172.252", "Hopefully Airshipper compatible. If you get a hard error when connecting, ping me and I will update the server!", ""),
            // freezee#4242
            small("Currently unnamed", "lbenard.dev", "West EU - I haven't properly set up the server yet - It's hosted on a basic VPS but can be scaled if deemed useful. Ping-wise I'm getting at most 40ms when travelling (Paris)", alpha2::ISO_A2_FRA),
        )
    };
}

pub fn small(name: &str, hostname: &str, desc: &str, location: &str) -> GameServer {
    GameServer::new(
        name,
        hostname,
        14004,
        desc,
        country_parser::parse(location),
        Some("https://auth.veloren.net"),
        Some("weekly"),
        false,
    )
}
