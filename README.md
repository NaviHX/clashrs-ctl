# clashrsctl

a simple tool to manage clash

# supported apis

|uri|method|description|status|
|---|---|---|---|
|`/trafic`|`GET`|get current traffic|x|
|`/logs`|`GET`|get real time logs|x|
|`/proxies`|`GET`|get the of proxies|x|
|`/proxies/:name`|`GET`|get the information of proxy `name`|x|
|`/proxies/:name/delay`|`GET`|get the delay of proxy `name`|x|
|`/proxies/:name`|`POST`|change the selected proxy|x|
|`/configs`|`GET`|get the current configuration|WIP|
|`/configs`|`PATCH`|change the configuration incrementally|WIP|
|`/configs`|`PUT`|reload the configuration|WIP|
|`/rules`|`GET`|get the rules|WIP|

