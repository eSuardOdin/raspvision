# Raspvision

Little project in order to gain some knowledge about rust and OS / Lowlevel.

Cheating a little bit as i'm using the crate sysinfo

## Sysinfo doc :
https://docs.rs/sysinfo/latest/sysinfo/


## Data :
```json
{
    "hostname": "string",
    "os": {
        "name": "string",
        "version": "string"
    },
    "cpu": {
        "model": "string",
        "architecture": "string",
        "cores": "int",
        "frequency": "int",
        "usage": "int",
        "temperature": "int"
    },
    "interfaces": [
        {
            "name": "string",
            "ipv4": "integer",
            "subnet": "integer"
        }
    ],
    "memory": {
        "total": "int",
        "free": "int",
        "used": "int"
    },
    "storage": [
        {
            "device": "string",
            "filesystem": "string",
            "total": "int",
            "free": "int",
            "used": "int"
        }
    ],
    "uptime": "int",
    "load_average": {
        "one_minute": "float",
        "five_minutes": "float",
        "fifteen_minutes": "float"
    },
    "processes": {
        "running": "int",
        "total": "int"
    }
}

```
------

Maybe for later if I need to create the wheel once again :
- Get info about memory -> /proc/meminfo
- 