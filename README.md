# Rinth synthesiser

FM and SSG synthesiser

## Usage

```shell
rinth-synth b[uild] <path/to/project.yml> # Will build separate tracks 
rinth-synth m[aster] <path/to/project.yml> # Will "master" tracks into one track
```

See `--help` for more

## File formats

### \<project\>.yml

```yaml
name: <NAME>
bpm: <BPM>
channels:
  - type: <SSG|FM>
    path: <RELATIVE_TO_YML_PATH>
    volume: [Optional in [0;1]]
```

Example:

```yaml
name: Megalovania
bpm: 210
channels:
  - type: SSG
    path: _channel
  - type: FM
    path: _channel
    volume: 0.7
```

### _channel (FM)

```
/ Comment
#<Frequency deviation>
@<Modulating frequency>
<Note in SPN> <Note value> [Time from previous note]
```

Example:

```
#1
@2000
D4 0.0625
Dd5 0.0625 0.0125
Db5 1/32+1/32 1/80
```

### _channel (SSG)

```
/ Comment
# Ignored
@ Ignored
<Note in SPN> <Note value> [Time from previous note]
```

Example:

```
/ Comment
# Comment 2
@ Comment 3
D4 0.0625
Dd5 0.0625 0.0125
Db5 1/32+1/32 1/80
```
More examples in [examples/](./examples)

## MIDI Converter

WIP MIDI converter is available in the `rinth-midi` crate.

### Usage

```shell
rinth-midi <path/to/midi.mid>
```

## License

The project is available for non-commercial personal use under the terms of the [BSD 3-Clause New (Revised) License](./LICENSE) and fully unavailable for any other kind of use.
