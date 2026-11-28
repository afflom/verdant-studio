Feature: gui-creator

  The creator journey of the Studio UI Prism, in app/src/Studio/Gui/Generation.lex.tex. The app answers
  studio TAB gui TAB creator TAB SEED with the journey, its views and its actions.

  @VS-104 @build
  Scenario: Studio.Gui.Generation presents the creator journey of the Studio UI Prism in Stitch Product.Experience's pattern: for an admitted seed, the prismFeatures view lists its features (Studio.PrismGeneration), the prismFacets view lists each facet by its media type, and the tune and bundle actions are enabled
    Given the creator journey in app/src/Studio/Gui/Generation.lex.tex
    When prism-code check has Lean's kernel prove Studio.Gui.Generation.presents_the_features_and_facets_of_a_seed
    Then the seed Verdant Studio shows the features Verdant and Studio, the four facets by media type, and tune and bundle enabled
    And prism-code build replays that request, answered with that view

  @VS-105 @build
  Scenario: Tuning is natural language that VerdantUI routes onto the Prism's cells (DEC-009), so Studio.Gui.Generation presents the features of the routed cell values, as Studio.PrismGeneration generates the facets from them
    Given the creator journey in app/src/Studio/Gui/Generation.lex.tex
    When prism-code check has Lean's kernel prove Studio.Gui.Generation.presents_the_tuned_features
    Then the routed values lantern and sea show as the features lantern and sea
    And prism-code build replays that request, answered with that view

  @VS-106 @build
  Scenario: Before a seed is entered, Studio.Gui.Generation presents the prismSeed view, with the tune and bundle actions disabled
    Given the creator journey in app/src/Studio/Gui/Generation.lex.tex
    When prism-code check has Lean's kernel prove Studio.Gui.Generation.asks_for_a_seed
    Then an empty seed shows prismSeed with tune and bundle disabled
    And prism-code build replays that request, answered with that view

  @VS-107 @build
  Scenario: Studio.Gui.Generation presents a refused seed in the prismRefused view with the refusal of Studio.PrismGeneration, its tune and bundle actions disabled
    Given the creator journey in app/src/Studio/Gui/Generation.lex.tex
    When prism-code check has Lean's kernel prove Studio.Gui.Generation.shows_why_a_seed_is_refused
    Then the seed ... shows prismRefused with the no-word refusal, tune and bundle disabled
    And prism-code build replays that request, answered with that view

  @VS-108 @open
  Scenario: VerdantUI renders the creator journey that Studio.Gui.Generation presents and routes the creator's natural-language tuning onto the Prism's cells (DEC-009), so a creator enters a seed, tunes it and takes the bundle in the Studio UI; until VerdantUI serves the journey, the prism route answers it
    Given the journey of VS-104
    When VerdantUI serves it
    Then a creator tunes the seed in natural language and takes the bundle there
