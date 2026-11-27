Feature: prism

  The Prism generation of REQ-PRISM-GENERATION: one seed, a word or a sentence,
  becomes a Verdant Prism whose facets are the artifacts Studio's generators
  write from it. Studio.Prism.Seed, in app/src/Studio/Prism/Seed.lex.tex, admits
  the seed and specifies Studio's subset of the Prism, its features, from it;
  Studio.PrismGeneration, in app/src/Studio/PrismGeneration.lex.tex, answers
  prism TAB MEDIA-TYPE TAB SEED with the facet of that media type, and
  prism TAB TAB SEED with the list of the Prism's facets.

  @VS-27 @build
  Scenario: A Prism's seed, one word or sentence, is admitted when every generator of the Prism admits it, and a seed a generator refuses is refused with that generator's own refusal
    Given the seed model in app/src/Studio/Prism/Seed.lex.tex
    When prism-code check has Lean's kernel prove Studio.Prism.Seed.admits_a_word_and_a_sentence and Studio.Prism.Seed.refuses_a_seed_a_generator_refuses
    Then the seeds verdant and Fatelines, a game of fate. are admitted
    And the seed café is answered with the MIDI generator's refusal that names it, and a followed by U+0001 with the SVG generator's XML-text refusal

  @VS-28 @build @rejects
  Scenario: The Prism generation refuses by name an empty seed
    Given the Prism generation in app/src/Studio/PrismGeneration.lex.tex
    When prism-code check has Lean's kernel prove Studio.Prism.Seed.refuses_the_empty_seed and Studio.PrismGeneration.refuses_a_request_with_no_seed
    Then an empty seed is answered with the empty-seed refusal
    And prism-code build replays the request prism TAB audio/midi TAB, answered with that refusal

  @VS-29 @build
  Scenario: The Prism generation lists a seed's facets by media type, answers each media type with the artifact its generator writes from the seed, and each artifact begins as its media type requires: audio/midi and audio/wave by their WHATWG MIME Sniffing patterns, image/svg+xml with an svg root in the SVG namespace
    Given the Prism generation in app/src/Studio/PrismGeneration.lex.tex
    When prism-code check has Lean's kernel prove Studio.PrismGeneration.lists_the_facets, Studio.PrismGeneration.routes_each_media_type_to_its_generator and Studio.PrismGeneration.each_facet_has_its_media_type
    Then the seed verdant lists the facets audio/midi, audio/wave and image/svg+xml
    And prism-code build replays the list and the audio/midi and image/svg+xml facets of the seed verdant

  @VS-30 @build @rejects
  Scenario: The Prism generation refuses by name a media type the Prism has no facet of, and an artifact is not taken for another media type
    Given the Prism generation in app/src/Studio/PrismGeneration.lex.tex
    When prism-code check has Lean's kernel prove Studio.PrismGeneration.refuses_a_facet_it_has_not
    Then a Standard MIDI File does not begin as an SVG document
    And prism-code build replays the request prism TAB text/plain TAB verdant, answered with the refusal that names text/plain

  @VS-31 @build @rejects
  Scenario: The Prism generation refuses by name a request that is not a media type, a tab, then the seed
    Given the Prism generation in app/src/Studio/PrismGeneration.lex.tex
    When prism-code check has Lean's kernel prove Studio.PrismGeneration.refuses_a_request_with_no_seed
    Then prism-code build replays the request prism TAB verdant, answered with the request-shape refusal
