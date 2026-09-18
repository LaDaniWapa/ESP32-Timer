# Screens & buttons

## Buttons

8 physical buttons:

- **Mode** — cycle between Clock / Pomodoro contexts
- **Snooze / Light / OK** — wake screen, confirm/select, snooze alarm (context-dependent)
- **Up** / **Down** — navigate lists, adjust values
- **Settings** — tap: context menu (alarms or pomodoro settings). Hold: jump to `Global_Settings` from anywhere
- **OFF** — tap: turn off backlight. Hold: delete selected item (in list screens)

**Global conventions:**

- **Tap** = quick/reversible action. **Hold** = heavier/destructive or global action. Applied consistently to `Settings`
  and `OFF` everywhere they appear.
- **Timeout** turns off the **backlight only** (not the display's VCC/power), so the controller keeps its
  configuration (orientation, color mode, etc.) and `flush()` keeps working normally when the screen wakes up — no
  re-init needed.
- Internally, Rust logic keeps running on whatever screen it was on when the backlight turned off — nothing is saved or
  reset by a timeout.
- 24h time format throughout (no AM/PM yet).

---

## Clock_Screen

Shows the current time.

| Button          | Action                |
|-----------------|-----------------------|
| Mode            | → `Pomodoro_Settings` |
| Settings (tap)  | → `Alarms_Screen`     |
| Settings (hold) | → `Global_Settings`   |
| Snooze / Light  | Wake screen           |
| Up / Down       | Nothing               |
| OFF (tap)       | Turn off backlight    |
| Timeout         | Turn off backlight    |

---

## Clock_Alarm

An alarm is ringing, screen is on.

| Button              | Action                             |
|---------------------|------------------------------------|
| Snooze              | → `Clock_Screen` (stops the alarm) |
| *(everything else)* | Nothing                            |

---

## Pomodoro_Alarm

A study/break interval just ended. No input needed — plays a long beep and automatically advances to the next interval.

| Button        | Action                                                                 |
|---------------|------------------------------------------------------------------------|
| Snooze (hold) | Exit Pomodoro entirely → `Clock_Screen`                                |
| Mode          | → `Clock_Screen` (view time), Pomodoro keeps running in the background |

---

## Pomodoro_Settings

Configure study/break intervals, shown as a flat list:

```
Study   [45] mins
Break   [10] mins
Study   [45] mins
Break   [10] mins
Study   [45] mins
Break   [30] mins
Add     [+]
```

| Button          | Action                                     |
|-----------------|--------------------------------------------|
| Mode            | → `Clock_Screen`                           |
| Settings (tap)  | Nothing (already here)                     |
| Settings (hold) | → `Global_Settings`                        |
| Up / Down       | Move the visual selection between items    |
| Snooze / Light  | Select highlighted item → enters edit mode |
| OFF (tap)       | Nothing                                    |
| OFF (hold)      | Delete highlighted item                    |

### Pomodoro_Settings — editing an existing interval (`editing = true`)

| Button   | Action                           |
|----------|----------------------------------|
| Up       | +5 minutes (wraps 5–120)         |
| Down     | -5 minutes (wraps 5–120)         |
| Snooze   | Save, `editing = false`          |
| Settings | Locked (no-op)                   |
| Mode     | Beep (reminder: unsaved changes) |

### Pomodoro_Settings — Add flow

Two-step confirmation:

1. Select **Study** or **Break** (defaults to the opposite of the previous item — e.g. previous is Break → new defaults
   to Study) → OK
2. Select minutes (5–120, wraps, step of 5) → OK
3. Saved and inserted into the list

If you pick the wrong type in step 1, delete the item afterward and re-add (same as any other list item).

---

## Pomodoro_Screen

Shows remaining minutes and whether the current interval is Study or Break.

| Button          | Action                                                       |
|-----------------|--------------------------------------------------------------|
| Mode            | → `Clock_Screen` (view time), Pomodoro keeps running         |
| Settings (tap)  | → `Pomodoro_Settings`, **stops** the running Pomodoro        |
| Settings (hold) | → `Global_Settings`                                          |
| Snooze / Light  | Wake screen                                                  |
| Up / Down       | Nothing                                                      |
| OFF (tap)       | Turn off backlight, Pomodoro keeps running in the background |
| Timeout         | Turn off backlight (same as OFF tap)                         |

---

## Alarms_Screen

List of configured alarms:

```
08:00     [ ]
10:30     [x]
18:45     [x]
Add       [+]
```

| Button          | Action                                                         |
|-----------------|----------------------------------------------------------------|
| Mode            | → `Clock_Screen`                                               |
| Settings (hold) | → `Global_Settings`                                            |
| Up / Down       | Move the visual selection between items                        |
| Snooze (tap)    | Toggle selected alarm on/off                                   |
| Snooze (hold)   | Edit selected alarm's time (enters the "big" hour/minute flow) |
| OFF (hold)      | Delete selected alarm                                          |

### Alarms_Screen — Add / Edit flow ("big" display)

Two-step confirmation, 24h format:

1. Select **hour** (0–23, wraps) → OK
2. Select **minute** (0–59, wraps) → OK
3. Saved

---

## Global_Settings

```
Theme            Red >
TimeZone      Madrid >
TimeOut      [10] mins
```

Two kinds of rows:

- **`Label   Value >`** — opens a dropdown-style list of options. Up/Down moves through the options, Snooze selects, OFF
  cancels the selection (closes without changing).
    - **Theme**: list of available color themes
    - **TimeZone**: list of available timezones
- **`Label   [N] unit`** — inline numeric value. Select it, then Up/Down adjusts it directly (its own step/range).
    - **TimeOut**: 0–30 minutes, step TBD (mins). **0 disables the timeout** (backlight stays on permanently) — useful
      if the device stays plugged in rather than running on battery.

- **TimeOut** step: 5 minutes.

| Button      | Action                                                           |
|-------------|------------------------------------------------------------------|
| Up / Down   | Move between rows / adjust value or option, depending on mode    |
| Snooze / OK | Confirm selection (enter a `>` submenu, or confirm a `[N]` edit) |
| OFF         | Cancel current selection / close submenu without saving          |
| Mode        | Locked (no-op)                                                   |
| Settings    | Locked (no-op)                                                   |

**Timeout behavior while inside `Global_Settings`:** the *previously saved* timeout value keeps applying globally (
including to this screen itself) until the new value is confirmed/saved here. This global timeout check is planned to
live in the UI thread, not per-screen.