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
    Then the seed verdant lists the facets audio/midi, audio/wave, image/svg+xml and application/zip
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

  @VS-78 @build
  Scenario: The Prism bundle's digests are SHA-256 and SHA-512 of FIPS 180-4, each giving the standard's digest of its one-block example abc
    Given the digest model in app/src/Studio/Prism/Bundle/Digest.lex.tex
    When prism-code check has Lean's kernel prove Studio.Prism.Bundle.Digest.sha256_of_the_fips_examples and Studio.Prism.Bundle.Digest.sha512_of_the_fips_examples
    Then the digests of abc are the ones FIPS 180-4's examples give

  @VS-79 @build
  Scenario: The Prism bundle's archive is a ZIP file of stored entries as PKWARE APPNOTE 6.3.3 lays it out and ISO/IEC 21320-1 restricts it: each entry's local header with its CRC-32 and sizes, then the central directory, then the end of central directory record
    Given the archive model in app/src/Studio/Prism/Bundle/Archive.lex.tex
    When prism-code check has Lean's kernel prove Studio.Prism.Bundle.Archive.archives_a_stored_entry and Studio.Prism.Bundle.Archive.archives_nothing_as_an_empty_directory
    Then the archive of a.txt holding abc is the stored ZIP file of that one entry
    And the archive of no entries is the end of central directory record alone

  @VS-80 @build
  Scenario: The Prism generation answers application/zip with the Prism's bundle: a BagIt 1.0 bag whose payload is every facet under data/ by its media type, with a SHA-256 and a SHA-512 payload manifest, serialized as a ZIP archive that begins with its local file header signature
    Given the bundle model in app/src/Studio/Prism/Bundle.lex.tex
    When prism-code check has Lean's kernel prove Studio.Prism.Bundle.bags_each_facet_with_both_manifests, Studio.Prism.Bundle.manifests_each_facet_by_sha256, Studio.Prism.Bundle.manifests_each_facet_by_sha512 and Studio.PrismGeneration.the_bundle_is_a_zip_archive
    Then the bag of three facets holds bagit.txt, manifest-sha256.txt, manifest-sha512.txt and data/audio/midi, data/audio/wave and data/image/svg+xml
    And it begins as application/zip requires, and a Standard MIDI File does not

  @VS-82 @build
  Scenario: A Prism's features are its seed segmented into words by the word boundaries of Unicode text segmentation (UAX #29), one feature per line, so a word is its own feature, a sentence has a feature per word, and a feature list is a fixed point
    Given the feature model in app/src/Studio/Prism/Feature.lex.tex
    When prism-code check has Lean's kernel prove Studio.Prism.Feature.a_word_is_its_own_feature, Studio.Prism.Feature.a_sentence_has_a_feature_per_word, Studio.Prism.Feature.keeps_a_word_whole_across_its_mid_letters_and_numbers, Studio.Prism.Feature.punctuation_alone_has_no_feature, Studio.Prism.Feature.features_are_a_fixed_point and Studio.Prism.Seed.specifies_a_feature_per_word
    Then the seed verdant has the one feature verdant, and Fatelines, a game of fate. has the features Fatelines, a, game, of and fate
    And can't, 3.14 and v2_x stay whole by rules WB6, WB7, WB11, WB12 and WB13a, and punctuation alone is no feature

  @VS-83 @build
  Scenario: The Prism generation answers features with the Prism's features, generates every facet from them, and takes a tuned feature list as its own seed
    Given the Prism generation in app/src/Studio/PrismGeneration.lex.tex
    When prism-code check has Lean's kernel prove Studio.PrismGeneration.lists_the_features, Studio.PrismGeneration.generates_each_facet_from_the_features and Studio.PrismGeneration.a_tuned_feature_list_is_its_own_seed
    Then prism-code build replays the request prism TAB features TAB Fatelines, a game of fate., answered with its five features one per line

  @VS-84 @build @rejects
  Scenario: The Prism generation refuses by name a seed with no word, since such a Prism has no feature
    Given the seed model in app/src/Studio/Prism/Seed.lex.tex
    When prism-code check has Lean's kernel prove Studio.Prism.Seed.refuses_a_seed_with_no_word, Studio.Prism.Seed.does_not_admit_a_seed_with_no_word and Studio.PrismGeneration.refuses_features_of_a_seed_with_no_word
    Then prism-code build replays the request prism TAB features TAB ..., answered with the no-word refusal

  @VS-85 @build
  Scenario: A Prism's closure closes a domain over every domain it shares a standard or a role with, to a fixed point, and spreads a cell to the same phase of each domain in that closure
    Given the closure model in app/src/Studio/Prism/Closure.lex.tex, over the register that verdant_studio publish writes to app/src/Studio/Prism/Closure/Register.lex.tex from catalog/standards-index.csv, model/mappings and model/matrix
    When prism-code check has Lean's kernel prove Studio.Prism.Closure.closes_domains_that_share_a_standard, Studio.Prism.Closure.closes_domains_through_shared_roles, Studio.Prism.Closure.a_domain_that_shares_nothing_is_its_own_closure, Studio.Prism.Closure.the_closure_is_a_fixed_point, Studio.Prism.Closure.spreads_a_cell_to_the_same_phase_of_its_closure and Studio.Prism.Closure.spreads_only_to_cells_the_register_holds
    Then over the pairs A-B, J-M and M-U, domain A closes to A and B, and domain J closes to J, M and U through M
    And domain C, which shares nothing, is its own closure, and J/OPS spreads only to the OPS cells the register holds

  @VS-86 @build
  Scenario: The Prism generation answers cells with the Prism's cells, one domain/phase per line of the standards index, each followed by a tab and its value, empty until a bound source places the seed's features on cells
    Given the Prism generation in app/src/Studio/PrismGeneration.lex.tex
    When prism-code check has Lean's kernel prove Studio.PrismGeneration.lists_the_cells
    Then the cells it answers are the closure register's, which verdant_studio publish writes from the standards index, so they never drift from it

  @VS-88 @build
  Scenario: A Prism's routed cells fill each register cell with the values routed to the same phase of any domain in its closure, and routed cells are a fixed point
    Given the closure model in app/src/Studio/Prism/Closure.lex.tex, whose cells VerdantUI routes weights onto (VerdantUI DEC-009 and DEC-016)
    When prism-code check has Lean's kernel prove Studio.Prism.Closure.fills_a_routed_cell_and_spreads_it_over_its_closure and Studio.Prism.Closure.routed_cells_are_a_fixed_point
    Then over the pairs A-B, J-M and M-U, the routed line J/OPS TAB lantern fills J/OPS and M/OPS, and leaves U with no OPS cell empty
    And the filled cells routed again answer themselves

  @VS-89 @build
  Scenario: The Prism generation refuses by name a routed line that is not a cell of the Prism's register
    Given the closure model in app/src/Studio/Prism/Closure.lex.tex
    When prism-code check has Lean's kernel prove Studio.Prism.Closure.names_a_routed_line_outside_the_register and Studio.Prism.Closure.admits_routed_lines_the_register_holds
    Then prism-code build replays the request prism TAB cells TAB Q9/OPS TAB x, answered with the refusal that names that line

  @VS-90 @build
  Scenario: The Prism generation takes the values routed onto its cells as the Prism's features and generates every facet from them
    Given the Prism generation in app/src/Studio/PrismGeneration.lex.tex
    When prism-code check has Lean's kernel prove Studio.Prism.Closure.a_routed_text_carries_its_values and Studio.PrismGeneration.lists_the_routed_values_as_features
    Then the request prism TAB features TAB J/OPS TAB lantern LF A/CON TAB sea is answered with lantern and sea, one per line
