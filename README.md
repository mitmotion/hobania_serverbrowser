# Serverbrowser

[![Discord](https://img.shields.io/discord/449602562165833758?label=discord)](https://discord.gg/rvbW3Z4)

## Protocol

`/v1/servers`

```json
{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "title": "Serverbrowser_v1_servers",
  "type": "object",
  "required": [
    "servers"
  ],
  "additionalProperties": false,
  "properties": {
    "servers": {
      "type": "array",
      "items": {
        "$ref": "#/$defs/server"
      }
    }
  },
  "$defs": {
    "server": {
      "type": "object",
      "required": [
        "serverName",
        "hostnameIp",
        "port",
        "location",
        "description",
        "authServer",
        "channel",
        "official"
      ],
      "additionalProperties": false,
      "properties": {
        "serverName": {
          "type": "string",
          "description": "Name of The Server"
        },
        "hostnameIp": {
          "type": "string",
          "description": "Hostname or IP of Server"
        },
        "port": {
          "type": "number",
          "description": "Port of the Server"
        },
        "location": {
          "type": "string",
          "description": "country_code location of the server"
        },
        "description": {
          "type": "string",
          "description": "Description of the Server"
        },
        "authServer": {
          "type": "string",
          "description": "FQDN of auth server used"
        },
        "channel": {
          "type": "string",
          "description": "channel of the Server"
        },
        "official": {
          "type": "boolean",
          "description": "is the gameserver officially hosted by the veloren developers"
        }
      }
    }
  }
}
```