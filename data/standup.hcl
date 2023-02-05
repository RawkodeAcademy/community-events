event "Standup" {
    description = "Standup meeting"

    discord {
        location = "voice-channel"
        name = "#coffee-bar"
    }

    time {
        start = "16:30:00"

        recurrence {
            daily = ["Mon"]
        }
    }
}
