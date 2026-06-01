#!/usr/bin/env nu

def main [
    dir: string = "."
    --plugin: string = "fswatch-watcher/1.0"
    --language: string = ""
] {
    let abs = ($dir | path expand)
    print $"Watching ($abs)"

    inotifywait -r -m -e close_write --format '%w%f' $abs
    | lines
    | each { |file|
        let args = [--entity $file --plugin $plugin --write]
        let args = if $language != "" { $args | append [--language $language] } else { $args }
        run-external "wakatime-cli" ...$args
    }
}
