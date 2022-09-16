# Veloren Server Browser

[![pipeline status](https://gitlab.com/veloren/serverbrowser/badges/master/pipeline.svg)](https://gitlab.com/veloren/veloren/commits/master) [![discord](https://img.shields.io/discord/449602562165833758.svg)](https://discord.gg/WEXSY9h) [![lines of code](https://tokei.rs/b1/gitlab/veloren/serverbrowser)](https://tokei.rs/b1/gitlab/veloren/serverbrowser)

# Adding a Server

To request a new server to be added to the Server Browser please raise a new issue using the `New Server` issue template. There are several requirements for servers to be listed in the server browser:

1) The server must be using the official https://auth.veloren.net auth server (this restriction will be removed in the future following the replacement of our current auth solution)
2) The server must auto-update to the weekly channel. A docker-compose.yml file which includes server auto-updating to the Weekly channel using [Watchtower](https://containrrr.dev/watchtower/) is available here: https://gitlab.com/veloren/veloren/-/blob/master/server-cli/docker-compose.yml
3) Emojis must not be used in the server name or description as Airshipper does not currently support rendering emoji characters.
3) Whilst not mandatory, it is suggested that the server be configured to respond to ICMP ping requests otherwise its status will show as _Error_ in the server browser.