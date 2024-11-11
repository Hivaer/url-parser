# Overview

The Motivation for this small Project was to get a deeper understanding in coding a state machine on the example of parsing an URL by tokenizing each character and changing state based on the character type.


## Input

```
"https://abc.example.com:443/path/:wildcard/index.html?foo=bar&biz=fiz#fragment"
```

## Output 

```
{
  protocol: "https",
  sub_domain: ["abc"],
  domain: "example",
  top_level_domain: "com",
  port: "443",
  path: [
    "path",
    ":wildcard",
    "index.html"
  ],
  query: {
    "foo": "bar",
    "biz": "fiz"
  },
  fragment: "fragment"
}
```
