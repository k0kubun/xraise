# Xraise

A command to raise X Window, which is the same as following shell script but faster.

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

## License

MIT License
