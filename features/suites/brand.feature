Feature: brand

  The Verdant Studio brand, in app/src/Studio/Gui/Brand.lex.tex. The app answers studio TAB brand TAB with the parts of the brand,
  and studio TAB brand TAB PART with that part, the facet of the Verdant Studio Prism its generator produces.

  @VS-215 @build
  Scenario: Verdant Studio has a cohesive brand that its UI carries (REQ-BRAND), and as a Prism of itself it generates its own brand from its seed, so Studio.Gui.Brand names the generated facets of the Verdant Studio Prism that make up the brand: its wordmark and its typeface
    Given the brand model in app/src/Studio/Gui/Brand.lex.tex
    When prism-code check has Lean's kernel prove Studio.Gui.Brand.lists_the_parts_of_the_brand
    Then an empty request is answered with the parts of the brand and the generator of each
    And prism-code build replays that request, answered with that answer

  @VS-216 @build
  Scenario: The wordmark of the Verdant Studio brand is the image/svg+xml facet that Studio.PrismGeneration generates from the seed Verdant Studio, composed and never generated again
    Given the brand model in app/src/Studio/Gui/Brand.lex.tex
    When prism-code check has Lean's kernel prove Studio.Gui.Brand.the_wordmark_is_the_svg_facet_of_the_seed
    Then the wordmark is answered with the SVG document of the seed
    And prism-code build replays that request, answered with that answer

  @VS-217 @build @rejects
  Scenario: Studio.Gui.Brand composes only facets a generator produces, so it refuses a part no generator of the Verdant Studio Prism makes, naming it
    Given the brand model in app/src/Studio/Gui/Brand.lex.tex
    When prism-code check has Lean's kernel prove Studio.Gui.Brand.refuses_a_part_the_brand_has_not
    Then a request for a palette is answered with the refusal
    And prism-code build replays that request, answered with that answer

  @VS-218 @build
  Scenario: The brand regenerates nothing: Studio.Gui.Brand answers each part with exactly the facet Studio.PrismGeneration generates for its media type from the seed Verdant Studio
    Given the brand model in app/src/Studio/Gui/Brand.lex.tex
    When prism-code check has Lean's kernel prove Studio.Gui.Brand.the_brand_regenerates_nothing
    Then each part is the facet of its media type
