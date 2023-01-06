# Community events

Community events are described as `.hcl` files. The events are available:

* as Discord events on the [Rawkode Academy Discord server](https://rawkode.chat)
* as an ICS calendar

## Sample events

```hcl
event {
    topic = "Standup"
    description = "Standup meeting" # optional

    location {
        type = "stage-channel" | "voice-channel" | "somewhere-else" # maps to "Where is your event?" in Discord
        name = "#coffee-bar"
    }

    time {
        start = "2021-01-01T09:00:00Z"
        recurrence = "FREQ=WEEKLY;BYDAY=MO,TU,WE,TH,FR" # subject to change to something more human readable | optional
    }

    image = "https://example.com/image.png" # optional
}
```
