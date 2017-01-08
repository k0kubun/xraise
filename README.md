# Xraise

A command to raise X Window, which is like a following shell script but faster.

```bash
command="..."

while read line; do
  pid="$(echo "$line" | cut -d" " -f4)"
  if [ "x${command}" = "x$(cat "/proc/${pid}/cmdline")" ]; then
    window_id="$(echo "$line" | cut -d" " -f1)"
    exec wmctrl -i -R "$window_id"
  fi
done <<< "$(wmctrl -l -p)"

exec "${command}"
```

## Installation

```bash
$ cargo install xraise
```

## Usage

```bash
$ xraise
24183 /opt/google/chrome/chrome: 'GitHub - Google Chrome'
24183 /opt/google/chrome/chrome: 'LINE'
22179 /usr/lib/slack/slack: 'Slack - k0kubun'
26781 /usr/share/nocturn/Nocturn: 'Nocturn'
27546 urxvt: 'urxvt'

# Activate or launch slack, urxvt
$ xraise /usr/lib/slack/slack
$ xraise urxvt

# Activate or launch Google Chrome, LINE
$ xraise /opt/google/chrome/chrome "Google Chrome" # tail match
$ xraise /opt/google/chrome/chrome LINE
```

## Notice

You may prefer `wmctrl -x -l` and `wmctrl -x -a $WM_CLASS`.

## License

MIT License
