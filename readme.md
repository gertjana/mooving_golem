# Mooving on Golem

This is a PoC to play around with [Golem](https://golem.cloud/)

## About the application

A `Moovable` is something that keeps track of distance moved, in my case my bike and my (lease) car

The mooving app is an app to keep track of my moovables and be able to update distance traveled, and do a linear extrapolation to see how much I moved at a certain date.

The original App is a command line app in Elixir which I use to keep track of my cycling, and make sure I don't go over the lease limit of my car.

Since then I use it to learn new languages and frameworks

## About the Golem PoC

Functionality is split into:

- business logic with tests in `./mooving`
- webassembly module in `./wasm` where the wit (webassembly system interface) and conversion methods between the wit structs and the business logic structs are defined

## build it

```bash
./test_and_build.sh
```

this will check/test/fmt/lint the code, build the webassembly binary and should result in a `mooving.wasm` file in the root directory

## run it

```bash
> golem component add -c mooving_comp ./mooving.wasm
gcomponentId: 1aeb6392-3415-492d-9a0a-f6db43fec746
componentVersion: 0
componentName: mooving_comp
componentSize: 476643
exports: 
  - left out for brevity

> golem instance add -c mooving_comp -i mooving_inst
instanceId:
  rawComponentId: 1aeb6392-3415-492d-9a0a-f6db43fec746
  instanceName: mooviong_inst
componentVersionUsed: 0

> golem instance invoke-and-await -c mooving_comp -i mooving_inst -F json \
        -f mooving:moovables/api/add-moovable -j '["my-bike", "bike"]'
[
  {
    "ok" : {
      "name" : "my-bike",
      "moovable-type" : "bike",
      "current" : false,
      "data" : []
    }
  }

> golem instance invoke-and-await -c mooving_comp -i mooving_inst -F json \ 
        -f mooving:moovables/api/add-data -j '["my-bike", {"km":100, "date": "2023-08-05"}]'
[
  {
    "ok" : {
      "name" : "my-bike",
      "moovable-type" : "bike",
      "current" : false,
      "data" : [
        {
          "date" : "2023-08-05",
          "km" : 100
        }
      ]
    }
  }
]
```

and after adding a few more data points, we can do a trend analysis

```bash
> golem instance invoke-and-await -c mooving_comp -i mooving_inst -F json \
        -f mooving:moovables/api/get-moovable -j '["my-bike"]'
[
  {
    "ok" : {
      "name" : "my-bike",
      "moovable-type" : "bike",
      "current" : false,
      "data" : [
        {
          "date" : "2023-08-10",
          "km" : 190
        },
        {
          "date" : "2023-08-07",
          "km" : 156
        },
        {
          "date" : "2023-08-05",
          "km" : 100
        }
      ]
    }
  }
]

> golem instance invoke-and-await -c mooving_comp -i mooving_inst -F json \ 
        -f mooving:moovables/api/get-trend -j '["my-bike", "2023-08-31"]'
[
  {
    "ok" : {
      "average-per-day" : 19.666666666666668,
      "days-till-end_date" : 21,
      "average-to-reach-goal" : 45.285714285714285,
      "goal" : 2000,
      "total" : 1049,
      "this-period" : 413,
      "moovable-name" : "my-bike"
    }
  }
]
```


## TODO

- watch the next webinar to learn new stuff :)
- get the CI working with the cargo component plugin
