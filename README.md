# clashrsctl

a simple tool to manage clash

# supported apis

|uri|method|description|status|
|---|---|---|---|
|`/trafic`|`GET`|get current traffic|o|
|`/logs`|`GET`|get real time logs|o|
|`/proxies`|`GET`|get the list of proxies|o|
|`/proxies/:name`|`GET`|get the information of proxy `name`|o|
|`/proxies/:name/delay`|`GET`|get the delay of proxy `name`|o|
|`/proxies/:name`|`POST`|change the selected proxy|o|
|`/configs`|`GET`|get the current configuration|o|
|`/configs`|`PATCH`|change the configuration incrementally|o|
|`/configs`|`PUT`|reload the configuration|o|
|`/rules`|`GET`|get the rules|o|
|`/version`|`GET`|get the version of clash core|o|
|`/connections`|`GET`|get the connection information|o|
|`/connections/`|`DELETE`|close all connections|o|
|`/connections/:id`|`DELETE`|close specific connections|o|

`Provider` APIs won't be put in consideration in the short term.

# Todo

1. `rules` api support: o
2. `configs` api support: o
3. `proxies` api support: o
4. `logs` api support: o
5. `trafic` api support: o
6. CLI: o
7. `version` api support: o
8. `connection` api support: o

