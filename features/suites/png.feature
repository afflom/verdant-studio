Feature: png

  The raster image generator Studio.Generator.RasterImage.Png, in
  app/src/Studio/Generator/RasterImage/Png.lex.tex: a Core-Wasm guest that
  encodes a frame as a PNG datastream (W3C PNG 3rd edition), truecolour with
  alpha at bit depth 8 or 16, filter type 0, and zlib stored blocks. The app
  answers generate TAB image/png TAB FRAME, where FRAME is a 4-byte big-endian
  width and height, one bit-depth byte, then the samples row by row.

  @VS-11 @build @rejects
  Scenario: The PNG generator refuses by name a frame whose header is not a 4-byte width and height from 1 to 2147483647 and a bit depth of 8 or 16
    Given the PNG model in app/src/Studio/Generator/RasterImage/Png.lex.tex
    When prism-code check has Lean's kernel prove Studio.Generator.RasterImage.Png.refuses_an_empty_frame
    Then an empty frame is answered with the header refusal
    And prism-code build replays the request generate TAB image/png TAB, answered with that refusal

  @VS-12 @build @rejects
  Scenario: The PNG generator refuses by name a frame whose samples do not fill its width, height and bit depth
    Given the PNG model in app/src/Studio/Generator/RasterImage/Png.lex.tex
    When prism-code check has Lean's kernel prove Studio.Generator.RasterImage.Png.refuses_a_short_frame
    Then a 1 by 1 frame at bit depth 8 with one sample byte is answered with the samples refusal
    And prism-code build replays that request, answered with that refusal

  @VS-13 @build
  Scenario: The PNG generator writes PNG's own datastream: its CRC-32 table is the table PNG's algorithm computes, the CRC of IEND is AE426082, and every image begins with the PNG signature
    Given the PNG model in app/src/Studio/Generator/RasterImage/Png.lex.tex
    When prism-code check has Lean's kernel prove Studio.Generator.RasterImage.Png.crc_table_is_the_standard_table, Studio.Generator.RasterImage.Png.crc_of_iend_is_the_standard_value and Studio.Generator.RasterImage.Png.begins_with_the_png_signature
    Then the 256 table entries equal PNG's make_crc_table, entry by entry
    And prism-code build replays a 2 by 2 frame, answered with the datastream zlib writes at level 0
