# clashrsctl

a simple tool to manage clash

# supported apis

|uri|method|description|status|
|---|---|---|---|
|`/trafic`|`GET`|get current traffic|x|
|`/logs`|`GET`|get real time logs|x|
|`/proxies`|`GET`|get the list of proxies|WIP|
|`/proxies/:name`|`GET`|get the information of proxy `name`|WIP|
|`/proxies/:name/delay`|`GET`|get the delay of proxy `name`|x|
|`/proxies/:name`|`POST`|change the selected proxy|x|
|`/configs`|`GET`|get the current configuration|o|
|`/configs`|`PATCH`|change the configuration incrementally|o|
|`/configs`|`PUT`|reload the configuration|o|
|`/rules`|`GET`|get the rules|o|

# Todo

1. `rules` api support: o
2. `configs` api support: o
3. `proxies` api support: x
4. `logs` api support: x
5. `trafic` api support: x
6. CLI: WIP

