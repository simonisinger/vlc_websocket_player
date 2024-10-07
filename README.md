# VLC Websocket Player

----

This is just a simple full duplex websocket player based on LibVLC and Rust.
The Player will report status changes to all active websocket connections and also can receive simple control commands
over websocket.
## Commands
### play
```json
{
    "command": "play",
    "path": "/a/full/path/to/video.mp4"
}
```

### pause

if its already paused it will resume the video
```json
{
    "command": "pause"
}
```

### stop

```json
{
    "command": "stop"
}
```

### Seeking

#### Seek forward
```json
{
    "command": "seek-forward",
    "seconds": 10
}
```

#### Seek backward
```json
{
    "command": "seek-backward",
    "seconds": 10
}
```


### Exit Player

```
{
    "command": "close-player"
}
```

## Status Messages

The status messages for play, pause and stop are mostly following the same rules, but play has a little difference
because there is a difference between the initial play and the play when it's resumed from pause. at initial play there
will also send the path of video at the status message

### pause, stop
```json
{
    "event": "<event>"
}
```

### initial play
```json
{
    "command": "play",
    "path": "/path/to/video.mp4"
}
```


### current time

this will only sent when a video is playing

the time value passthroughs directly from LibVLC

```json
{
    "command": "status",
    "time": 32444
}
```