Feature: generator

  The media generators of a Verdant Prism, in app/src/Studio/Generator/. Each
  one is a definition of Studio's app, so prism-code builds it into the app's
  Core-Wasm guest (Studio.Generator.Guest), and the app answers a generator
  request, generate TAB audio/midi TAB FEATURES, or generate TAB audio/wave TAB FEATURES,, with the file it writes.

  @VS-16 @build
  Scenario: The MIDI generator, Studio.Generator.Music.Midi, writes a Prism's features as a format 0 Standard MIDI File with one track at 96 ticks per quarter note, each feature byte a MIDI 1.0 key number sounded as one quarter note on channel 1
    Given the MIDI generator in app/src/Studio/Generator/Music/Midi.lex.tex
    When prism-code check has Lean's kernel prove Studio.Generator.Music.Midi.word32_is_big_endian, Studio.Generator.Music.Midi.writes_a_format_0_file and Studio.Generator.Music.Midi.writes_one_note_per_feature
    Then the feature E is written as an MThd chunk of format 0, one track and division 96, then one MTrk chunk with a Note On and a Note Off of key 69 and an End of Track
    And prism-code build replays the request for the feature E, answered with that file

  @VS-17 @build @rejects
  Scenario: The MIDI generator refuses a Prism with no features, and names a feature that is not ASCII text, since every byte it writes from a feature is a MIDI 1.0 data byte
    Given the MIDI generator in app/src/Studio/Generator/Music/Midi.lex.tex
    When prism-code check has Lean's kernel prove Studio.Generator.Music.Midi.refuses_no_features, Studio.Generator.Music.Midi.refuses_a_feature_by_name and Studio.Guard.guard_refuses_exactly_non_ascii
    Then a Prism with no features is answered with the refusal
    And prism-code build replays the request with no features, answered with that refusal
    And prism-code build replays the feature café, answered with the refusal that names it

  @VS-18 @build
  Scenario: The synthesis, Studio.Generator.AudioFile.Synthesis, renders the Standard MIDI File the MIDI generator writes as 16-bit PCM at 48 kHz: each sounding key a sine at its MIDI tuning frequency, key 69 at 440 Hz, peaking at 3281, the level at which a 997 Hz tone reads -23 LUFS
    Given the synthesis in app/src/Studio/Generator/AudioFile/Synthesis.lex.tex
    When prism-code check has Lean's kernel prove Studio.Generator.AudioFile.Synthesis.key_69_steps_at_440_hertz, Studio.Generator.AudioFile.Synthesis.sine_starts_at_zero, Studio.Generator.AudioFile.Synthesis.sine_peaks_at_the_level and Studio.Generator.AudioFile.Synthesis.sine_troughs_at_the_negated_level
    Then key 69 advances the phase by 440 Hz at 48 kHz, and the sine starts at 0, peaks at 3281 and falls to -3281

  @VS-19 @build
  Scenario: The BWF generator, Studio.Generator.AudioFile.Bwf, writes a Prism's features as a Broadcast Wave Format version 2 file, RIFF WAVE with bext, fmt (PCM, mono, 48 kHz, 16-bit) and data chunks in that order, carrying the synthesis of the MIDI generator's file
    Given the BWF generator in app/src/Studio/Generator/AudioFile/Bwf.lex.tex
    When prism-code check has Lean's kernel prove Studio.Generator.AudioFile.Bwf.le32_is_little_endian, Studio.Generator.AudioFile.Bwf.format_is_pcm_mono_48000_16, Studio.Generator.AudioFile.Bwf.extension_is_version_2_with_coding_history, Studio.Generator.AudioFile.Bwf.wave_orders_bext_fmt_data, Studio.Generator.AudioFile.Bwf.wave_carries_version_2, Studio.Generator.AudioFile.Bwf.wave_carries_the_coding_history, Studio.Generator.AudioFile.Bwf.wave_carries_fmt_then_data and Studio.Generator.AudioFile.Bwf.wave_ends_with_the_samples
    Then the file opens with RIFF, WAVE and a 646-byte bext chunk of version 2 whose description is the features and whose coding history is A=PCM,F=48000,W=16,M=mono,T=Verdant Studio
    And a one-sample file of 700 bytes carries version 2 at byte 366, the coding history at byte 622, then the fmt chunk and a data chunk holding that sample
    And prism-code build replays the BWF request with no features and the feature café, answered with the refusals of the MIDI generator
