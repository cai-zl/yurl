# yurl

[简体中文](./README-ch.md)

> meaning of name: yaml url.

this is a small project for learning the rust language.

its main function is to initiate http requests through yaml files.

# usage

## example yaml file: 

```yaml
vars:
  name: tom
  prefix: http://127.0.0.1:8000
requests:
  - order: 1
    name: post-form
    url: ${var.prefix}/post/form
    method: POST
    headers:
    params:
      name: post-form
    content_type: FORM
    response_type: JSON
  - order: 3
    name: post-json
    url: ${var.prefix}/post/json
    method: POST
    headers:
    params:
      name: ${res.post-form.code}
    content_type: JSON
    response_type: JSON
  - order: 6
    name: post-url
    url: ${var.prefix}/post/url
    method: POST
    headers:
    params:
      name: ${fun.datetime}
    content_type: URLENCODED
    response_type: JSON
  - order: 2
    name: get-url
    url: ${var.prefix}/get/url
    method: GET
    headers:
    params:
      name: get-url
    content_type: URLENCODED
    response_type: JSON
  - order: 5
    name: get-form
    url: ${var.prefix}/get/form
    method: GET
    headers:
    params:
      name: get-form
    content_type: FORM
    response_type: JSON
  - order: 8
    name: get-json
    url: ${var.prefix}/get/json
    method: GET
    headers:
    params:
      name: get-json
    content_type: JSON
    response_type: JSON
  - order: 7
    name: put-form
    url: ${var.prefix}/put/form
    method: PUT
    headers:
    params:
      name: put-form
    content_type: FORM
    response_type: JSON
  - order: 22
    name: put-json
    url: ${var.prefix}/put/json
    method: PUT
    headers:
    params:
      name: put-json
    content_type: JSON
    response_type: JSON
  - order: 13
    name: put-url
    url: ${var.prefix}/put/url
    method: PUT
    headers:
    params:
      name: put-url
    content_type: URLENCODED
    response_type: JSON
  - order: 17
    name: delete-form
    url: ${var.prefix}/delete/form
    method: DELETE
    headers:
    params:
      name: delete-form
    content_type: FORM
    response_type: JSON
  - order: 16
    name: delete-json
    url: ${var.prefix}/delete/json
    method: DELETE
    headers:
    params:
      name: delete-json
    content_type: JSON
    response_type: JSON
  - order: 0
    name: delete-url
    url: ${var.prefix}/delete/url
    method: DELETE
    headers:
    params:
      name: ${fun.uuid}
    content_type: URLENCODED
    response_type: JSON
```

## command

### run

run commands.

```shell
yurl run -f ./test.yaml -p

# run : run subcommand
# --file | -f: specify file path
# --pretty | -p: pretty output

# ╭───────┬─────────────┬────────┬───────────────────────────────────┬─────────────────────────────────┬─────────┬────────────────────────────────────────────────────────────────────────╮
# │ order │ name        │ method │ url                               │ params                          │ headers │ response                                                               │
# ├───────┼─────────────┼────────┼───────────────────────────────────┼─────────────────────────────────┼─────────┼────────────────────────────────────────────────────────────────────────┤
# │ 0     │ delete-url  │ DELETE │ http://127.0.0.1:8000/delete/url  │ {"name": "${fun.uuid}"}         │ {}      │ {"code":200,"message":"success","data":{"name":"${fun.uuid}"}}         │
# │ 1     │ post-form   │ POST   │ http://127.0.0.1:8000/post/form   │ {"name": "post-form"}           │ {}      │ {"code":200,"message":"success","data":{"name":"post-form"}}           │
# │ 2     │ get-url     │ GET    │ http://127.0.0.1:8000/get/url     │ {"name": "get-url"}             │ {}      │ {"code":200,"message":"success","data":{"name":"get-url"}}             │
# │ 3     │ post-json   │ POST   │ http://127.0.0.1:8000/post/json   │ {"name": "200"}                 │ {}      │ {"code":200,"message":"success","data":{"name":"200"}}                 │
# │ 5     │ get-form    │ GET    │ http://127.0.0.1:8000/get/form    │ {"name": "get-form"}            │ {}      │ {"code":200,"message":"success","data":{"name":"get-form"}}            │
# │ 6     │ post-url    │ POST   │ http://127.0.0.1:8000/post/url    │ {"name": "2024-06-14 21:41:22"} │ {}      │ {"code":200,"message":"success","data":{"name":"2024-06-14 21:41:22"}} │
# │ 7     │ put-form    │ PUT    │ http://127.0.0.1:8000/put/form    │ {"name": "put-form"}            │ {}      │ {"code":200,"message":"success","data":{"name":"put-form"}}            │
# │ 8     │ get-json    │ GET    │ http://127.0.0.1:8000/get/json    │ {"name": "get-json"}            │ {}      │ {"code":200,"message":"success","data":{"name":"get-json"}}            │
# │ 13    │ put-url     │ PUT    │ http://127.0.0.1:8000/put/url     │ {"name": "put-url"}             │ {}      │ {"code":200,"message":"success","data":{"name":"put-url"}}             │
# │ 16    │ delete-json │ DELETE │ http://127.0.0.1:8000/delete/json │ {"name": "delete-json"}         │ {}      │ {"code":200,"message":"success","data":{"name":"delete-json"}}         │
# │ 17    │ delete-form │ DELETE │ http://127.0.0.1:8000/delete/form │ {"name": "delete-form"}         │ {}      │ {"code":200,"message":"success","data":{"name":"delete-form"}}         │
# │ 22    │ put-json    │ PUT    │ http://127.0.0.1:8000/put/json    │ {"name": "put-json"}            │ {}      │ {"code":200,"message":"success","data":{"name":"put-json"}}            │
# ╰───────┴─────────────┴────────┴───────────────────────────────────┴─────────────────────────────────┴─────────┴────────────────────────────────────────────────────────────────────────╯
```

