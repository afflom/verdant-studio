Feature: svg

  The vector image generator Studio.Generator.VectorImage.Svg, in
  app/src/Studio/Generator/VectorImage/Svg.lex.tex: a Core-Wasm guest that
  writes a Prism's seed as an SVG 2 document, the seed as the text of one text
  element. The app answers generate TAB image/svg+xml TAB SEED.

  @VS-14 @build @rejects
  Scenario: The SVG generator refuses by name an empty seed
    Given the SVG model in app/src/Studio/Generator/VectorImage/Svg.lex.tex
    When prism-code check has Lean's kernel prove Studio.Generator.VectorImage.Svg.refuses_the_empty_seed
    Then an empty seed is answered with the empty-seed refusal
    And prism-code build replays the request generate TAB image/svg+xml TAB, answered with that refusal

  @VS-15 @build @rejects
  Scenario: The SVG generator writes only XML text: it escapes & < and >, accepts UTF-8 text, and refuses by name a seed with a control character, a surrogate, a noncharacter or a malformed sequence
    Given the SVG model in app/src/Studio/Generator/VectorImage/Svg.lex.tex
    When prism-code check has Lean's kernel prove Studio.Generator.VectorImage.Svg.escapes_markup, Studio.Generator.VectorImage.Svg.accepts_utf8_text, Studio.Generator.VectorImage.Svg.refuses_a_control_character and Studio.Generator.VectorImage.Svg.refuses_a_surrogate_and_a_noncharacter
    Then R&D <one> is written as R&amp;D &lt;one&gt;
    And prism-code build replays the seed a followed by U+0001, answered with the XML-text refusal