### function

function commands.

#### list all functions
```yaml
yurl function list

# or
# yurl --fun -l
# function | --fun: function subcommand
# list | -l: list all functions

# ╭──────────────────┬───────────────────────────────┬─────────────────────╮
# │ key              │ about                         │ result              │
# ├──────────────────┼───────────────────────────────┼─────────────────────┤
# │ date             │ get current date.             │ 2024-06-14          │
# │ date_max         │ get current date max time.    │ 2024-06-14 23:59:59 │
# │ date_min         │ get current date min time.    │ 2024-06-14 00:00:00 │
# │ datetime         │ get current datetime.         │ 2024-06-14 21:46:18 │
# │ time             │ get current time.             │ 21:46:18            │
# │ timestamp        │ get current timestamp.        │ 1718372778          │
# │ timestamp_millis │ get current timestamp millis. │ 1718372778422       │
# ╰──────────────────┴───────────────────────────────┴─────────────────────╯
```

#### call a function

```yaml
yurl --fun -c -k date
# or
# yurl function call --key date
# function | --fun: function subcommand
# call | -c: call a function
# --key | -k: key of a function

# 2024-06-14
```

#### search function

```yaml
yurl --fun -s -k date
# or
# yurl function search --key date
# function | --fun: function subcommand
# search | -s: search function
# --key | -k: key of a function(fuzzy query)

# ╭──────────┬────────────────────────────┬─────────────────────╮
# │ key      │ about                      │ result              │
# ├──────────┼────────────────────────────┼─────────────────────┤
# │ date     │ get current date.          │ 2024-06-14          │
# │ date_max │ get current date max time. │ 2024-06-14 23:59:59 │
# │ date_min │ get current date min time. │ 2024-06-14 00:00:00 │
# │ datetime │ get current datetime.      │ 2024-06-14 21:54:07 │
# ╰──────────┴────────────────────────────┴─────────────────────╯
```

### generate

generate commands.

```yaml
yurl generate -o template.yaml
# or
# yurl --gen --out template.yaml

# please view template.yaml
```

template.yaml: 

```yaml
# variable, use ${var.name} can obtain.
vars:
  name: tom
  host: 127.0.0.1
# request list
requests:
  # request execution order
  - order: 2
    # request name
    name: hello
    # request url, can use expression, example: http://${var.host}:8080/hello
    url: http://127.0.0.1:8080/hello
    # request method: GET | POST | PUT | DELETE
    method: POST
    # request headers, can use expression.
    headers:
      tenant-id: 10000
      # get variable value expression.
      name: ${var.name}
      # get function value expression, function list can be viewed through [yurl function list].
      uuid: ${fun.uuid}
      # get response value expression, only when the dependent request response_type=JSON.
      # expression describe: ${res.    hello.                             token}
      #                        fixed   dependent request the name         dependent request the response json path
      token: ${res.hello.token}
    # request params, can use expression.
    params:
      name: ${var.name}
      id: ${fun.uuid}
    # request ContentType: URLENCODED | FORM | JSON | FILE
    content_type: JSON
    # response data type: TEXT | JSON | HTML | FILE
    response_type: JSON
```